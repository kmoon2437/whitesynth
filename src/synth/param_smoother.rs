use std::f64::consts::PI; // 원주율
use std::f64::consts::E; // 자연로그의 밑

pub struct ParamSmoother {
    slope: f64,
    smoothing_time_samples: f64,
    current_val: f64,
    target_val: f64
}

impl ParamSmoother {
    pub fn new(smoothing_time_ms: f64, sample_rate: f64) -> Self {
        return Self {
            slope: 0.0,
            smoothing_time_samples: smoothing_time_ms / 1000.0 * sample_rate,
            current_val: 0.0,
            target_val: 0.0
        };
    }

    fn set_val(&mut self, val: f64) {
        self.target_val = val;
        self.slope = (val - self.current_val) / self.smoothing_time_samples;
    }

    #[inline]
    pub fn process(&mut self, val: f64) -> f64 {
        if val != self.target_val {
            self.set_val(val);
        }

        self.current_val += self.slope;
        if self.slope > 0.0 && self.current_val >= self.target_val {
            self.slope = 0.0;
            self.current_val = self.target_val;
        } else if self.slope < 0.0 && self.current_val <= self.target_val {
            self.slope = 0.0;
            self.current_val = self.target_val;
        }

        return self.current_val;
    }
}