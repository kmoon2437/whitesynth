/**
 * 상수 모음
 */

pub mod manufacturers {
    // 편의상 제일 앞자리가 0x00이 아닌 경우에도 3바이트 배열을 사용
    pub const STD_DEV: [u8; 3] = [0x7d, 0x00, 0x00];
    pub const STD_NON_REALTIME: [u8; 3] = [0x7e, 0x00, 0x00];
    pub const STD_REALTIME: [u8; 3] = [0x7f, 0x00, 0x00];
    pub const ROLAND: [u8; 3] = [0x41, 0x00, 0x00];
    pub const WHITESYNTH: [u8; 3] = STD_DEV;
    //pub const WHITESYNTH: [u8; 3] = [0x00, 0x34, 0x0f]; // 이랬으면 좋겠다
}