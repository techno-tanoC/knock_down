use tokio::net::TcpListener;
use tokio::process::Command;
use tokio::time::{delay_for, Duration};

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .unwrap_or("7777".to_string())
        .parse::<u16>()
        .expect("Invalid PORT string");

    let timeout = std::env::var("TIMEOUT")
        // .unwrap_or("600".to_string())
        .unwrap_or("20".to_string())
        .parse::<u64>()
        .expect("Invalid TIMEOUT string");

    let command = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    let _child = Command::new("sh")
        .arg("-c")
        // .arg("echo sleep; sleep 5; echo awake")
        .arg(command)
        .kill_on_drop(true)
        .spawn()
        .expect("Failed to spawn command");

    let mut listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();
    let accept = listener.accept();

    let delay = delay_for(Duration::from_secs(timeout));

    tokio::select! {
        _ = accept => {}
        _ = delay => {}
    }
}
