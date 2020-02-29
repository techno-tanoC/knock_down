use std::process::Command;
use std::net::TcpListener;

fn main() {
    let port = std::env::var("PORT")
        .expect("PORT is not found in environment variables")
        .parse::<u16>()
        .expect("failed to parse PORT into u16");

    let args = std::env::args().collect::<Vec<_>>().join(" ");

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(args)
        .spawn()
        .expect("failed to execute child process");

    let listener = TcpListener::bind(("0.0.0.0", port)).unwrap();
    listener.accept().expect("failed to accept");
    //
    // match child.try_wait() {
    //     Ok(Some(status)) => {
    //         println!("child exited with status {}", status);
    //     },
    //     Ok(None) => {
    //         child.kill().expect("failed to kill");
    //         println!("killed child");
    //     },
    //     Err(e) => {
    //         println!("error on wait child: {}", e);
    //     }
    // }
}
