# codexist-core

This crate implements the business logic for Codexist. It is designed to be used by the various Codexist UIs written in Rust.

## Dependencies

Note that `codexist-core` makes some assumptions about certain helper utilities being available in the environment. Currently, this support matrix is:

### macOS

Expects `/usr/bin/sandbox-exec` to be present.

### Linux

Expects the binary containing `codexist-core` to run the equivalent of `codexist sandbox linux` (legacy alias: `codexist debug landlock`) when `arg0` is `codexist-linux-sandbox`. See the `codexist-arg0` crate for details.

### All Platforms

Expects the binary containing `codexist-core` to simulate the virtual `apply_patch` CLI when `arg1` is `--codexist-run-as-apply-patch`. See the `codexist-arg0` crate for details.
