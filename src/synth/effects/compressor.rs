/**
 * 컴프레서
 * 참고문헌: https://www.utsbox.com/?p=1939
 */
use crate::synth::effects::envelope::Envelope;

pub struct Compressor{
    threshold:f64,
    ratio:f64,
    makeup_gain:f64,

    env:Envelope
}

impl Compressor{
    pub fn new(sample_rate:f64) -> Self{
        let env = Envelope::new_with_params(sample_rate,10.0,0.0,0.0,1.0,10.0);
        return Self{
            threshold:0.005,
            ratio:4.0,
            makeup_gain:5.0,
            env:env
        };
    }

    pub fn set_threshold(&mut self,val:f64){
        self.threshold = val.max(0.0);
    }

    pub fn set_ratio(&mut self,val:f64){
        self.ratio = val.max(0.0);
    }

    pub fn set_makeup_gain(&mut self,val:f64){
        self.makeup_gain = val.max(0.0);
    }
    
    pub fn process(&mut self,input:f64) -> f64{
        let tmp = input.abs();
        if tmp > self.threshold {
            self.env.attack();
        }else{
            self.env.release();
        }
        let output_abs = self.threshold + (tmp - self.threshold) / (1.0 + self.env.process(self.ratio - 1.0));
        return input/tmp*output_abs*self.makeup_gain;
    }
}