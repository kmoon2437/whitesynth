use std::f64::consts::PI;

pub(super) struct WaveShaper {
    curve: Vec<f64>,
    curve_len: f64 // 계산용
}

impl WaveShaper {
    pub(super) fn new() -> Self {
        return Self {
            curve: vec![0.0, 0.0],
            curve_len: 2.0
        };
    }

    pub(super) fn set_curve(&mut self, curve: Vec<f64>) {
        self.curve = curve;
        self.curve_len = self.curve.len() as f64;
    }

    pub(super) fn process(&self, buf: &mut [f64]) {
        for src in buf.iter_mut() {
            let v = ((self.curve_len - 1.0) / 2.0) * ((*src) + 1.0);
            let f = v - v.floor();
            let k = v.floor() as usize;
            
            *src = if v < 0.0 {
                self.curve[0]
            } else if v >= self.curve_len - 1.0 {
                self.curve[self.curve.len() - 1]
            } else {
                (1.0 - f) * self.curve[k] + f * self.curve[k + 1]
            }
        }
    }
}

pub(super) struct WaveShaperCurveFactory {
    sample_rate: f64
}

impl WaveShaperCurveFactory {
    pub(super) fn new() -> Self {
        return Self {
            sample_rate: 44100.0
        };
    }

    pub(super) fn standard(&self, k: f64) -> Vec<f64> {
        let len = self.sample_rate;
        let mut curve = vec![0.0; len as usize];
        let deg = PI / 180.0;

        for i in 0..(len as usize) {
            let x = (i as f64) * 2.0 / len - 1.0;
            curve[i] = (k + 3.0) * x * 57.0 * deg / (PI + k * x.abs());
        }

        return curve;
    }

    pub(super) fn standard_lower(&self, k: f64) -> Vec<f64> {
        let len = self.sample_rate;
        let mut curve = vec![0.0; len as usize];
        let deg = PI / 180.0;

        for i in 0..(len as usize) {
            let x = (i as f64) * 2.0 / len - 1.0;
            curve[i] = (k + 3.0) * x * 20.0 * deg / (PI + k * x.abs());
        }

        return curve;
    }

    pub(super) fn asymetric(&self) -> Vec<f64> {
        let len = self.sample_rate;
        let mut curve = vec![0.0; len as usize];
        for i in 0..(len as usize) {
            let x = (i as f64) * 2.0 / len - 1.0;
            if x < -0.08905 {
                curve[i] = (-3.0 / 4.0) * (1.0 - ((1.0 - (x.abs() - 0.032857)).powi(12)) + (1.0 / 3.0) * (x.abs() - 0.032847)) + 0.01;
            } else if x >= -0.08905 && x < 0.320018 {
                curve[i] = (-6.153 * (x * x)) + 3.9375 * x;
            } else {
                curve[i] = 0.630035;
            }
        }
        return curve;
    }

    pub(super) fn not_so_distorted(&self, k: f64) -> Vec<f64> {
        let a = (k / 150.0 + 2.0).powi(3);
        let len = self.sample_rate / 2.0;
        let mut curve = vec![0.0; len as usize];
        for d in 0..(len as usize) {
            let f = 2.0 * (d as f64) / len - 1.0;
            curve[d] = (a + 1.0) * f / (1.0 + a * f.abs());
        }
        return curve;
    }
}