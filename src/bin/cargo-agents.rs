//! `cargo agents` entry point. See `symposium::entry`.

#[tokio::main]
async fn main() -> std::process::ExitCode {
    symposium::entry::main().await
}
