//#![allow(non_snake_case)]

/**
 * whitesynth cli
 */

extern crate whitesynth;
extern crate log4rs;
extern crate log;
extern crate chrono;

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

fn init_logger(verbose: bool) -> log4rs::Handle {
    let file_log_pattern = "{d(%Y-%m-%d %H:%M:%S %Z)} [{l}] {m}{n}";
    let console_log_pattern = "{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}{n}";
    let date = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let stdout = ConsoleAppender::builder()
    .encoder(Box::new(PatternEncoder::new(console_log_pattern)))
    .build();

    let file = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new(file_log_pattern)))
    .build(format!("log/{}.log",date))
    .unwrap();

    let latest_log_file = FileAppender::builder()
    .append(false)
    .encoder(Box::new(PatternEncoder::new(file_log_pattern)))
    .build("log/latest.log")
    .unwrap();

    let console_log_filter = ThresholdFilter::new(if verbose { log::LevelFilter::Trace }else{ log::LevelFilter::Info });
    let config = Config::builder()
    .appender(Appender::builder().build("file",Box::new(file)))
    .appender(Appender::builder().build("latestLogFile",Box::new(latest_log_file)))
    .appender(Appender::builder().filter(Box::new(console_log_filter)).build("console",Box::new(stdout)))
    .build(Root::builder()
        .appender("file")
        .appender("latestLogFile")
        .appender("console")
        .build(log::LevelFilter::Trace))
    .unwrap();
    
    return log4rs::init_config(config).unwrap();
}

fn main(){
    //log4rs::init_file("log4rs.yml",Default::default()).unwrap();
    let _handle = init_logger(false);

    let a1:[i32;3] = [1,2,3];
    let a2:[i32;4] = [1,1,3,4];
    log::debug!("{}",whitesynth::util::compare_array(&a1,&a2));

    log::info!("아ㅏㅏ무것도 없다 이런 얘기예요");
    log::warn!("{}",whitesynth::consts::AAAAAAMUGOTTO_HAJI_ANKO_GUGYONGMAN_HESSOYO);
    log::error!("AHHHHH NIMI SHIRAIT");
}