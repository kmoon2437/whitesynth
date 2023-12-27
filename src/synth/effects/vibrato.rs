/**
 * 비브라토
 */
use crate::synth::effects::lfo::LFO;
use crate::synth::effects::pitch_bend::PitchBend;

pub struct Vibrato {
    pitch_bend: PitchBend,
    lfo: LFO
}

impl Vibrato {
    pub fn new(sample_rate: f64) -> Self {
        let mut pitch_bend = PitchBend::new(sample_rate);
        pitch_bend.set_pitch_range(5.0);
        return Self {
            pitch_bend: pitch_bend,
            lfo: LFO::new(sample_rate)
        };
    }
    pub fn set_pitch_range(&mut self, val: f64) {
        self.pitch_bend.set_pitch_range(val.max(0.0));
    }

    pub fn process(&mut self, input: f64) -> f64 {
        return input;
    }
}