#![allow(non_snake_case)]

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

fn initLogger(verbose: bool) -> log4rs::Handle {
    let fileLogPattern = "{d(%Y-%m-%d %H:%M:%S %Z)} [{l}] {m}{n}";
    let consoleLogPattern = "{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}{n}";
    let date = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let stdout = ConsoleAppender::builder()
    .encoder(Box::new(PatternEncoder::new(consoleLogPattern)))
    .build();

    let file = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new(fileLogPattern)))
    .build(format!("log/{}.log",date))
    .unwrap();

    let latestLogFile = FileAppender::builder()
    .append(false)
    .encoder(Box::new(PatternEncoder::new(fileLogPattern)))
    .build("log/latest.log")
    .unwrap();

    let consoleLogFilter = ThresholdFilter::new(if verbose { log::LevelFilter::Trace }else{ log::LevelFilter::Info });
    let config = Config::builder()
    .appender(Appender::builder().build("file",Box::new(file)))
    .appender(Appender::builder().build("latestLogFile",Box::new(latestLogFile)))
    .appender(Appender::builder().filter(Box::new(consoleLogFilter)).build("console",Box::new(stdout)))
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
    let _handle = initLogger(false);

    let a1:[i32;3] = [1,2,3];
    let a2:[i32;4] = [1,1,3,4];
    log::debug!("{}",whitesynth::util::compareArray(&a1,&a2));

    log::info!("아ㅏㅏ무것도 없다 이런 얘기예요");
    log::warn!("{}",whitesynth::consts::AAAAAAMUGOTTO_HAJI_ANKO_GUGYONGMAN_HESSOYO);
    log::error!("AHHHHH NIMI SHIRAIT");
}