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
        console::ConsoleAppender,
        file::FileAppender
    },
    config::{ Appender, Config, Root },
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter
};

// 테스트용
//use std::fs::File;
use whitesynth::audio_drivers::AudioDriver;
use whitesynth::audio_drivers::alsa::ALSA;
use whitesynth::synth::effects::lfo::LFO;

fn init_logger(verbose: bool) -> Result<log4rs::Handle, Box<dyn std::error::Error>> {
    let file_log_pattern = "{d(%Y-%m-%d %H:%M:%S %Z)} [{l}] {m}{n}";
    let console_log_pattern = "{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}{n}";
    let date = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let stdout = ConsoleAppender::builder()
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
                .build("console", Box::new(stdout))
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _handle = init_logger(false)?;

    let mut lfo = LFO::new(48000.0);
    lfo.set_frequency(220.0);
    let alsa = ALSA::new(48000);
    for _ in 0..(2 * 48000) {
        let smpl = lfo.sine() / 2.0;
        alsa.send_sample(smpl, smpl)?;
    }
    std::thread::sleep(std::time::Duration::from_secs(3));

    return Ok(());
}