pub const LED_TIMEOUT_NEVER: [u8; 6] = [0xAA, 0xBB, 0x03, 0x03, 0x00, 0xEE];
pub const LED_TIMEOUT_10SEC: [u8; 6] = [0xAA, 0xBB, 0x03, 0x03, 0x0A, 0xEE];
pub const LED_TIMEOUT_30SEC: [u8; 6] = [0xAA, 0xBB, 0x03, 0x03, 0x1E, 0xEE];

pub const BEEPING_ENABLE: [u8; 6] = [0xAA, 0xBB, 0x03, 0x02, 0x01, 0xEE];
pub const BEEPING_DISABLE: [u8; 6] = [0xAA, 0xBB, 0x03, 0x02, 0x00, 0xEE];

pub const AUTO_DETECT_ENABLE: [u8; 6] = [0xAA, 0xBB, 0x03, 0x81, 0x01, 0xEE];
pub const AUTO_DETECT_DISABLE: [u8; 6] = [0xAA, 0xBB, 0x03, 0x81, 0x00, 0xEE];

pub const SWITCH_TO_PC1: [u8; 6] = [0xAA, 0xBB, 0x03, 0x01, 0x01, 0xEE];
pub const SWITCH_TO_PC2: [u8; 6] = [0xAA, 0xBB, 0x03, 0x01, 0x02, 0xEE];
pub const SWITCH_TO_PC3: [u8; 6] = [0xAA, 0xBB, 0x03, 0x01, 0x03, 0xEE];
pub const SWITCH_TO_PC4: [u8; 6] = [0xAA, 0xBB, 0x03, 0x01, 0x04, 0xEE];
pub const SWITCH_TO_PC5: [u8; 6] = [0xAA, 0xBB, 0x03, 0x01, 0x05, 0xEE];
pub const SWITCH_TO_PC6: [u8; 6] = [0xAA, 0xBB, 0x03, 0x01, 0x06, 0xEE];
pub const SWITCH_TO_PC7: [u8; 6] = [0xAA, 0xBB, 0x03, 0x01, 0x07, 0xEE];
pub const SWITCH_TO_PC8: [u8; 6] = [0xAA, 0xBB, 0x03, 0x01, 0x08, 0xEE];

pub const READ_CURRENT_PORT: [u8; 6] = [0xAA, 0xBB, 0x03, 0x10, 0x00, 0xEE];
// Returns 0xAA 0xBB 0x03 0x11 0xXX 0xEE  (0xXX indicates current port)
// 0x00->PC1, 0x01->PC2, 0x02->PC3 ... 0x07->PC8
