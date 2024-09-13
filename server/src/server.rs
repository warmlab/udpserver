//#[path = "./listener.rs"]
//mod listener;

use crate::listener::Listener;

use std::net::{SocketAddr, Ipv4Addr, Ipv6Addr};
use std::io::Error as IOError;

use std::str;
//use std::io;

//use format_bytes::format_bytes;

use log::{info, warn, error};


pub struct Server {
    name: String,
    ip_address: SocketAddr,
    port: u16
}

impl Server {
    pub fn new(name: &str, ip_address: &str, port: u16) -> Server {
        let ip = ip_address
            .parse()
            .unwrap_or_else(|_| {
                warn!("Invalid IP address provided, defaulting to 127.0.0.1:7070");
                SocketAddr::from(([127, 0, 0, 1], 7070))
            });

        info!("IP address set to: {:?}", ip);
/*
        if let Ok(ip) = ip_address.parse() {
            info!("ip address: {:?}", ip);
            Server {
                name: String::from(name),
                ip_address: ip,
                port: port
            }
        } else {
            warn!("default ip address");
            Server {
                name: String::from(name),
                ip_address: SocketAddr::from(([127, 0, 0, 1], 7070)),
                port: port
            }
        } */
       Server {
            name: String::from(name),
            ip_address: ip,
            port,
        }
    }

    pub fn start(&self) -> Result<(), IOError> {
        let mut listener: Listener = Listener::new(self.ip_address);
        //listener.init_socket();
        listener.start()
    }
}
