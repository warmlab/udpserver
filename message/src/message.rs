use crate::types::MessageType;

pub const MESSAGE_UNKNOWN_FLAG: u8 = 0x01u8;  // message begin flag
pub const MESSAGE_BEGIN: u8 = 0x3Au8;  // message begin flag, it is ':'
pub const MESSAGE_END: [u8; 2] = [0x0Du8, 0x0Au8];  // message end flag, it is '\r\n'
// message version at present
pub const MESSAGE_VERSION: u8 = 0x01u8;
// reserve bytes for message(for future use)
pub const MESSAGE_FILL_LEN: usize =	8usize;
// Message head length
pub const MESSAGE_HEAD_LEN: usize =	MESSAGE_FILL_LEN + 8usize;


#[derive(Debug, PartialEq, Clone)]
pub struct MessageHead {
    pub begin_flag: u8,         // begin flag, for example 0x02
    pub send_terminal_type: u8, // the terminal type of sender
    pub recv_terminal_type: u8, // the terminal type of receiver
    pub version: u8,            // version, from 1
  
    pub message_type: MessageType,             // message type, this flag is used to recognize the type of message header
    pub data_type: u8,                // data type, this flag is used to recoginze the data of type
    pub reserv: [u8; MESSAGE_FILL_LEN], // in base class, just as placement.
    pub body_len: u16,  // message body length(in bytes)
}

#[derive(Debug, Clone)]
pub struct Message {
      pub head: MessageHead,
    
      pub body: Box<Vec<u8>>, // message body
      pub end_flag: [u8; 2],   // the end flag of the message
      pub meet_end: bool,      // used in serializing, true if the message was analysised correctly
}

impl Message {
    pub fn new() -> Message {
        Message {
            head: MessageHead {
                begin_flag: 0u8,
                send_terminal_type: 0u8,
                recv_terminal_type: 0u8,
                version: 0u8,
                message_type: MessageType::MSG_UNKNOWN,
                data_type: 0u8,
                reserv: [0u8; MESSAGE_FILL_LEN],
                body_len: 0u16,
            },
            body: Box::new(Vec::<u8>::new()),
            end_flag: [MESSAGE_UNKNOWN_FLAG, MESSAGE_UNKNOWN_FLAG],
            meet_end: false
        }        
    }

    /*
    pub fn new_from_bytes(bytes: &[u8], length: usize) -> Option<Message> {
        for (index, byte) in bytes.into_iter().enumerate() {
            if *byte == MESSAGE_BEGIN {
                if length - index < MESSAGE_HEAD_LEN {
                    return None;
                }
                // copy bytes to message header
                let header = unsafe {
                    transmute::<[u8; MESSAGE_HEAD_LEN], MessageHead>(&bytes[index..length])
                };

                if length >= index + MESSAGE_HEAD_LEN + header.body_len + 1 {
                    if bytes[index + MESSAGE_HEAD_LEN + header.body_len] == MESSAGE_END {
                        return Message {
                            head: header,
                            body: Box::new(bytes[index+MESSAGE_HEAD_LEN..length].to_vec()), //Vec::with_capacity(header.data_len));
                            end_flag: MESSAGE_END,
                            meet_end: true,
                        }
                    } else {
                        return Message {
                            head: header,
                            body: Box::new(bytes[index+MESSAGE_HEAD_LEN..length].to_vec()), //Vec::with_capacity(header.data_len));
                            end_flag: MESSAGE_UNKNOWN_FLAG,
                            meet_end: false,
                        }
                    }
                } else {
                    return Message {
                        head: header,
                        body: Box::new(bytes[index+MESSAGE_HEAD_LEN..length].to_vec()), //Vec::with_capacity(header.data_len));
                        end_flag: MESSAGE_UNKNOWN_FLAG,
                        meet_end: false,
                    }
                }


                //if header.version != MESSAGE_VERSION {
                //    return None;
                //}

                //return header;
            }
        }

        return None
    } */

    pub fn set_bytes(&mut self, bytes: &[u8], length: usize) -> (usize, usize) {
        for (index, &byte) in bytes.iter().enumerate() {
            if byte == MESSAGE_BEGIN {
                if length - index < MESSAGE_HEAD_LEN {
                    return (index, 0);
                }
                // copy bytes to message header
                //let header = unsafe {
                //    transmute::<[u8; MESSAGE_HEAD_LEN], MessageHead>(&bytes[index..index+MESSAGE_HEAD_LEN])
                //};
                // is there any convenient way to do these
                self.head.begin_flag = bytes[index];
                self.head.send_terminal_type = bytes[index+1];
                self.head.recv_terminal_type = bytes[index+2];
                self.head.version = bytes[index+3];
                self.head.message_type = MessageType::try_from_byte(bytes[index+4]);
                self.head.data_type = bytes[index+5];
                //std::slice::bytes::copy_memory(self.head.reserv, bytes[index+6..index+6+MESSAGE_FILL_LEN]);
                self.head.reserv.copy_from_slice(&bytes[index+6..index+6+MESSAGE_FILL_LEN]);
                self.head.body_len = u16::from_be_bytes([bytes[index+6+MESSAGE_FILL_LEN], bytes[index+6+MESSAGE_FILL_LEN+1]]);

                if length >= index + MESSAGE_HEAD_LEN + self.head.body_len as usize + 2 {
                    if bytes[index + MESSAGE_HEAD_LEN + self.head.body_len as usize] == MESSAGE_END[0] &&
                       bytes[index + MESSAGE_HEAD_LEN + self.head.body_len as usize + 1] == MESSAGE_END[1] {
                        self.body.append(&mut bytes[index+MESSAGE_HEAD_LEN..index+MESSAGE_HEAD_LEN+self.head.body_len as usize].to_vec());
                        self.end_flag = MESSAGE_END;
                        self.meet_end = true;
                        
                        return (index, MESSAGE_HEAD_LEN+self.head.body_len as usize + 2);
                    } else {
                        self.body.append(&mut bytes[index+MESSAGE_HEAD_LEN..index+MESSAGE_HEAD_LEN+self.head.body_len as usize].to_vec());
                        return (index, MESSAGE_HEAD_LEN+self.head.body_len as usize); // no end flag
                    }
                } else {
                    self.body.append(&mut bytes[index+MESSAGE_HEAD_LEN..length].to_vec());
                    return (index, length - index);
                }
            }
        }

        return (length, 0)
    }

    pub fn append_bytes(&mut self, bytes: &[u8], length: usize) -> usize {
        let body_len = self.body.len();

        if length > self.head.body_len as usize - body_len + 1 {
            if !self.meet_end {
                self.body.append(&mut bytes[..self.head.body_len as usize - body_len].to_vec());
            }
            let index = self.head.body_len as usize - body_len; 
            if bytes[index] == MESSAGE_END[0] &&
                bytes[index + 1] == MESSAGE_END[1] {
                //self.meet_end = bytes[self.head.body_len as usize - self.body.len()] == MESSAGE_END[0] &&
                //                bytes[self.head.body_len as usize - self.body.len() + 1] == MESSAGE_END[1];

                self.meet_end = true;
                index + 2
            } else {
                self.meet_end = false;
                index
            }
        } else {
            if !self.meet_end {
                self.body.append(&mut bytes.to_vec());
            }
            
            length
        }
    }

    pub fn enough_bytes_for_head(length: usize) -> bool {
        length >= MESSAGE_HEAD_LEN
    }
    
    pub fn empty(&self) -> bool {
        self.head.begin_flag == MESSAGE_BEGIN
    }

    pub fn complete(&self) -> bool {
        self.meet_end
    }
}
