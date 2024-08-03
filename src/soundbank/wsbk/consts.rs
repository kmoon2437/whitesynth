pub mod artc_src { // source, control
    // 기본: 0x0000 - 0x00ff
    pub const NONE: u32 = 0x0000;
    pub const PITCH_WHEEL: u32 = 0x0001;
    pub const NOTE_ON_VELOCITY: u32 = 0x0002;
    pub const NOTE_NUMBER: u32 = 0x0003;
    pub const VOLUME_ENV: u32 = 0x0004;
    pub const MODULATION_ENV: u32 = 0x0005;
    pub const NOTE_AFTERTOUCH: u32 = 0x0006;
    pub const CHANNEL_AFTERTOUCH: u32 = 0x0007;
    pub const MODULATION_LFO: u32 = 0x0008;
    pub const VIBRATO_LFO: u32 = 0x0009;

    // midi control change: 0x00010000 - 0x000100ff
    pub const MIDI_CONTROL_CHANGE: u32 = 0x00010000;
    pub fn midi_cc(no: u32) -> u32 {
        return MIDI_CONTROL_CHANGE + no;
    }
    pub fn is_midi_cc(val: u32) -> bool {
        return MIDI_CONTROL_CHANGE == (val & 0xffff0000);
    }

    // midi rpn: 0x00020000 - 0x0002ffff
    pub const MIDI_RPN: u32 = 0x00020000;
    pub fn midi_rpn(msb: u32, lsb: u32) -> u32 {
        return MIDI_RPN + (msb << 8) + lsb;
    }
    pub fn is_midi_rpn(val: u32) -> bool {
        return MIDI_RPN == (val & 0xffff0000);
    }

    // midi nrpn: 0x00040000 - 0x0004ffff
    pub const MIDI_NRPN: u32 = 0x00040000;
    pub fn midi_nrpn(msb: u32, lsb: u32) -> u32 {
        return MIDI_NRPN + (msb << 8) + lsb;
    }
    pub fn is_midi_nrpn(val: u32) -> bool {
        return MIDI_NRPN == (val & 0xffff0000);
    }
}

pub mod artc_dest { // destination
    pub const NONE: u32 = 0x0000;
    pub const GAIN: u32 = 0x0001;
    pub const PITCH: u32 = 0x0002;
    pub const PAN: u32 = 0x0003;

    // 얘넨 딱히 구현 안 할 예정
    pub const NOTE_ON_VELOCITY: u32 = 0x0004;
    pub const NOTE_NUMBER: u32 = 0x0005;

    // 6채널(5.1채널?) 출력용이라 구현 안할 수도 있음
    pub const LEFT_SEND: u32 = 0x0020;
    pub const RIGHT_SEND: u32 = 0x0021;
    pub const CENTER_SEND: u32 = 0x0022;
    pub const LFE_CHANNEL_SEND: u32 = 0x0023;
    pub const LEFT_REAR_SEND: u32 = 0x0024;
    pub const RIGHT_REAR_SEND: u32 = 0x0025;

    pub const REVERB_SEND_COEFF: u32 = 0x0026;
    pub const CHORUS_SEND_COEFF: u32 = 0x0027;

    pub const MODULATION_LFO_FREQUENCY: u32 = 0x0100;
    pub const MODULATION_LFO_START_DELAY: u32 = 0x0101;
    pub const VIBRATO_LFO_FREQUENCY: u32 = 0x0110;
    pub const VIBRATO_LFO_START_DELAY: u32 = 0x0111;

    pub const VOLUME_ENV_DELAY: u32 = 0x0200;
    pub const VOLUME_ENV_ATTACK: u32 = 0x0201;
    pub const VOLUME_ENV_HOLD: u32 = 0x0202;
    pub const VOLUME_ENV_DECAY: u32 = 0x0203;
    pub const VOLUME_ENV_SUSTAIN: u32 = 0x0204;
    pub const VOLUME_ENV_RELEASE: u32 = 0x0205;
    pub const VOLUME_ENV_SHUTDOWN: u32 = 0x0206;

    pub const MODULATION_ENV_DELAY: u32 = 0x0300;
    pub const MODULATION_ENV_ATTACK: u32 = 0x0301;
    pub const MODULATION_ENV_HOLD: u32 = 0x0302;
    pub const MODULATION_ENV_DECAY: u32 = 0x0303;
    pub const MODULATION_ENV_SUSTAIN: u32 = 0x0304;
    pub const MODULATION_ENV_RELEASE: u32 = 0x0305;
    pub const MODULATION_ENV_SHUTDOWN: u32 = 0x0306;

    pub const LPF_CUTOFF: u32 = 0x0500;
    pub const LPF_Q: u32 = 0x0501;
    pub const HPF_CUTOFF: u32 = 0x0600;
    pub const HPF_Q: u32 = 0x0601;
}

pub mod artc_transform {
    pub const LINEAR: u8 = 0x0;
    pub const CONCAVE: u8 = 0x1;
    pub const CONVEX: u8 = 0x2;
    pub const SWITCH: u8 = 0x3;

    pub const INVERTED: u8 = 0x10;
    pub const BIPOLAR: u8 = 0x20;

    pub fn is_inverted(transform: u8) -> bool {
        return (transform & 0xf0) == INVERTED;
    }
    pub fn is_bipolar(transform: u8) -> bool {
        return (transform & 0xf0) == BIPOLAR;
    }
}