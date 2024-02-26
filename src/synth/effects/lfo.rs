/**
 * 일정한 주기의 파동을 생성함
 * 모든 결과값의 범위는 -1.0 ~ 1.0 임
 */
use std::f64::consts::PI;

pub struct LFO {
    // 초당 샘플 수
    sample_rate: f64,

    // 주파수
    frequency: f64,

    // 주기(주파수와 초당 샘플 수를 통해 계산)
    period: f64,

    // 시작점부터 지난 시간(샘플 단위)
    tick: f64
}

impl LFO {
    pub fn new(sample_rate: f64) -> Self {
        let mut lfo = Self {
            sample_rate: sample_rate,
            frequency: 0.0,
            period: 0.0,
            tick: -1.0
        };
        lfo.set_frequency(440.0);
        return lfo;
    }

    pub fn set_frequency(&mut self, val: f64) {
        self.frequency = val.max(0.0);
        self.period = self.sample_rate / self.frequency;
    }

    fn next_tick(&mut self) -> f64 {
        self.tick += 1.0; // 초기값을 -1로 설정한 이유
        return self.tick;
    }

    pub fn sine(&mut self) -> f64 {
        return (2.0 * PI * self.next_tick() / self.period).sin();
    }

    pub fn sawtooth(&mut self) -> f64 {
        let tick = self.next_tick() / self.period;
        return 2.0 * (tick - tick.round());
    }

    pub fn square(&mut self) -> f64 {
        return self.sine().signum();
    }

    pub fn triangle(&mut self) -> f64 {
        return self.sine().asin();
    }
}