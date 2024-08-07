/**
 * 샘플 하나에 오디오 필터 적용
 * 스테레오는 좌/우를 각각 따로 하면 됨(밑에 있음)
 * 참고문헌: https://www.utsbox.com/?page_id=523
 */
use std::f64::consts::PI; // 원주율
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

#[allow(non_snake_case)]
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
    
    /**
     * 아무것도 하지 않도록 함
     */
    pub fn clear(&mut self) {
        self.a0 = 1.0;
        self.b0 = 1.0;
        self.a1 = 0.0;
        self.a2 = 0.0;
        self.b1 = 0.0;
        self.b2 = 0.0;
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

    /**
     * freq = cutoff 주파수
     * bw = 대역폭 범위 (옥타브 단위)
     */
    pub fn notch(&mut self, freq: f64, bw: f64) {
        let omega = 2.0 * PI * freq / self.sample_rate;
        let alpha = omega.sin() * (LN_2 / 2.0 * bw * omega / omega.sin()).sinh();

        self.a0 = 1.0 + alpha;
        self.a1 = -2.0 * omega.cos();
        self.a2 = 1.0 - alpha;
        self.b0 = 1.0;
        self.b1 = -2.0 * omega.cos();
        self.b2 = 1.0;
    }

    /**
     * freq = cutoff 주파수
     * q = 그냥 q (resonance에 관여하는 값)
     * gain_db = freq 아래 주파수에 대한 gain (dB 단위)
     */
    pub fn low_shelf(&mut self, freq: f64, q: f64, gain_db: f64) {
        let omega = 2.0 * PI * freq / self.sample_rate;
        let A = 10.0_f64.powf(gain_db / 40.0);
        let beta = A.sqrt() / q;

        self.a0 = (A + 1.0) + (A - 1.0) * omega.cos() + beta * omega.sin();
        self.a1 = -2.0 * ((A - 1.0) + (A + 1.0) * omega.cos());
        self.a2 = (A + 1.0) + (A - 1.0) * omega.cos() - beta * omega.sin();
        self.b0 =  A * ((A + 1.0) - (A - 1.0) * omega.cos() + beta * omega.sin());
        self.b1 =  2.0 * A * ((A - 1.0) - (A + 1.0) * omega.cos());
        self.b2 =  A * ((A + 1.0) - (A - 1.0) * omega.cos() - beta * omega.sin());
    }

    /**
     * freq = cutoff 주파수
     * q = 그냥 q (resonance에 관여하는 값)
     * gain_db = freq 위의 주파수에 대한 gain (dB 단위)
     */
    pub fn high_shelf(&mut self, freq: f64, q: f64, gain_db: f64) {
        let omega = 2.0 * PI * freq / self.sample_rate;
        let A = 10.0_f64.powf(gain_db / 40.0);
        let beta = A.sqrt() / q;

        self.a0 = (A + 1.0) - (A - 1.0) * omega.cos() + beta * omega.sin();
        self.a1 =  2.0 * ((A - 1.0) - (A + 1.0) * omega.cos());
        self.a2 = (A + 1.0) - (A - 1.0) * omega.cos() - beta * omega.sin();
        self.b0 =  A * ((A + 1.0) + (A - 1.0) * omega.cos() + beta * omega.sin());
        self.b1 = -2.0 * A * ((A - 1.0) + (A + 1.0) * omega.cos());
        self.b2 =  A * ((A + 1.0) + (A - 1.0) * omega.cos() - beta * omega.sin());
    }

    /**
     * freq = cutoff 주파수
     * bw = 대역폭 범위 (옥타브 단위)
     * gain_db = freq 주변 주파수에 대한 gain (dB 단위)
     */
    pub fn peaking(&mut self, freq: f64, bw: f64, gain_db: f64) {
        let omega = 2.0 * PI * freq / self.sample_rate;
        let alpha = omega.sin() * (LN_2 / 2.0 * bw * omega / omega.sin()).sinh();
        let A = 10.0_f64.powf(gain_db / 40.0);

        self.a0 =  1.0 + alpha / A;
        self.a1 = -2.0 * omega.cos();
        self.a2 =  1.0 - alpha / A;
        self.b0 =  1.0 + alpha * A;
        self.b1 = -2.0 * omega.cos();
        self.b2 =  1.0 - alpha * A;
    }

    pub fn process(&mut self, buf: &mut [f64]) {
        for src in buf.iter_mut() {
            let output = self.b0 / self.a0 * (*src)
                + self.b1 / self.a0 * self.input1
                + self.b2 / self.a0 * self.input2
                - self.a1 / self.a0 * self.output1
                - self.a2 / self.a0 * self.output2;
    
            self.input2 = self.input1;
            self.input1 = *src;
    
            self.output2 = self.output1;
            self.output1 = output;
    
            *src = output;
        }
    }
}