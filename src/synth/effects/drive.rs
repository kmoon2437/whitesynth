/**
 * 드라이브 이펙트
 * 스테레오는 좌/우를 각각 따로 하면 됨
 * 참고문헌: https://www.utsbox.com/?p=1426
 */
use crate::synth::effects::filter::Filter;

pub struct Drive{
    // 게인(Drive) 값
    drive:f64,
    
    // 출력 볼륨
    // 0.0~1.0 사이
    volume:f64,
    
    // 필터 처리기
    hpf:Filter,

    // 초당 샘플 수
    //sample_rate:f64
}

impl Drive{
    pub fn new(sample_rate:f64) -> Self{
        let mut hpf = Filter::new(sample_rate);
        hpf.high_pass(200.0,1.0/((2.0_f64).sqrt()));
        return Self{
            drive:300.0,
            volume:0.3,
            hpf:hpf,
        }
    }

    pub fn set_drive(&mut self,val:f64){
        self.drive = val.max(0.0);
    }

    pub fn set_volume(&mut self,val:f64){
        self.volume = val.max(0.0).min(1.0);
    }

    pub fn process(&mut self,input:f64) -> f64{
        let tmp = self.hpf.process(input)*self.drive;
        return tmp.max(-1.0).min(1.0)*self.volume;
    }
}