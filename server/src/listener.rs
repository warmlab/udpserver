use std::net::SocketAddr;
use std::net::UdpSocket;
use std::io::Error as IOError;

use log::{info, warn, error};

use message::Message;
use crate::thread::ThreadPool;
use crate::processor::Processor;

pub struct Listener {
    pool: ThreadPool,
    socket: Option<UdpSocket>,
    message: Message
}

impl Listener {
    pub fn new(addr: SocketAddr) -> Listener {
        Listener {
            // Create a poll instance
            pool: ThreadPool::new(4), 
            // Bind the UDP socket to an address
            socket: match UdpSocket::bind(addr) {
                Ok(sock) => Some(sock),
                Err(e) => {
                    error!("Cannot create udp socket for addr[{}], reason: [{}]", addr, e);
                    None
                }
            },

            // Create storage for events
            message: Message::new()
        }
    }

    pub fn start(&mut self) -> Result<(), IOError>{
        let mut buf = [0u8; 1024];
        let mut offset: usize = 0;

        //info!("UDP server listening on {}", self.ip_address);

        loop {
            // Poll for events
            info!("{} begin listening...", env!("CARGO_PKG_NAME"));

            // Process each event
            if let Some(&mut sock) = &self.socket.as_ref().as_mut() {
                // Read from the socket
                match sock.recv_from(&mut buf[offset..]) {
                    Ok((amt, src)) => {
                        let (index, len) = self.process_incoming_data(&mut buf, offset, amt);
                        offset = self.move_bytes(&mut buf, offset, amt, index, len);

                        if self.message.complete() {
                            let msg: Message = self.message.clone();
                            self.message.clear(); // clear the message
                            self.pool.execute(move || {
                                let processor: Processor = Processor::new();
                                processor.process(msg, src);
                            });
                        }
                    },
                    Err(e) => {
                        error!("Error receiving from socket: {:?}", e);
                        //return e;
                    }
                }
            }
        }
    }

    fn process_incoming_data(&mut self, buf: &mut [u8], offset: usize, amt: usize) -> (usize, usize) {
        if self.message.empty() {
            if Message::enough_bytes_for_head(amt + offset) {
                self.message.set_bytes(&buf, amt + offset)
            } else {
                (0, 0)
            }
        } else {
            let len = self.message.append_bytes(&buf, amt + offset);
            (0, len)
        }
    }

    fn move_bytes(&self, buf: &mut [u8], offset: usize, amt: usize, index: usize, len: usize) -> usize {
        /*let mut n = 0;
        for i in len + index .. amt + offset {
            buf[n] = buf[i];
            n += 1;
        }
        //offset = n; // offset = amt + offset - len - index;
        n*/
        buf.copy_within(index + len..amt + offset, 0);
        amt + offset - len - index
    }
}
