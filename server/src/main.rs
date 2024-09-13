/*
extern crate syslog;
#[macro_use]
extern crate log;
*/
mod server;
mod listener;
mod processor;
mod thread;
//mod buffer;

/*
use std::fs::File;
use std::io;

use daemonize::Daemonize;
*/

use std::fs::File;
use std::io::{self, Write};
use std::os::fd::IntoRawFd;
use std::process::exit;
use std::time::Duration;
//use std::thread;
use syslog::{Facility, Formatter3164, BasicLogger};
use log::{SetLoggerError, LevelFilter, info};

use server::Server;

fn main() -> io::Result<()> {
    /*
    // Set up daemonization
    let stdout = File::create("/tmp/udp_server.out").unwrap();
    let stderr = File::create("/tmp/udp_server.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/udp_server.pid") // Specify the PID file
        .chown_pid_file(true) // Change the owner of the PID file to the user
        .working_directory("/tmp") // Change working directory
        .stdout(stdout) // Redirect stdout to a file
        .stderr(stderr); // Redirect stderr to a file

    match daemonize.start() {
        Ok(_) => println!("Daemonized with success"),
        Err(e) => eprintln!("Error, {}", e),
    }

    let server: Server = Server::new("test server", "0.0.0.0", 7070);
    
    server.start()
    */
    // Step 1: Fork the process to create a new child process
    match unsafe { libc::fork() } {
        0 => {
            // Child process continues execution
        }
        -1 => {
            eprintln!("Fork failed");
            exit(1);
        }
        _ => {
            // Parent process exits
            exit(0);
        }
    }

    // Step 2: Create a new session and process group
    unsafe {
        libc::setsid();
    }

    // Step 3: Fork again to prevent the daemon from acquiring a controlling terminal
    match unsafe { libc::fork() } {
        0 => {
            // Second child process continues execution
        }
        -1 => {
            eprintln!("Fork failed");
            exit(1);
        }
        _ => {
            // First child process exits
            exit(0);
        }
    }

    // Step 4: Change the current working directory to /tmp
    std::env::set_current_dir("/tmp").unwrap();

    // Step 5: Close file descriptors
    unsafe {
        libc::close(0);
        libc::close(1);
        libc::close(2);
    }

    // Optional: Redirect stdin, stdout, stderr to /dev/null
    let devnull = File::open("/dev/null").unwrap();
    let _ = devnull.into_raw_fd();

    // Step 6: Log initialization
    let formatter = Formatter3164 {
        facility: Facility::LOG_DAEMON,
        hostname: None,
        process: "udpserver".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("Could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .expect("Could not set logger");

    info!("Daemon started");
    
    let server: Server = Server::new("test server", "0.0.0.0:7070", 7070);
    
    server.start()

    /*
    // Daemon loop
    loop {
        // Your daemon logic goes here
        log::info!("Daemon is running...");
        thread::sleep(Duration::from_secs(60));
    }*/
}
