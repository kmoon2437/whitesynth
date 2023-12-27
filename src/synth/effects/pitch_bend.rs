/**
 * 피치휠
 * 참고문헌: https://www.utsbox.com/?p=2557
 */
use crate::util;
use crate::synth::effects::ring_buffer::RingBuffer;

pub struct PitchBend{
    // 0 이상
    pitch_range:f64,

    // -1 ~ 1
    pitch:f64,

    speed:f64,
    ring_buf:RingBuffer,
    ring_buf_pos:f64
}

impl PitchBend{
    pub fn new(sample_rate:f64) -> Self{
        let ring_buf = RingBuffer::new(sample_rate as usize);
        ring_buf.set_interval(10000);
        let processor = Self{
            pitch_range:12.0,
            pitch:0.0,
            speed:0.0,
            ring_buf:ring_buf,
            ring_buf_pos:0.0
        };
        processor.calc_speed();
        return processor;
    }
    
    fn calc_speed(&mut self){
        self.speed = 2.0.pow(self.pitch_range*self.pitch / 12.0) - 1.0;
    }

    pub fn set_pitch_range(&mut self,val:f64){
        self.pitch_range = val.max(0.0);
        self.calc_speed();
    }

    pub fn set_pitch(&mut self,val:f64){
        self.pitch = val.max(-1.0).min(1.0);
        self.calc_speed();
    }

    pub fn process(&mut self,input:f64) -> f64{
        return input;
    }
}