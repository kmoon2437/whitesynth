use whitesynth::synth::param_smoother::{ ParamSmoother };

/** gnuplot으로 시각화할 수 있는 형태의 그래프 데이터를 stdout(콘솔)으로 출력 */
fn main() {
    let time = std::time::Instant::now();
    let mut smoother = ParamSmoother::new(10.0, 48000.0);
    let mut val = 0.0;
    for i in 0..48000 {
        if i == 2 * 4800 {
            val = 127.0;
        } else if i == 7 * 4800 {
            val = 0.0;
        }
        println!("{} {}", i, smoother.process(val));
    }
    eprintln!("{}", time.elapsed().as_micros());
}