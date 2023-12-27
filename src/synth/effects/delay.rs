/**
 * 딜레이
 * 참고문헌: https://www.utsbox.com/?p=1517
 */
use crate::util;
use crate::synth::effects::ring_buffer::RingBuffer;
use crate::synth::effects::ring_buffer::MAX_INTERVAL_SEC_F64;

pub struct Delay{
    level:f64,
    feedback:f64,
    sample_rate:f64,
    ring_buf:RingBuffer
}

impl Delay{
    pub fn new(sample_rate:f64) -> Self{
        return Self::new_with_params(sample_rate,250.0,0.25,0.75);
    }

    pub fn new_with_params(sample_rate:f64,time:f64,level:f64,feedback:f64) -> Self{
        let mut ring_buf = RingBuffer::new(sample_rate as usize);
        ring_buf.set_interval((sample_rate * (time / 1000.0)) as usize);
        return Self{
            level:level,
            feedback:feedback,
            sample_rate:sample_rate,
            ring_buf:ring_buf
        };
    }
    
    pub fn set_time(&mut self,time:f64){
        self.ring_buf.set_interval((self.sample_rate * (time.max(0.0).min(MAX_INTERVAL_SEC_F64) / 1000.0)) as usize);
    }
    
    pub fn set_level(&mut self,val:f64){
        self.level = val.max(0.0).min(1.0);
    }
    
    pub fn set_feedback(&mut self,val:f64){
        self.feedback = val.max(0.0).min(1.0);
    }

    pub fn process(&mut self,input:f64) -> f64{
        let output = util::synth::mix_two_samples(input,self.level * self.ring_buf.read(0));
        self.ring_buf.write(util::synth::mix_two_samples(input,self.feedback * self.ring_buf.read(0)));
        self.ring_buf.next();
        return output;
    }
}