use crate::{CancellationToken, Error, IoError, LocalIo, ReadSystemInfo, view::SystemSnapshot};
use nddev_device_sync_adapter_io::LocalChannel;
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct SystemInfo {
    io: LocalIo,
    path: PathBuf,
}

impl SystemInfo {
    pub fn new(io: LocalIo, path: PathBuf) -> Result<Self, Error> {
        if !path.is_absolute() {
            return Err(IoError::InvalidConfiguration.into());
        }
        Ok(Self { io, path })
    }

    pub fn for_current_user(io: LocalIo) -> Result<Self, Error> {
        Self::new(io, native_socket()?)
    }
}

impl ReadSystemInfo for SystemInfo {
    async fn sample(&self, cancellation: CancellationToken) -> Result<SystemSnapshot, Error> {
        let path = self.path.clone();
        let start = Instant::now();
        let bytes = self
            .io
            .local(cancellation, move |cancellation| {
                let mut channel =
                    LocalChannel::connect(&path, Duration::from_secs(5), 4096, cancellation)?;
                channel.send_frame(br#"{"interval":5}"#)?;
                channel.read_frame()
            })
            .await?;
        let sample: SystemSnapshot =
            serde_json::from_slice(&bytes).map_err(|_| Error::InvalidSample)?;
        sample.validate().map_err(|error| match error {
            InvalidSample::Version => Error::UnsupportedVersion,
            InvalidSample::Metrics => Error::InvalidSample,
        })?;
        tracing::debug!(
            event.name = "sysinfo.sample.read",
            duration_ms = start.elapsed().as_millis() as u64,
            outcome = "ok"
        );
        Ok(sample)
    }
}

fn native_socket() -> Result<PathBuf, Error> {
    #[cfg(target_os = "linux")]
    let path = std::env::var_os("XDG_RUNTIME_DIR")
        .map(|root| PathBuf::from(root).join("rldyour-sysinfo.sock"));
    #[cfg(target_os = "macos")]
    let path = std::env::var_os("HOME").map(|root| {
        PathBuf::from(root).join("Library/Application Support/rldyour-sysinfo/rldyour-sysinfo.sock")
    });
    #[cfg(target_os = "windows")]
    let path = std::env::var_os("LOCALAPPDATA")
        .or_else(|| std::env::var_os("TEMP"))
        .map(|root| PathBuf::from(root).join("rldyour-sysinfo/rldyour-sysinfo.sock"));
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    return Err(Error::UnsupportedPlatform);
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    path.ok_or_else(|| IoError::Unavailable.into())
}

impl SystemSnapshot {
    pub(crate) fn validate(&self) -> Result<(), InvalidSample> {
        if self.v != 1 {
            return Err(InvalidSample::Version);
        }
        if [
            self.cpu.usage,
            self.memory.used,
            self.memory.swap,
            self.gpu.usage,
            self.gpu.memory,
        ]
        .into_iter()
        .flatten()
        .any(|value| !value.is_finite() || !(0.0..=100.0).contains(&value))
            || [self.cpu.temp, self.gpu.temp, self.disk.temp]
                .into_iter()
                .flatten()
                .any(|value| !value.is_finite())
        {
            return Err(InvalidSample::Metrics);
        }
        Ok(())
    }
}

pub(crate) enum InvalidSample {
    Version,
    Metrics,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        NativeIo,
        view::{Cpu, Disk, Gpu, Memory, Network},
    };

    fn snapshot() -> SystemSnapshot {
        SystemSnapshot {
            v: 1,
            cpu: Cpu {
                usage: Some(12.5),
                temp: Some(41.0),
            },
            memory: Memory {
                used: Some(60.0),
                swap: None,
            },
            gpu: Gpu {
                usage: None,
                memory: None,
                temp: None,
            },
            disk: Disk {
                read: None,
                write: None,
                temp: Some(35.0),
            },
            net: Network {
                rx: Some(1024),
                tx: None,
            },
        }
    }

    #[test]
    fn valid_sample_with_missing_hardware_readings_passes() {
        assert!(snapshot().validate().is_ok());
    }

    #[test]
    fn unsupported_protocol_version_is_rejected() {
        let mut sample = snapshot();
        sample.v = 2;
        assert!(matches!(sample.validate(), Err(InvalidSample::Version)));
    }

    #[test]
    fn out_of_range_and_non_finite_metrics_are_rejected() {
        for usage in [101.0, -1.0, f64::NAN, f64::INFINITY] {
            let mut sample = snapshot();
            sample.cpu.usage = Some(usage);
            assert!(matches!(sample.validate(), Err(InvalidSample::Metrics)));
        }
    }

    #[test]
    fn non_finite_temperature_is_rejected() {
        let mut sample = snapshot();
        sample.disk.temp = Some(f64::NAN);
        assert!(matches!(sample.validate(), Err(InvalidSample::Metrics)));
    }

    #[test]
    fn constructor_requires_an_absolute_socket_path() {
        let io = NativeIo::new(1).unwrap().local_handle();
        assert!(matches!(
            SystemInfo::new(io.clone(), PathBuf::from("relative.sock")),
            Err(Error::Io(IoError::InvalidConfiguration))
        ));
        assert!(SystemInfo::new(io, std::env::temp_dir().join("rldyour-sysinfo.sock")).is_ok());
    }
}
