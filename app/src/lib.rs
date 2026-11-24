//! A program for measuring the release assets of the pipeline.

/// What the program prints for `--version`: its name and its version.
#[must_use]
pub fn version() -> String {
    format!("app {}", env!("CARGO_PKG_VERSION"))
}
