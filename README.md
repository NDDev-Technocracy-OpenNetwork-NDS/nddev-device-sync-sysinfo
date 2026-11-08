# NDDev OpenNetwork · sysinfo

AGPL-3.0-only NDS adapter. [NDDev OpenNetwork](https://nddev.ai).

Read CPU, memory, GPU, disk and network samples from the native daemon. Missing readings remain null.

The native owner is the independent [NDDev Systeminfo](https://github.com/NDDev-Technocracy-OpenNetwork/nddev-systeminfo)
product. `provider.lock.json` pins its qualified release archive, exact size,
SHA-256 and executable path for acceptance. The adapter uses its stable local v1
contract and does not embed another collector or require an NDS account in the
standalone product.

`contracts/module.json` is the canonical compiled-in runtime descriptor;
`module.yaml` identifies its repository package. Caller-owned local/process
capability handles share a bounded agent admission budget. Native responses,
private paths and content are never Debug or telemetry payloads.

Run `just check`, then `just provider-check` with the explicit `NDS_TEST_*`
provider paths used by CI. Acceptance exercises the real pinned native owner
in isolated temporary state. Linux provider acceptance and portable source
checks are distinct; an absent provider is unavailable, not a successful stub.
There is no backup/recovery implementation or private estate configuration.

Release journals live in [GitHub Releases](https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-sysinfo/releases).
