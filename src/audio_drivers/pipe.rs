use std::io::{ self, Write };
use crate::audio_drivers::AudioDriver;

pub enum SampleEndian {
    Little, Big
}

pub struct Pipe {
    endian: SampleEndian
}

impl Pipe {
    pub fn new(endian: SampleEndian) -> Self {
        return Self { endian: endian };
    }
}

impl AudioDriver for Pipe {
    fn get_is_realtime(&self) -> bool {
        return true;
    }

    fn send_sample(&self, left: f64, right: f64) -> Result<(), Box<dyn std::error::Error>> {
        let left_o;
        let right_o;
        match self.endian {
            SampleEndian::Little => {
                left_o = (left as f32).to_le_bytes();
                right_o = (right as f32).to_le_bytes();
            }
            SampleEndian::Big => {
                left_o = (left as f32).to_be_bytes();
                right_o = (right as f32).to_be_bytes();
            }
        }
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write(&left_o)?;
        handle.write(&right_o)?;
        handle.flush()?;
        return Ok(());
    }

    fn terminate(self) {}
}