use crate::types::MessageType;

const MESSAGE_UNKNOWN_FLAG: u8 = 0x01u8;  // message begin flag
const MESSAGE_BEGIN: u8 = 0x02u8;  // message begin flag
const MESSAGE_END: u8 = 0x03u8;  // message begin flag
// message version at present
const MESSAGE_VERSION: u8 =		0x01u8;
// reserve bytes for message(for future use)
const MESSAGE_FILL_LEN: usize =	8usize;
// Message head length
const MESSAGE_HEAD_LEN: usize =	MESSAGE_FILL_LEN + 8usize;


#[derive(Clone)]
pub struct MessageHead {
    begin_flag: u8,         // begin flag, for example 0x02
    send_terminal_type: u8, // the terminal type of sender
    recv_terminal_type: u8, // the terminal type of receiver
    version: u8,            // version, from 1
  
    pub message_type: MessageType,             // message type, this flag is used to recognize the type of message header
    data_type: u8,                // data type, this flag is used to recoginze the data of type
    reserv: [u8; MESSAGE_FILL_LEN], // in base class, just as placement.
    body_len: u16,  // message body length(in bytes)
}

#[derive(Clone)]
pub struct Message {
      pub head: MessageHead,
    
      body: Box<Vec<u8>>, // message body
      end_flag: u8,   // the end flag of the message
      meet_end: bool,      // used in serializing, true if the message was analysised correctly
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
            end_flag: MESSAGE_UNKNOWN_FLAG,
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
                self.head.body_len = u16::from_be_bytes([bytes[index+6+MESSAGE_FILL_LEN], bytes[index+6+MESSAGE_FILL_LEN+1]]);

                if length >= index + MESSAGE_HEAD_LEN + self.head.body_len as usize + 1 {
                    if bytes[index + MESSAGE_HEAD_LEN + self.head.body_len as usize] == MESSAGE_END {
                        self.body.append(&mut bytes[index+MESSAGE_HEAD_LEN..index+MESSAGE_HEAD_LEN+self.head.body_len as usize].to_vec());
                        self.end_flag = MESSAGE_END;
                        self.meet_end = true;
                        
                        return (index, index+MESSAGE_HEAD_LEN+self.head.body_len as usize +1);
                    } else {
                        self.body.append(&mut bytes[index+MESSAGE_HEAD_LEN..index+MESSAGE_HEAD_LEN+self.head.body_len as usize].to_vec());
                        return (index, index+MESSAGE_HEAD_LEN+self.head.body_len as usize); // no end flag
                    }
                } else {
                    self.body.append(&mut bytes[index+MESSAGE_HEAD_LEN..length].to_vec());
                    return (index, length);
                }
            }
        }

        return (0, 0)
    }

    pub fn append_bytes(&mut self, bytes: &[u8], length: usize) -> usize {
        let body_len = self.body.len();

        if length > self.head.body_len as usize - body_len {
            if !self.meet_end {
                self.body.append(&mut bytes[..self.head.body_len as usize - body_len].to_vec());
            }
            self.meet_end = bytes[self.head.body_len as usize - self.body.len()] == MESSAGE_END;

            self.head.body_len as usize - body_len + 1
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
