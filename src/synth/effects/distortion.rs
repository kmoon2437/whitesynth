/**
 * 디스토션
 * 참고문헌: https://www.utsbox.com/?p=1426
 */

use crate::synth::effects::filter::Filter;

pub struct Distortion {
    // 게인(Drive) 값
    drive: f64,

    // 출력 볼륨
    // 0.0~1.0 사이
    volume: f64,

    // 필터 처리기(필요시 활성화)
    pre_filter: Filter
}

impl Distortion {
    pub fn new(sample_rate: f64) -> Self {
        let mut this = Self {
            drive: 50.0,
            volume: 0.5,
            pre_filter: Filter::new(sample_rate)
        };
        this.set_enable_pre_filter(true);
        return this;
    }

    pub fn set_enable_pre_filter(&mut self, enable: bool) {
        if enable {
            self.pre_filter.high_pass(100.0, 1.0 / ((2.0_f64).sqrt()));
        } else {
            self.pre_filter.clear();
        }
    }

    pub fn set_drive(&mut self, val: f64) {
        self.drive = val.max(0.0);
    }

    pub fn set_volume(&mut self, val: f64) {
        self.volume = val.max(0.0).min(1.0);
    }

    pub fn process(&mut self, buf: &mut [f64]) {
        self.pre_filter.process(buf);
        for src in buf.iter_mut() {
            *src = ((*src) * self.drive).max(-1.0).min(1.0) * self.volume;
        }
    }
}