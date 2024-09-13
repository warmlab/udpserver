//mod thread;
//mod processor;

use std::net::SocketAddr;
use std::net::UdpSocket;
use std::io::Error as IOError;


//use mio::net::UdpSocket;
//use mio::{Interest, Events, Poll, Token};
use log::{info, warn, error};

use message::{Message, MessageHandler};
use crate::thread::ThreadPool;
use crate::processor::Processor;

pub struct Listener {
    pool: ThreadPool,
    socket: Option<UdpSocket>,
    //events: Events,
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
            //events: Events::with_capacity(128),
            message: Message::new()
        }
    }

    pub fn start(&mut self) -> Result<(), IOError>{
        let mut buf = [0u8; 1024];
        let mut offset: usize = 0;
        
        let mut index: usize = 0;
        let mut len: usize = 0;

        //info!("UDP server listening on {}", self.ip_address);

        loop {
            // Poll for events
            info!("begin listening...");

            // Process each event
            if let Some(&mut sock) = &self.socket.as_ref().as_mut() {
                // Read from the socket
                match sock.recv_from(&mut buf[offset..]) {
                    Ok((amt, src)) => {
                        let (index, len) = self.process_incoming_data(&mut buf, offset, amt);
                        offset = self.move_bytes(&mut buf, offset, amt, index, len);

                        if self.message.complete() {
                            let msg = self.message.clone();
                            self.pool.execute(move || {
                                let mut processor = Processor::new();
                                processor.handle_message(msg, src);
                            });
                        }
                        /*
                        if self.message.empty() {
                            if Message::enough_bytes_for_head(amt + offset) { // compare to the length of message head
                                (index, len) = self.message.set_bytes(&buf, amt + offset);
                                offset = self.move_bytes(&mut buf, offset, amt, index, len);
                            } else {
                                offset += amt;
                            }
                        } else {
                            len = self.message.append_bytes(&buf, amt + offset);
                            offset = self.move_bytes(&mut buf, offset, amt, 0, len);
                        }

                        if self.message.complete() {
                            let msg = self.message.clone();
                            // copy the message and process the message in another thread
                            self.pool.execute(move || {
                                // handle the message
                                Processor::handle_message(msg, src);
                            });
                        } */
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
