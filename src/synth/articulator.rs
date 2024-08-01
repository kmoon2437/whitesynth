use crate::soundbank::wsbk::{ self, articulators };

pub struct Articulator {
    data: wsbk::Articulator
}

fn normalize(val: f64, val_type: u32) -> f64 {
    return if
        val_type == articulators::src::PITCH_WHEEL
    { // 0 - 16383
        val / 16383.0
    } else if
        val_type == articulators::src::NOTE_ON_VELOCITY
        || val_type == articulators::src::NOTE_NUMBER
        || val_type == articulators::src::NOTE_AFTERTOUCH
        || val_type == articulators::src::CHANNEL_AFTERTOUCH
        || articulators::src::is_midi_cc(val_type)
    { // 0 - 127
        val / 127.0
    } else if
        val_type == articulators::src::VOLUME_ENV
        || val_type == articulators::src::MODULATION_ENV
    { // 0.0 - 1.0
        val
    } else if
        val_type == articulators::src::MODULATION_LFO
        || val_type == articulators::src::VIBRATO_LFO
    { // -1.0 - 1.0
        (val + 1.0) / 2.0
    };
}

fn do_process_transform(val: f64, transform_type: u8) {
    return if transform_type == articulators::transform::LINEAR {
        val
    } else if transform_type == articulators::transform::CONCAVE {
        if val > 1.0 - 10.0.powf(-12.0 / 5.0) {
            1.0
        } else {
            (-5.0 / 12.0) * (1.0 - val).log10()
        }
    } else if transform_type == articulators::transform::CONVEX {
        if val > 1.0 - 10.0.powf(-12.0 / 5.0) {
            1.0
        } else {
            (-5.0 / 12.0) * (1.0 - val).log10()
        }
    } else if transform_type == articulators::transform::SWITCH {
        if val >= 0.5 {
            1.0
        } else {
            0.0
        }
    };
}

fn process_transform(val: f64, transform: u8) -> f64 {
    let transform_type = transform & 0x0f;

    let inverted = articulators::transform::is_inverted(transform);
    let bipolar = articulators::transform::is_bipolar(transform);

    let val = if inverted { 1.0 - val } else { val };
    return if bipolar {
        let val = val * 2.0 - 1.0;
        val.sgn() * do_process_transform(val.abs())
    } else {
        do_process_transform(val)
    };
}

impl Articulator {
    // 여기서 나온 값을 dest val에 더하게 됨
    pub fn process(&self, src_val: f64, control_val: f64) {
        /* 0.0 - 1.0 범위 또는 -1.0 - 1.0 범위로 변환 */

        // 1차적으로 0.0 - 1.0 범위로 변환
        let src_normalized = normalize(src_val, self.data.src);
        let control_normalized = normalize(control_val, self.data.control);

        // ...그런 다음 변환 함수를 적용+필요 시 -1.0 - 1.0 범위로 변환
        let src_normalized = process_transform(src_normalized, self.data.src_transform);
        let control_normalized = process_transform(control_normalized, self.data.control_transform);

        return process_transform(src_normalized * control_normalized * self.data.scale, self.data.main_transform);
    }
}