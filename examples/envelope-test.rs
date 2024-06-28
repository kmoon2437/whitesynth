use whitesynth::synth::envelope::{ Envelope, EnvelopeMode };

/** gnuplot으로 시각화할 수 있는 형태의 그래프 데이터를 stdout(콘솔)으로 출력 */
fn main() {
    let mut env = Envelope::with_params(48000.0, EnvelopeMode::Normal, 150.0, 100.0, 50.0, 150.0, 0.4, 200.0);
    for i in 0..48000 {
        if i == 2 * 4800 {
            env.attack();
        } else if i == 7 * 4800 {
            env.release();
        }
        env.process(1);
        println!("{} {}", i, env.get_level());
    }
}