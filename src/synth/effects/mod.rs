/**
 * 각종 이펙트
 */
pub mod compressor;
pub mod delay;
pub mod drive;
pub mod envelope;
pub mod filter;
pub mod lfo;
pub mod ring_buffer;
pub mod pitch_bend;
pub mod reverb;
pub mod vibrato;

/**
 * 각종 trait 정의
 */

pub trait MonoEffect {
    fn process(&mut self, input: f64) -> f64;
}

pub trait StereoEffect {
    fn process(&mut self, input_l: f64, input_r: f64) -> (f64, f64);
}

/**
 * mono 이펙트 2가지를 stereo로 처리할 수 있도록 함
 */
pub struct StereoMonoEffect<L: MonoEffect, R: MonoEffect> {
    left_processor: L,
    right_processor: R
}

impl<L: MonoEffect, R: MonoEffect> StereoMonoEffect<L, R> {
    pub fn new(left_processor: L, right_processor: R) -> Self {
        return Self {
            left_processor: left_processor,
            right_processor: right_processor
        };
    }
}

impl<L: MonoEffect, R: MonoEffect> StereoEffect for StereoMonoEffect<L, R> {
    fn process(&mut self, input_l: f64, input_r: f64) -> (f64, f64) {
        return (
            self.left_processor.process(input_l),
            self.right_processor.process(input_r)
        );
    }
}