/**
 * 오디오 출력용 드라이버
 * fluidsynth에 있는 것들 가져온 거라 몇몇은 지워질 수도 있음
 */
// 윈도우
//pub mod wasapi;
//pub mod waveout;

// 안드로이드
//pub mod oboe;
//pub mod opensles;

// 리눅스
pub mod alsa;
//pub mod pulse_audio;
//pub mod jack; // 윈도우에서도 사용 가능
//pub mod oss; // 유닉스에서도 사용 가능

// 기타
pub mod audio_file; // 그저 wav 파일
//pub mod core_audio; // 맥os,아이폰(ios): 지원예정 없음
//pub mod port_audio; // 리눅스,유닉스,윈도우,맥os 등등
//pub mod sdl2; // 리눅스,유닉스,윈도우,맥os,아이폰(ios) 등등

pub trait AudioDriver {
    // 실시간으로 맞춰줘야 되는지 여부
    fn get_is_realtime(&self) -> bool;

    // 오디오 데이터 전송
    fn send_sample(&self, left: f64, right: f64) -> Result<(), Box<dyn std::error::Error>>;

    // 끝! 낸다
    fn terminate(self);
}