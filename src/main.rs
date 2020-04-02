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
        .unwrap_or("600".to_string())
        .parse::<u64>()
        .expect("Invalid TIMEOUT string");

    // skip first arg, which is the command name itself.
    let command = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    let child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .kill_on_drop(true)
        .spawn()
        .expect("Failed to spawn command")
        .wait_with_output();

    let mut listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();
    let accept = listener.accept();

    let delay = delay_for(Duration::from_secs(timeout));

    tokio::select! {
        _ = child => {
            println!("knock_down: stopped sub process");
        }
        _ = accept => {
            println!("knock_down: knocked down");
        }
        _ = delay => {
            println!("knock_down: timeout");
        }
    }
}
