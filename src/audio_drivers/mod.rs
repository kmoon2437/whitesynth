// 윈도우
pub mod directSound; // 어차피 wasapi 쓰는거랑 별 차이 없지 않을까
pub mod wasapi;
pub mod waveout;

// 안드로이드
pub mod oboe;
pub mod opensles;

// 리눅스
pub mod alsa;
pub mod pulse_audio;
pub mod jack; // 윈도우에서도 사용 가능
pub mod oss; // 유닉스에서도 사용 가능

// 기타
pub mod file; // 그저 wav 파일
pub mod core_audio; // 맥os,아이폰(ios): 지원예정 없음
pub mod port_audio; // 리눅스,유닉스,윈도우,맥os 등등
pub mod sdl2; // 리눅스,유닉스,윈도우,맥os,아이폰(ios) 등등

pub trait AudioDriver{}