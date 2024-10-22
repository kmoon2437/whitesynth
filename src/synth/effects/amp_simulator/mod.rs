/**
 * 일렉기타 앰프
 * 참조 소스코드: https://github.com/micbuffa/WebAudio-Guitar-Amplifier-Simulator-3/blob/master/js/amp.js
 */

use std::f64::consts::FRAC_1_SQRT_2;
use crate::synth::effects::filter::Filter;

mod wave_shaper;
use wave_shaper::{ WaveShaper, WaveShaperCurveFactory };

pub struct GuitarAmpSimulator {
    input_gain: f64,

    low_shelf1: Filter,
    low_shelf2: Filter,
    preamp_1_gain: f64,
    curve_factory: WaveShaperCurveFactory,
    wave_shaper1: WaveShaper,
    high_pass1: Filter,

    low_shelf3: Filter,
    preamp_2_gain: f64,

    wave_shaper2: WaveShaper,
    output_gain: f64,

    bass_filter: Filter,
    bass_freq: f64,
    mid_filter: Filter,
    mid_freq: f64,
    treble_filter: Filter,
    treble_freq: f64,
    presence_filter: Filter,
    presence_freq: f64,

    cut1_filter: Filter,
    cut1_freq: f64,
    cut2_filter: Filter,
    cut2_freq: f64,

    master_gain: f64
}

impl GuitarAmpSimulator {
    pub fn new(sample_rate: f64) -> Self {
        let input_gain = 1.0;

        let mut low_shelf1 = Filter::new(sample_rate);
        low_shelf1.low_shelf(720.0, FRAC_1_SQRT_2, -6.0);

        let mut low_shelf2 = Filter::new(sample_rate);
        low_shelf2.low_shelf(320.0, FRAC_1_SQRT_2, -5.0);

        let preamp_1_gain = 1.0;
        
        let curve_factory = WaveShaperCurveFactory::new();

        let mut wave_shaper1 = WaveShaper::new();
        wave_shaper1.set_curve(curve_factory.asymetric());
        
        let mut high_pass1 = Filter::new(sample_rate);
        high_pass1.high_pass(6.0, FRAC_1_SQRT_2);
    
        let mut low_shelf3 = Filter::new(sample_rate);
        low_shelf3.low_shelf(720.0, FRAC_1_SQRT_2, -6.0);

        let preamp_2_gain = 1.0;

        let mut wave_shaper2 = WaveShaper::new();
        wave_shaper2.set_curve(curve_factory.standard(0.0));

        let output_gain = 1.0;

        let mut bass_filter = Filter::new(sample_rate);
        let bass_freq = 100.0;
        bass_filter.low_shelf(bass_freq, FRAC_1_SQRT_2, 0.0);

        let mut mid_filter = Filter::new(sample_rate);
        let mid_freq = 1700.0;
        mid_filter.peaking(mid_freq, 1.0, -10.0);

        let mut treble_filter = Filter::new(sample_rate);
        let treble_freq = 7500.0;
        treble_filter.high_shelf(treble_freq, FRAC_1_SQRT_2, -40.0);

        let mut presence_filter = Filter::new(sample_rate);
        let presence_freq = 3200.0;
        presence_filter.peaking(presence_freq, 1.0, 16.0);

        // 얘네 둘은 필요하면 활성화할 예정
        let mut cut1_filter = Filter::new(sample_rate);
        let cut1_freq = 10000.0;
        cut1_filter.peaking(cut1_freq, 0.1, 0.0);

        let mut cut2_filter = Filter::new(sample_rate);
        let cut2_freq = 17500.0;
        cut2_filter.peaking(cut2_freq, 1.0, -20.0);

        let master_gain = 1.0;

        return Self {
            input_gain,

            low_shelf1,
            low_shelf2,
            preamp_1_gain,
            curve_factory,
            wave_shaper1,
            high_pass1,

            low_shelf3,
            preamp_2_gain,

            wave_shaper2,
            output_gain,

            bass_filter,
            bass_freq,
            mid_filter,
            mid_freq,
            treble_filter,
            treble_freq,
            presence_filter,
            presence_freq,

            cut1_filter,
            cut1_freq,
            cut2_filter,
            cut2_freq,

            master_gain
        };
    }

    // 0.0 <= drive <= 1500.0
    pub fn set_drive(&mut self, drive: f64) {
        let drive = drive.max(0.0).min(1500.0);
        self.wave_shaper2.set_curve(self.curve_factory.standard(drive));
    }

    pub fn set_bass_gain_db(&mut self, gain_db: f64) {
        self.bass_filter.low_shelf(self.bass_freq, FRAC_1_SQRT_2, gain_db);
    }

    pub fn set_mid_gain_db(&mut self, gain_db: f64) {
        self.mid_filter.peaking(self.mid_freq, 1.0, gain_db);
    }

    pub fn set_treble_gain_db(&mut self, gain_db: f64) {
        self.treble_filter.high_shelf(self.treble_freq, FRAC_1_SQRT_2, gain_db);
    }

    pub fn set_presence_gain_db(&mut self, gain_db: f64) {
        self.presence_filter.peaking(self.presence_freq, 1.0, gain_db);
    }

    pub fn set_master_gain(&mut self, gain: f64) {
        self.master_gain = gain;
    }

    pub fn process(&mut self, buf: &mut [f64]) {
        for src in buf.iter_mut() {
            *src *= self.input_gain;
        }

        self.low_shelf1.process(buf);
        self.low_shelf2.process(buf);

        for src in buf.iter_mut() {
            *src *= self.preamp_1_gain;
        }

        self.wave_shaper1.process(buf);
        self.high_pass1.process(buf);
        
        self.low_shelf3.process(buf);
        for src in buf.iter_mut() {
            *src *= self.preamp_2_gain;
        }
    
        self.wave_shaper2.process(buf);
        for src in buf.iter_mut() {
            *src *= self.output_gain;
        }
    
        self.bass_filter.process(buf);
        self.mid_filter.process(buf);
        self.treble_filter.process(buf);
        self.presence_filter.process(buf);

        self.cut1_filter.process(buf);
        self.cut2_filter.process(buf);

        for src in buf.iter_mut() {
            *src *= self.master_gain;
        }
    }
}