/**
 * whitesynth cli
 */
extern crate whitesynth;
extern crate chrono;
extern crate log;
extern crate log4rs;

// 로그 찍는거 관련
// 뭐가 이렇게 많냐
use chrono::offset::Local;
use log4rs::{
    append::{
        console::{ ConsoleAppender, Target },
        file::FileAppender
    },
    config::{ Appender, Config, Root },
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter
};

// 테스트용
use std::fs::File;
use std::io::{ Read, Write };
use whitesynth::audio_drivers::AudioDriver;
use whitesynth::audio_drivers::pipe::{ Pipe, SampleEndian };
use whitesynth::synth::effects::lfo::LFO;

fn init_logger(verbose: bool) -> Result<log4rs::Handle, Box<dyn std::error::Error>> {
    let file_log_pattern = "{d(%Y-%m-%d %H:%M:%S %Z)} [{l}] {m}{n}";
    let console_log_pattern = "{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}{n}";
    let date = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let console = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new(console_log_pattern)))
        .build();

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(file_log_pattern)))
        .build(format!("log/{}.log", date))?;

    let latest_log_file = FileAppender::builder()
        .append(false)
        .encoder(Box::new(PatternEncoder::new(file_log_pattern)))
        .build("log/latest.log")?;

    let console_log_filter = ThresholdFilter::new(if verbose {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Info
    });
    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file)))
        .appender(Appender::builder().build("latestLogFile", Box::new(latest_log_file)))
        .appender(
            Appender::builder()
                .filter(Box::new(console_log_filter))
                .build("console", Box::new(console))
        )
        .build(
            Root::builder()
                .appender("file")
                .appender("latestLogFile")
                .appender("console")
                .build(log::LevelFilter::Trace)
        )?;

    return Ok(log4rs::init_config(config)?);
}

fn slice2array(slice: &[u8]) -> [u8;2] {
    return [slice[0], slice[1]];
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _handle = init_logger(false)?;

    let mut lfo = LFO::new(48000.0);
    lfo.set_frequency(220.0*(2.0_f64.powf(3.0/12.0)));
    let driver = Pipe::new(SampleEndian::Little);
    for _ in 0..(2 * 48000) {
        let smpl = lfo.square() / 2.0;
        driver.send_sample(smpl, smpl)?;
    }
    std::thread::sleep(std::time::Duration::from_secs(3));

/*
    let args: Vec<String> = std::env::args().collect();
    log::info!("args: {} {}", &args[1], &args[2]);
    let mut file = File::open(&args[1])?;
    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf)?;
    let len = buf.len()/(2*2); // 채널 2개, signed 16비트 샘플, 초당 샘플 수 48000
    log::info!("len: {}", len);
    
    let mut new_file = File::create(&args[2])?;
    for ii in 0..len {
        let i = ii*4;
        let left = i16::from_le_bytes(slice2array(&buf[i..=(i+1)]));
        let right = i16::from_le_bytes(slice2array(&buf[(i+2)..=(i+3)]));
        let left_f = (left as f64) / (if left < 0 { 32768.0 } else { 32767.0 });
        let right_f = (right as f64) / (if right < 0 { 32768.0 } else { 32767.0 });
        let o_left_f = -left_f;
        let o_right_f = -right_f;
        let o_left = (o_left_f / 1.0 * (if o_left_f < 0.0 { 32768.0 }else{ 32767.0 })) as i16;
        let o_right = (o_right_f / 1.0 * (if o_right_f < 0.0 { 32768.0 }else{ 32767.0 })) as i16;
        if ii % 480000 == 0 {
            //println!("{} {} {}",i,o_left,o_right);
            log::debug!("{} {} {}",i,o_left,o_right);
        }
        new_file.write_all(&(o_left.to_le_bytes()))?;
        new_file.write_all(&(o_right.to_le_bytes()))?;
    }
    log::info!("completed");
*/
    
    return Ok(());
}