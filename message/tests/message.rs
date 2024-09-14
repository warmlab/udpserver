#[cfg(test)]
mod message_tests {
    use message::{self, Message, MessageHead};
    use message::MessageType;

    #[test]
    fn test_new() {
        let m: Message = Message::new();
        //assert_eq!(m.head.begin_flag, message::MESSAGE_BEGIN);
        assert_eq!(m.head, MessageHead {
            begin_flag: 0u8,
            send_terminal_type: 0u8,
            recv_terminal_type: 0u8,
            version: 0u8,
            message_type: MessageType::MSG_UNKNOWN,
            data_type: 0u8,
            reserv: [0u8; message::MESSAGE_FILL_LEN],
            body_len: 0u16,
        });
    }

    #[test]
    fn test_set_bytes_not_enough_for_head() {
        let mut m: Message = Message::new();
        let buf: [u8; 16] = [0x4A, 0x3A, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let (index, length) = m.set_bytes(&buf, buf.len());
        assert_eq!(index, 1);
        assert_eq!(length, 0);
        assert_eq!(m.head, MessageHead {
            begin_flag: 0u8,
            send_terminal_type: 0u8,
            recv_terminal_type: 0u8,
            version: 0u8,
            message_type: MessageType::MSG_UNKNOWN,
            data_type: 0u8,
            reserv: [0u8; message::MESSAGE_FILL_LEN],
            body_len: 0u16,
        });
        assert!(!m.meet_end);
    }

    #[test]
    fn test_set_bytes_empty_data() {
        let mut m: Message = Message::new();
        let buf: [u8; 20] = [0x3A, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0D, 0x0A, 0x4A, 0x5A];
        let (index, length) = m.set_bytes(&buf, buf.len());
        assert_eq!(index, 0);
        assert_eq!(length, 18);
        assert_eq!(m.head, MessageHead {
            begin_flag: 0x3Au8,
            send_terminal_type: 1,
            recv_terminal_type: 2,
            version: 1,
            message_type: MessageType::MSG_CLIENT_CONNECT,
            data_type: 1,
            reserv: [0, 0, 0, 0, 0, 0, 0, 0],
            body_len: 0
        });
        assert!(m.meet_end);
    }

    #[test]
    fn test_set_bytes_data() {
        let mut m: Message = Message::new();
        let buf: [u8; 34] = [0x3A, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x10,
                             0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 
                             0x0D, 0x0A];
        let (index, length) = m.set_bytes(&buf, buf.len());
        assert_eq!(index, 0);
        assert_eq!(length, 34);
        assert_eq!(m.head, MessageHead {
            begin_flag: 0x3Au8,
            send_terminal_type: 1,
            recv_terminal_type: 2,
            version: 1,
            message_type: MessageType::MSG_CLIENT_CONNECT,
            data_type: 1,
            reserv: [0, 0, 0, 0, 0, 0, 0, 1],
            body_len: 16 
        });
        assert!(m.meet_end);
    }

    #[test]
    fn test_set_bytes_data_not_starts_with_begin_flag() {
        let mut m: Message = Message::new();
        let buf: [u8; 34] = [0x2A, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x10,
                             0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 
                             0x0D, 0x0A];
        let (index, length) = m.set_bytes(&buf, buf.len());
        assert_eq!(index, 34);
        assert_eq!(length, 0);
        assert_eq!(m.head, MessageHead {
            begin_flag: 0u8,
            send_terminal_type: 0u8,
            recv_terminal_type: 0u8,
            version: 0u8,
            message_type: MessageType::MSG_UNKNOWN,
            data_type: 0u8,
            reserv: [0u8; message::MESSAGE_FILL_LEN],
            body_len: 0u16,
        });
        assert!(!m.meet_end);
    }

    #[test]
    fn test_set_bytes_data_not_starts_with_begin_flag2() {
        let mut m: Message = Message::new();
        let buf: [u8; 40] = [0x01, 0x02, 0x03, 0x04,
                             0x3A, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x10,
                             0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 
                             0x0D, 0x0A, 0x0B, 0x0C];
        let (index, length) =m.set_bytes(&buf, buf.len());
        assert_eq!(index, 4);
        assert_eq!(length, 34);
        assert_eq!(m.head, MessageHead {
            begin_flag: 0x3Au8,
            send_terminal_type: 1,
            recv_terminal_type: 2,
            version: 1,
            message_type: MessageType::MSG_CLIENT_CONNECT,
            data_type: 1,
            reserv: [0, 0, 0, 0, 0, 0, 0, 1],
            body_len: 16 
        });
        assert!(m.meet_end);
    }

    #[test]
    fn test_set_bytes_data_not_starts_with_begin_flag3() {
        let mut m: Message = Message::new();
        let buf: [u8; 41] = [0x01, 0x02, 0x03, 0x04, 0x05,
                             0x3A, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x10,
                             0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
                             0x0D, 0x0A, 0x0B, 0x0C, 0x0D];
        let (index, length) = m.set_bytes(&buf, buf.len());
        assert_eq!(index, 5);
        assert_eq!(length, 32);
        assert_eq!(m.head, MessageHead {
            begin_flag: 0x3Au8,
            send_terminal_type: 1,
            recv_terminal_type: 2,
            version: 1,
            message_type: MessageType::MSG_CLIENT_CONNECT,
            data_type: 1,
            reserv: [0, 0, 0, 0, 0, 0, 0, 1],
            body_len: 16 
        });
        assert!(!m.meet_end);
    }

    #[test]
    fn test_append_bytes() {
        let mut m: Message = Message::new();
        let buf: [u8; 18] = [0x4A, 0x4B, 0x3A, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10];
        let (index, length) = m.set_bytes(&buf, buf.len());
        assert_eq!(index, 2);
        assert_eq!(length, 16);
        assert_eq!(m.head, MessageHead {
            begin_flag: 0x3Au8,
            send_terminal_type: 1,
            recv_terminal_type: 2,
            version: 1,
            message_type: MessageType::MSG_CLIENT_CONNECT,
            data_type: 1,
            reserv: [0, 0, 0, 0, 0, 0, 0, 0],
            body_len: 0x10
        });
        assert!(!m.meet_end);
        let buf: [u8; 10] = [0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C];
        let length = m.append_bytes(&buf, buf.len());
        assert_eq!(length, 10);
        let body_result: Box<Vec<u8>> = Box::new(vec![0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C]);
        assert_eq!(m.body, body_result);
        assert!(!m.meet_end);
        let length = m.append_bytes(&buf, buf.len());
        assert_eq!(length, 6);
        assert!(!m.meet_end);

    }

    #[test]
    fn test_append_bytes2() {
        let mut m: Message = Message::new();
        let buf: [u8; 18] = [0x4A, 0x4B, 0x3A, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10];
        let (index, length) = m.set_bytes(&buf, buf.len());
        assert_eq!(index, 2);
        assert_eq!(length, 16);
        assert_eq!(m.head, MessageHead {
            begin_flag: 0x3Au8,
            send_terminal_type: 1,
            recv_terminal_type: 2,
            version: 1,
            message_type: MessageType::MSG_CLIENT_CONNECT,
            data_type: 1,
            reserv: [0, 0, 0, 0, 0, 0, 0, 0],
            body_len: 0x10
        });
        assert!(!m.meet_end);
        let buf: [u8; 10] = [0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x0D, 0x0A, 0x6C, 0x6C];
        let length = m.append_bytes(&buf, buf.len());
        assert_eq!(length, 10);
        let body_result: Box<Vec<u8>> = Box::new(vec![0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x6C, 0x0D, 0x0A, 0x6C, 0x6C]);
        assert_eq!(m.body, body_result);
        assert!(!m.meet_end);
        let length = m.append_bytes(&buf, buf.len());
        assert_eq!(length, 8);
        assert!(m.meet_end);

    }
}
