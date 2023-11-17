#![allow(non_snake_case)]

/**
 * whitesynth cli
 */

extern crate whitesynth;
extern crate log4rs;
extern crate log;
extern crate chrono;

// 로그 찍는거 관련
// 뭐가 이렇게 많냐
use log4rs::{
    append::{
        console::ConsoleAppender,
        file::FileAppender
    },
    encode::pattern::PatternEncoder,
    config::{ Appender,Config,Root },
    filter::threshold::ThresholdFilter
};
use chrono::offset::Local;

// 테스트용
use std::fs::File;
use whitesynth::soundbank::sf2::SF2;

fn init_logger(verbose: bool) -> Result<log4rs::Handle,Box<dyn std::error::Error>>{
    let file_log_pattern = "{d(%Y-%m-%d %H:%M:%S %Z)} [{l}] {m}{n}";
    let console_log_pattern = "{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}{n}";
    let date = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let stdout = ConsoleAppender::builder()
    .encoder(Box::new(PatternEncoder::new(console_log_pattern)))
    .build();

    let file = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new(file_log_pattern)))
    .build(format!("log/{}.log",date))?;

    let latest_log_file = FileAppender::builder()
    .append(false)
    .encoder(Box::new(PatternEncoder::new(file_log_pattern)))
    .build("log/latest.log")?;

    let console_log_filter = ThresholdFilter::new(if verbose { log::LevelFilter::Trace }else{ log::LevelFilter::Info });
    let config = Config::builder()
    .appender(Appender::builder().build("file",Box::new(file)))
    .appender(Appender::builder().build("latestLogFile",Box::new(latest_log_file)))
    .appender(Appender::builder().filter(Box::new(console_log_filter)).build("console",Box::new(stdout)))
    .build(Root::builder()
        .appender("file")
        .appender("latestLogFile")
        .appender("console")
        .build(log::LevelFilter::Trace))?;
    
    return Ok(log4rs::init_config(config)?);
}

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let _handle = init_logger(false)?;

    let args:Vec<String> = std::env::args().collect();
    log::info!("args: {}",&args[1]);
    let mut file = File::open(&args[1])?;
    let sf = SF2::new(&mut file)?;
    log::info!("Soundfont version: {}.{}",sf.info.sf_version[0],sf.info.sf_version[1]);
    log::info!("Target sound engine: {}",sf.info.target_sound_engine);
    log::info!("Soundfont name: {}",sf.info.bank_name);
    log::info!("ROM name: {}",sf.info.rom_name);
    log::info!("ROM version: {}.{}",sf.info.rom_version[0],sf.info.rom_version[1]);
    log::info!("Created at: {}",sf.info.created_date);
    log::info!("Engineers: {}",sf.info.engineers);
    log::info!("Target hardware: {}",sf.info.target_hardware);
    log::info!("Copyright: {}",sf.info.copyright);
    log::info!("Created with: {}",sf.info.created_software);
    log::info!("Comments: {}",sf.info.comments);

    return Ok(());
}