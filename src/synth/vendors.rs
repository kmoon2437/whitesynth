/**
 * 미디음원 제조사 ID 상수
 * 설명 문서에서는 일반적으로 많이 쓰이는 manufacturer를 사용함
 * 소스코드에서는 작성의 편의를 위해 단어가 짧은 vendor를 사용함
 */

#[derive(PartialEq, Eq)]
pub enum VendorId {
    Standard(u8),
    Extended(u8, u8)
}

pub const STD_DEV: VendorId = VendorId::Standard(0x7d);
pub const STD_NON_REALTIME: VendorId = VendorId::Standard(0x7e);
pub const STD_REALTIME: VendorId = VendorId::Standard(0x7f);
pub const ROLAND: VendorId = VendorId::Standard(0x41);
pub const YJ: VendorId = STD_DEV;
//pub const YJ: VendorId = VendorId::Extended(0x34, 0x0f); // 이랬으면 좋겠다(ID 발급받으려면 매년마다 돈 내야 됨)