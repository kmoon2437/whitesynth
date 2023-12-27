use crate::audio_drivers::AudioDriver;
use alsa::pcm::{ Access, Format, State, PCM };
use std::sync::mpsc;
use std::thread;

pub struct ALSA {
    sample_sender: mpsc::Sender<(f64, f64)>
}

impl ALSA {
    pub fn new(sample_rate: usize) -> Self {
        let (sample_sender, sample_receiver) = mpsc::channel::<(f64, f64)>();
        thread::spawn(move || {
            let pcm = PCM::new("default", alsa::Direction::Playback, false).unwrap();
            let hwp = pcm.hw_params_current().unwrap();
            hwp.set_channels(2).unwrap();
            hwp.set_rate(sample_rate.try_into().unwrap(), alsa::ValueOr::Nearest)
                .unwrap();
            hwp.set_format(Format::float64()).unwrap();
            hwp.set_access(Access::RWInterleaved).unwrap();
            pcm.hw_params(&hwp).unwrap();
            let swp = pcm.sw_params_current().unwrap();
            swp.set_start_threshold(hwp.get_buffer_size().unwrap())
                .unwrap();
            pcm.sw_params(&swp).unwrap();
            let io = pcm.io_f64().unwrap();
            if pcm.state() != State::Running {
                pcm.start().unwrap();
            }
            loop {
                match sample_receiver.recv() {
                    Ok(smpls) => match io.writei(&[smpls.0, smpls.1]) {
                        Ok(_) => {}
                        Err(_) => {}
                    },
                    Err(_e) => {
                        pcm.drain().unwrap();
                        break;
                    }
                }
            }
        });
        return Self {
            sample_sender: sample_sender
        };
    }
}

impl AudioDriver for ALSA {
    fn get_is_realtime(&self) -> bool {
        return true;
    }

    fn send_sample(&self, left: f64, right: f64) -> Result<(), Box<dyn std::error::Error>> {
        self.sample_sender.send((left, right))?;
        return Ok(());
    }

    fn terminate(self) {
        drop(self.sample_sender);
    }
}