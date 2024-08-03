use crate::soundbank::wsbk::consts::{
    artc_src, artc_dest, artc_transform
};
use crate::soundbank::wsbk::Articulator;

fn normalize(val: f64, val_type: u32) -> f64 {
    return if
        val_type == artc_src::PITCH_WHEEL
    { // 0 - 16383
        val / 16383.0
    } else if
        val_type == artc_src::NOTE_ON_VELOCITY
        || val_type == artc_src::NOTE_NUMBER
        || val_type == artc_src::NOTE_AFTERTOUCH
        || val_type == artc_src::CHANNEL_AFTERTOUCH
        || artc_src::is_midi_cc(val_type)
    { // 0 - 127
        val / 127.0
    } else if
        val_type == artc_src::VOLUME_ENV
        || val_type == artc_src::MODULATION_ENV
    { // 0.0 - 1.0
        val
    } else if
        val_type == artc_src::MODULATION_LFO
        || val_type == artc_src::VIBRATO_LFO
    { // -1.0 - 1.0
        (val + 1.0) / 2.0
    };
}

fn do_process_transform(val: f64, transform_type: u8) -> f64 {
    return if transform_type == artc_transform::LINEAR {
        val
    } else if transform_type == artc_transform::CONCAVE {
        if val > 1.0 - 10.0.powf(-12.0 / 5.0) {
            1.0
        } else {
            (-5.0 / 12.0) * (1.0 - val).log10()
        }
    } else if transform_type == artc_transform::CONVEX {
        if val > 1.0 - 10.0.powf(-12.0 / 5.0) {
            1.0
        } else {
            (-5.0 / 12.0) * (1.0 - val).log10()
        }
    } else if transform_type == artc_transform::SWITCH {
        if val >= 0.5 {
            1.0
        } else {
            0.0
        }
    };
}

fn process_transform(val: f64, transform: u8) -> f64 {
    let transform_type = transform & 0x0f;

    let inverted = artc_transform::is_inverted(transform);
    let bipolar = artc_transform::is_bipolar(transform);

    let val = if inverted { 1.0 - val } else { val };
    return if bipolar {
        let val = val * 2.0 - 1.0;
        val.sgn() * do_process_transform(val.abs())
    } else {
        do_process_transform(val)
    };
}

// 여기서 나온 값을 dest val에 더하게 됨
fn process_articulation(articulator: Articulator, src_val: f64, control_val: f64) -> f64 {
    /* 0.0 - 1.0 범위 또는 -1.0 - 1.0 범위로 변환 */

    // 1차적으로 0.0 - 1.0 범위로 변환
    let src_normalized = normalize(src_val, articulator.src);
    let control_normalized = normalize(control_val, articulator.control);

    // ...그런 다음 변환 함수를 적용+필요 시 -1.0 - 1.0 범위로 변환
    let src_normalized = process_transform(src_normalized, articulator.src_transform);
    let control_normalized = process_transform(control_normalized, articulator.control_transform);

    return process_transform(src_normalized * control_normalized * articulator.scale, articulator.main_transform);
}

pub struct ArticulationUnit {
    articulators: Vec<Articulator>
}

impl ArticulationUnit {
    pub fn process(artc_val: &mut AriculationValues) {}
}

