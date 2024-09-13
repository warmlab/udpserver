// Status Code
//#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(PartialEq, Clone, Copy)]
pub enum StatusType {
    STATUS_OK = 0x00,                // OK
    STATUS_VIEW = 0x01,              // View status - used to identify the cabin or hall side
    STATUS_WAIT = 0x02,              // Waiting status
    STATUS_CONNECTED = 0x03,         // Connected
    STATUS_BINDED = 0x04,            // Binded status
    STATUS_ONLINE = 0x05,            // Online status

    STATUS_OTHER_CARBIN_CONNECTED = 0x10,  // Another cabin side connected
    STATUS_OTHER_REMOTE_CONNECTED = 0x11,  // Another hall side connected

    STATUS_BIND_MULTIROTOR = 0x20,         // Bind multirotor ground station
    STATUS_UNBIND_MULTIROTOR = 0x21,       // Unbind multirotor ground station
    STATUS_BIND_HELICOPTER = 0x22,         // Bind helicopter ground station
    STATUS_UNBIND_HELICOPTER = 0x23,       // Unbind helicopter ground station
    STATUS_CONNECT_DTE = 0x24,             // Connect DTE ground station
    STATUS_DISCONNECT_DTE = 0x25,          // Disconnect DTE ground station
    STATUS_FLY_START = 0x26,               // Flight test start status
    STATUS_FLY_STOP = 0x27,                // Flight test stop status
    STATUS_FLOW_TEST_PROCESS = 0x28,       // GNSS test in progress
    STATUS_FLOW_TEST_START = 0x29,         // GNSS test start status
    STATUS_FLOW_TEST_STOP = 0x2A,          // GNSS test stop status

    STATUS_STATION_ONLINE = 0xD1,          // Ground station online
    STATUS_STATION_BINDED = 0xD2,          // Ground station binded

    STATUS_PRIVILEGE_REQUEST = 0xE0,       // Hall side requests control privilege from cabin
    STATUS_PRIVILEGE_REFUSE = 0xE1,        // Cabin side refuses to switch privilege
    STATUS_PRIVILEGE_GRANT = 0xE2,         // Grant control privilege
    STATUS_PRIVILEGE_REVOKE = 0xE3,        // Revoke control privilege
    STATUS_PRIVILEGE_GOT = 0xE4,           // Control privilege obtained
    STATUS_PRIVILEGE_LOST = 0xE5,          // Control privilege lost
    STATUS_PRIVILEGE_SWITCHING = 0xE6,     // Switching control privilege

    STATUS_OFFLINE = 0xF1,                 // Offline
    STATUS_NOT_BIND = 0xF2,                // Not binded
    STATUS_NOT_CONNECT = 0xF3,             // Not connected
    STATUS_EXCEPTION = 0xF4,               // Exception
    STATUS_REFUSE = 0xF5,                  // Refused

    STATUS_ERROR = 0xFF
}
