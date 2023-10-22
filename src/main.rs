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

// 실제 synth 관련
use whitesynth::util;
use whitesynth::synth::effects::filter::Filter;

// 테스트용
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::convert::TryInto;

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

fn slice2array(slice: &[u8]) -> [u8;2]{
    return [slice[0],slice[1]];
}

fn main(){
    //log4rs::init_file("log4rs.yml",Default::default()).unwrap();
    let _handle = init_logger(false);

    let args:Vec<String> = std::env::args().collect();
    log::info!("args: {} {}",&args[1],&args[2]);
    let mut file = File::open(&args[1]).unwrap();
    let mut buf:Vec<u8> = vec![];
    file.read_to_end(&mut buf).unwrap();
    let len = buf.len()/(2*2); // 채널 2개, signed 16비트 샘플, 초당 샘플 수 48000
    log::info!("len: {}",len);

    let mut processorL = Filter::new(48000.0);
    let mut processorR = Filter::new(48000.0);
    let q = (std::f64::consts::PI/4.0).sin();

    // low-pass filter(저음역대만 통과)
    let freq = 440.0;
    processorL.low_pass(freq,q);
    processorR.low_pass(freq,q);

    // high-pass filter(고음역대만 통과)
    /*let freq = 1972.0;
    processorL.high_pass(freq,q);
    processorR.high_pass(freq,q);*/

    let mut newFile = File::create(&args[2]).unwrap();
    for ii in 0..len{
        let i = ii*4;
        let left = i16::from_le_bytes(slice2array(&buf[i..=(i+1)]));
        let right = i16::from_le_bytes(slice2array(&buf[(i+2)..=(i+3)]));
        let left_f = (left as f64) / (if left < 0 { 32768.0 }else{ 32767.0 });
        let right_f = (right as f64) / (if right < 0 { 32768.0 }else{ 32767.0 });
        let o_left_f = processorL.process(left_f);
        let o_right_f = processorR.process(right_f);
        let o_left = (o_left_f / 1.0 * (if o_left_f < 0.0 { 32768.0 }else{ 32767.0 })) as i16;
        let o_right = (o_right_f / 1.0 * (if o_right_f < 0.0 { 32768.0 }else{ 32767.0 })) as i16;
        if ii % 480000 == 0 {
            //println!("{} {} {}",i,o_left,o_right);
            log::debug!("{} {} {}",i,o_left,o_right);
        }
        newFile.write_all(&(o_left.to_le_bytes()));
        newFile.write_all(&(o_right.to_le_bytes()));
    }
}