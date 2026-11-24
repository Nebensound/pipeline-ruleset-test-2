//! A program for measuring the release assets of the pipeline.

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--version") {
        println!("app {}", env!("CARGO_PKG_VERSION"));
    }
}
