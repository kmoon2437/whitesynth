/**
 * 샘플 하나에 오디오 필터 적용
 * 스테레오는 좌/우를 각각 따로 하면 됨(밑에 있음)
 * 참고문헌: https://www.utsbox.com/?page_id=523
 */
use std::f64::consts::PI; // 무빙에서 김봉석이 매번 외우는 그거
use std::f64::consts::LN_2; // 2의 자연로그 값

pub struct Filter {
    // filter 처리 관련 변수
    // 필터의 종류, frequency 등 각종 파라미터에 따라 변화
    a0: f64,
    a1: f64,
    a2: f64,
    b0: f64,
    b1: f64,
    b2: f64,

    // 이전 입/출력
    input1: f64,
    input2: f64,
    output1: f64,
    output2: f64,

    // 초당 샘플 수
    sample_rate: f64
}

impl Filter {
    pub fn new(sample_rate: f64) -> Self {
        return Self {
            a0: 1.0,
            a1: 0.0,
            a2: 0.0,
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,

            input1: 0.0,
            input2: 0.0,
            output1: 0.0,
            output2: 0.0,

            sample_rate: sample_rate
        };
    }

    pub fn process(&mut self, input: f64) -> f64 {
        let output = self.b0 / self.a0 * input
            + self.b1 / self.a0 * self.input1
            + self.b2 / self.a0 * self.input2
            - self.a1 / self.a0 * self.output1
            - self.a2 / self.a0 * self.output2;

        self.input2 = self.input1;
        self.input1 = input;

        self.output2 = self.output1;
        self.output1 = output;

        return output;
    }

    /**
     * freq = cutoff 주파수
     * q = 그냥 q (resonance에 관여하는 값)
     */
    pub fn low_pass(&mut self, freq: f64, q: f64) {
        let omega = 2.0 * PI * freq / self.sample_rate;
        let alpha = omega.sin() / (2.0 * q);

        self.a0 = 1.0 + alpha;
        self.a1 = -2.0 * omega.cos();
        self.a2 = 1.0 - alpha;

        let b = 1.0 - omega.cos();
        self.b0 = b / 2.0;
        self.b1 = b;
        self.b2 = b / 2.0;
    }

    /**
     * freq = cutoff 주파수
     * q = 그냥 q (resonance에 관여하는 값)
     */
    pub fn high_pass(&mut self, freq: f64, q: f64) {
        let omega = 2.0 * PI * freq / self.sample_rate;
        let alpha = omega.sin() / (2.0 * q);

        self.a0 = 1.0 + alpha;
        self.a1 = -2.0 * omega.cos();
        self.a2 = 1.0 - alpha;

        let b = 1.0 + omega.cos();
        self.b0 = b / 2.0;
        self.b1 = -b;
        self.b2 = b / 2.0;
    }

    /**
     * freq = cutoff 주파수
     * bw = 대역폭 범위 (옥타브 단위)
     */
    pub fn band_pass(&mut self, freq: f64, bw: f64) {
        let omega = 2.0 * PI * freq / self.sample_rate;
        let alpha = omega.sin() * (LN_2 / 2.0 * bw * omega / omega.sin()).sinh();

        self.a0 = 1.0 + alpha;
        self.a1 = -2.0 * omega.cos();
        self.a2 = 1.0 - alpha;
        self.b0 = alpha;
        self.b1 = 0.0;
        self.b2 = -alpha;
    }
}

pub struct StereoFilter {
    processor_l: Filter,
    processor_r: Filter
}

impl StereoFilter {
    pub fn new(sample_rate: f64) -> Self {
        return Self {
            processor_l: Filter::new(sample_rate),
            processor_r: Filter::new(sample_rate)
        };
    }

    pub fn process(&mut self, input_l: f64, input_r: f64) -> (f64, f64) {
        return (
            self.processor_l.process(input_l),
            self.processor_r.process(input_r)
        );
    }

    pub fn low_pass(&mut self, freq: f64, q: f64) {
        self.processor_l.low_pass(freq, q);
        self.processor_r.low_pass(freq, q);
    }

    pub fn high_pass(&mut self, freq: f64, q: f64) {
        self.processor_l.high_pass(freq, q);
        self.processor_r.high_pass(freq, q);
    }

    pub fn band_pass(&mut self, freq: f64, bw: f64) {
        self.processor_l.band_pass(freq, bw);
        self.processor_r.band_pass(freq, bw);
    }
}