Historical record, not current release status.

# Standalone provider qualification

Provider acceptance moves from historical Systeminfo 0.3.1 to the independent
OpenNetwork 0.3.3 release. Its original repository identity is 1333577156 and
qualified source is `b8c29f90d5a9ff9593136d0a28e9f41aa0a3d488` (CI 42491248188).
The release archive's exact size, SHA-256 and nested executable path live in
`provider.lock.json`. The v1 socket and module identity remain stable. Native
product qualification covers Linux, macOS ARM/Intel and Windows; this module's
actual pinned-provider acceptance is Linux, with portable adapter tests on the
other desktop platforms. Downstream NDS composition belongs to NDS #171.

Both pull-request and main qualification use the shared downloader's exact
qualified revision from agent #191, accepting the canonical product owner.
Ubuntu source checks and an actual isolated sample from the released archive
passed before review; no collector or private runtime was copied into NDS.