/**
 * Hz 단위: 값 자체는 int의 범위(-2147483648 - 2147483647)와 같으며,
 * 실제로 적용할 때는 이 값을 10000으로 나눈 다음 2의 지수로 집어넣어서 나온 결과값을 Hz 단위로 적용함
 * 즉 23219 => 2.powf(23219.0 / 10000.0) = 약 5Hz가 되는 거고
 * 5Hz => 10000.0 * log2(5) = 약 23219가 되는 거임
 * 따라서 실제 Hz 값의 범위는 거의 0에 가까운 값에서 시작해서 우리가 일반적으로는 세지 못할 정도로 커짐
 * 그러므로 실질적으로는 "값 자체"를 -143000 - 143000 범위 내에서 사용할 것을 권장함. 이렇게 하면 Hz 값의 범위가 약 0.00005Hz - 약 20171Hz가 됨
 * 
 * 시간 단위: 값 자체는 int의 범위(-2147483648 - 2147483647)와 같으며,
 * 실제로 적용할 때는 이 값을 10000으로 나눈 다음 2의 지수로 집어넣어서 나온 결과값을 밀리초 단위로 적용함
 * 즉 23219 => 2.powf(23219.0 / 10000.0) = 약 5ms가 되는 거고
 * 5ms => 10000.0 * log2(5) = 약 23219가 되는 거임
 * 따라서 실제 초 단위 값의 범위는 거의 0에 가까운 값에서 시작해서 우리가 일반적으로는 세지 못할 정도로 커짐
 * 그러므로 실질적으로는 "값 자체"를 -2147483648 - 180000 범위 내에서 사용할 것을 권장함. 이렇게 하면 밀리초 값의 범위가 약 0ms - 262144ms(262.144초)가 됨
 */
pub struct AriculationValues {
    // 게인: 0.001dBFS 단위
    // 즉 -144000 = -144dBFS가 되는 거임
    pub gain: i32,

    // 피치: 0.1cent 단위
    // 즉 12000 = 1200cent = 12key가 되는 거임
    pub pitch: i32,

    // pan: -10000 - 0 - 10000
    pub pan: i32,

    // 0 - 10000 - 100000 (0.01% 단위, 즉 0.0% - 100.0% - 1000.0%)
    // 이 값을 percentage로 치환한 다음 control change 값에 곱해 reverb/chorus send level을 결정함
    pub reverb_send_coeff: i32,
    pub chorus_send_coeff: i32,

    // lfo
    // freq = Hz 단위, start delay = 시간 단위
    pub modulation_lfo_freq: i32,
    pub modulation_lfo_start_delay: i32,
    pub vibrato_lfo_freq: i32,
    pub vibrato_lfo_start_delay: i32,

    // sustain = 0 - 10000 (0.01% 단위)
    // 나머지 = 시간 단위
    pub volume_env_delay: i32,
    pub volume_env_attack: i32,
    pub volume_env_hold: i32,
    pub volume_env_decay: i32,
    pub volume_env_sustain: i32,
    pub volume_env_release: i32,

    // sustain = 0 - 10000 (0.01% 단위)
    // 나머지 = 시간 단위
    pub modulation_env_delay: i32,
    pub modulation_env_attack: i32,
    pub modulation_env_hold: i32,
    pub modulation_env_decay: i32,
    pub modulation_env_sustain: i32,
    pub modulation_env_release: i32,

    // cutoff = Hz 단위
    // q = 0.001dbFS 단위(즉 -144000 = -144dBFS가 되는 거임)
    pub lpf_cutoff: i32,
    pub lpf_q: i32,
    pub hpf_cutoff: i32,
    pub hpf_q: i32
}

impl AriculationValues {
    pub fn new() -> Self {
        return Self {
            gain: 0,
            pitch: 0,
            pan: 0,

            reverb_send_coeff: 10000,
            chorus_send_coeff: 10000,

            modulation_lfo_freq: 0,
            modulation_lfo_start_delay: -2147483648,
            vibrato_lfo_freq: 0,
            vibrato_lfo_start_delay: -2147483648,

            volume_env_delay: -2147483648,
            volume_env_attack: -2147483648,
            volume_env_hold: -2147483648,
            volume_env_decay: -2147483648,
            volume_env_sustain: 10000,
            volume_env_release: -2147483648,

            modulation_env_delay: -2147483648,
            modulation_env_attack: -2147483648,
            modulation_env_hold: -2147483648,
            modulation_env_decay: -2147483648,
            modulation_env_sustain: 10000,
            modulation_env_release: -2147483648,

            lpf_cutoff: 143000,
            lpf_q: 0,
            hpf_cutoff: -2147483648,
            hpf_q: 0
        };
    }
}