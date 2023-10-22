extern crate alsa;
use alsa::pcm::{ PCM,HwParams,Format,Access,IO };

pub struct ALSADriver{
    pub pcm:PCM,
    pub io:IO<f64>
}

impl ALSADriver{
    pub fn new(sampleRate:i32) -> Self{
        let pcm = PCM::new("default",alsa::Direction::Playback,false).unwrap();
        let hwp = HwParams::any(&pcm).unwrap();
        hwp.set_channels(2).unwrap();
        hwp.set_rate(sampleRate,alsa::ValueOr::Nearest).unwrap();
        hwp.set_format(Format::float64()).unwrap();
        hwp.set_access(Access::RWInterleaved).unwrap();
        pcm.hw_params(&hwp).unwrap();
        let hwp = pcm.hw_params_current().unwrap();
        let swp = pcm.sw_params_current().unwrap();
        swp.set_start_threshold(hwp.get_buffer_size().unwrap()).unwrap();
        pcm.sw_params(&swp).unwrap();
        return Self{
            pcm:pcm,
            io:pcm.io_f64()
        };
    }
    
    pub fn send_sample(){}
}