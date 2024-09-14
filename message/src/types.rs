use std::mem::transmute;
//use std::convert::{TryFrom, TryInto};

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MessageType {
    MSG_UNKNOWN = 0x00,                // Unknown type
    MSG_CLIENT_CONNECT,                // Client connection message
    MSG_CLIENT_CONNECT_RESPOND,        // Server's response to client connection
    MSG_CLIENT_DISCONNECT,             // Client disconnection message
    MSG_CLIENT_DISCONNECT_RESPOND,     // Response to client disconnection

    MSG_FLY_START,                     // Flight start message
    MSG_FLY_START_RESPOND,             // Response to flight start message
    MSG_FLY_STOP,                      // Flight stop message
    MSG_FLY_STOP_RESPOND,              // Response to flight stop message

    MSG_TEST_START,                    // Test start message
    MSG_TEST_CONTINUE,                 // Test continue message
    MSG_TEST_START_RESPOND,            // Response to test start message

    MSG_TEST_STOP,                     // Test stop message
    MSG_TEST_STOP_RESPOND,             // Response to test stop message

    MSG_REQUEST_PAYLOAD_STATUS,        // Request to obtain payload status message
    MSG_REQUEST_PAYLOAD_STATUS_RESPOND,// Response to request for payload status

    MSG_STOP_PAYLOAD_STATUS,           // Stop obtaining payload status message
    MSG_STOP_PAYLOAD_STATUS_RESPOND,   // Response to stop payload status message

    MSG_SEND_PAYLOAD_COMMAND,          // Message to send command to client
    MSG_SEND_PAYLOAD_COMMAND_RESPOND,  // Response to sending command to client

    MSG_BIND_STATION,                  // Bind ground station message
    MSG_BIND_STATION_RESPOND,          // Response to bind ground station message
    MSG_UNBIND_STATION,                // Unbind ground station message
    MSG_UNBIND_STATION_RESPOND,        // Response to unbind ground station message

    MSG_REQUEST_PRIVILEGE,             // Request control privilege message
    MSG_REQUEST_PRIVILEGE_RESPOND,     // Response to request control privilege message

    MSG_SWITCH_PRIVILEGE,              // Switch control privilege message
    MSG_SWITCH_PRIVILEGE_RESPOND,      // Response to switch control privilege message

    MSG_HEART_BEAT,                    // Heartbeat signal message
    MSG_HEART_BEAT_RESPOND,            // Response to heartbeat signal message

    MSG_GET_NET_PARAM,                 // Get network parameters message
    MSG_GET_NET_PARAM_RESPOND,         // Response to get network parameters message
    MSG_UPDATE_INTRA_NET_PARAM,        // Update internal network parameters message
    MSG_UPDATE_INTRA_NET_PARAM_RESPOND,// Response to update internal network parameters
    MSG_UPDATE_STATION_NET_PARAM,      // Update ground station network parameters message
    MSG_UPDATE_STATION_NET_PARAM_RESPOND, // Response to update ground station network parameters

    MSG_GET_ERROR_PARAM,               // Get error parameters message
    MSG_GET_ERROR_PARAM_RESPOND,       // Response to get error parameters message
    MSG_UPDATE_ERROR_PARAM,            // Update error parameters message
    MSG_UPDATE_ERROR_PARAM_RESPOND,    // Response to update error parameters

    MSG_GET_SIMPLE_PARAM,              // Get sampling parameters message
    MSG_GET_SIMPLE_PARAM_RESPOND,      // Response to get sampling parameters message
    MSG_UPDATE_SIMPLE_PARAM,           // Update sampling parameters message
    MSG_UPDATE_SIMPLE_PARAM_RESPOND,   // Response to update sampling parameters

    MSG_SEND_FILE_REQUEST,             // Request to send file message
    MSG_SEND_FILE_REQUEST_RESPOND,     // Response to request to send file
    MSG_SEND_FILE,                     // Send file message

    MSG_REQUEST_VIDEO,                 // Request video message
    MSG_SEND_VIDEO,                    // Response to request video message

    MSG_SEND_GUIDE_DATA,               // Send guide data to DTE ground station message

    MSG_MAX_TYPE = 0xFF,
}

impl MessageType {
  pub fn try_from_byte(value: u8) -> MessageType {
    let v: MessageType = unsafe { transmute(value) };
    v
    //match TryFrom::<u8>::try_from(value) {
    ////match value.try_into() {
    //  Ok(t) => t,
    //  Err(_) => MessageType::MSG_UNKNOWN
    //}
  }

  pub fn try_into_byte(value: MessageType) -> u8 {
    value as u8
    //match TryInto::<u8>::try_into(value) {
    //  Ok(v) => v,
    //  Err(_) => 0x00u8
    //}
  }
}

//impl From<u8> for MessageType {
//  fn from(value: u8) -> Self {
//      MessageType::MSG_UNKNOWN
//  }
//}
