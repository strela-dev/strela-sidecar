use std::env;
use std::net::TcpStream;
use craftping::sync::ping;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;


fn main() -> Result<(), std::io::Error> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let mut signals = Signals::new(&[SIGTERM, SIGINT])?;
    thread::spawn(move || {
        for _ in signals.forever() {
            r.store(false, Ordering::SeqCst);
        }
    });


    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(1));
        execute_ping();
    }

    println!("Shutting down gracefully...");
    Ok(())
}


fn execute_ping() {
    let hostname = "localhost";
    let port = 25565;

    match TcpStream::connect((hostname, port)) {
        Ok(stream) => {
            ping_and_execute_command(stream, hostname, port)
        }
        Err(e) => {
            println!("Error: {}", e)
        }
    }
}

fn ping_and_execute_command(mut stream: TcpStream, hostname: &str, port: u16) {
    let pong = ping(&mut stream, hostname, port).expect("Cannot ping server");

    // Retrieve the pod name from the environment variable
    let pod_name = env::var("POD_NAME").expect("POD_NAME environment variable not set");

    // Get the number of online players
    let online_players = pong.online_players;

    // Construct the kubectl patch command
    let patch_command = format!(
        "kubectl patch minecraftserver {} --type=merge -p '{{\"status\":{{\"players\":{}}}}}' --subresource=status",
        pod_name, online_players
    );

    println!("test: {}", patch_command);

    // Execute the kubectl patch command
    let output = Command::new("sh")
        .arg("-c")
        .arg(&patch_command)
        .output()
        .expect("Failed to execute command");

    // Check if the command was executed successfully
    if output.status.success() {
        println!("Successfully patched the Minecraft server status.");
    } else {
        // Handle the error (if any)
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed with error:\n{}", stderr);
    }
}