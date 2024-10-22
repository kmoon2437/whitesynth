/**
 * 샘플레이트 48000 + 32비트 little endian float 샘플(f32le)에서만 동작함
 */

use std::fs::File;
use std::io::{ Read, Write };
use whitesynth::synth::effects::distortion::Distortion;
use whitesynth::synth::effects::amp_simulator::GuitarAmpSimulator;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {} {}", &args[1], &args[2]);
    let mut file = File::open(&args[1])?;
    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf)?;
    let len = buf.len()/(2*4); // 채널 2개, 32비트 float 샘플(little endian), 초당 샘플 수 48000
    println!("len: {}", len);

    let mut new_file = File::create(&args[2])?;

    let mut dist_l = Distortion::new(48000.0);
    let mut dist_r = Distortion::new(48000.0);
    dist_l.set_drive(100.0);
    dist_r.set_drive(100.0);
    let mut amp_l = GuitarAmpSimulator::new(48000.0);
    let mut amp_r = GuitarAmpSimulator::new(48000.0);
    amp_l.set_drive(0.0);
    amp_r.set_drive(0.0);
    let final_gain = 0.3;

    let mut left = vec![0.0; len];
    let mut right = vec![0.0; len];

    for ii in 0..len {
        let i = ii*8;
        left[ii] = f32::from_le_bytes(buf[i..=(i+3)].try_into()?) as f64;
        right[ii] = f32::from_le_bytes(buf[(i+4)..=(i+7)].try_into()?) as f64;
    }

    dist_l.process(&mut left);
    dist_r.process(&mut right);
    println!("amp processing");
    let now = std::time::Instant::now();
    amp_l.process(&mut left);
    amp_r.process(&mut right);
    println!("{}", now.elapsed().as_micros());

    let mut left_max_abs: f64 = 0.0;
    let mut right_max_abs: f64 = 0.0;
    for (src_left, src_right) in left.iter_mut().zip(right.iter_mut()) {
        *src_left *= final_gain;
        *src_right *= final_gain;
        left_max_abs = left_max_abs.max(src_left.abs());
        right_max_abs = right_max_abs.max(src_right.abs());
    }
    println!("{} {}", left_max_abs, right_max_abs);

    for (src_left, src_right) in left.iter_mut().zip(right.iter_mut()) {
        new_file.write_all(&((*src_left as f32).to_le_bytes()))?;
        new_file.write_all(&((*src_right as f32).to_le_bytes()))?;
    }

    println!("completed");

    return Ok(());
}