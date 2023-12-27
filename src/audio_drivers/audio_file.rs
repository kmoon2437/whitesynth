use std::fs::File;
//use std::io::Write;
use crate::audio_drivers::AudioDriver;

pub struct AudioFile{
    file:File
}

impl AudioFile{
    pub fn new(sample_rate:usize,path:&str) -> Result<Self,std::io::Error>{
        let file = File::create(path)?;
        let _ = &sample_rate;
        return Ok(Self{
            file:file
        });
    }
}

impl AudioDriver for AudioFile{
    fn get_is_realtime(&self) -> bool{
        return false;
    }

    fn send_sample(&self,left:f64,right:f64) -> Result<(),Box<dyn std::error::Error>>{
        (left,right);
        let _ = &self.file;
        return Ok(());
    }
    
    fn terminate(self){}
}