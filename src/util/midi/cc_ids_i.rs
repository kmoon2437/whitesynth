/**
 * control change 상수 모음의 usize 버전
 */

pub const BANK_SELECT: usize = 0x00;
pub const MODULATION: usize = 0x01;
pub const BREATH: usize = 0x02;

pub const FOOT: usize = 0x04;
pub const PORTAMENTO_TIME: usize = 0x05;
pub const DATA_ENTRY_MSB: usize = 0x06;
pub const CHANNEL_VOLUME: usize = 0x07;
pub const BALANCE: usize = 0x08;

pub const PAN: usize = 0x0a;
pub const EXPRESSION: usize = 0x0b;
pub const EFFECT_1: usize = 0x0c;
pub const EFFECT_2: usize = 0x0d;

pub const GENERAL_PURPOSE_1: usize = 0x10;
pub const TONE_PARAMETER_1: usize = 0x10;
pub const GENERAL_PURPOSE_2: usize = 0x11;
pub const TONE_PARAMETER_2: usize = 0x11;
pub const GENERAL_PURPOSE_3: usize = 0x12;
pub const TONE_PARAMETER_3: usize = 0x12;
pub const GENERAL_PURPOSE_4: usize = 0x13;
pub const TONE_PARAMETER_4: usize = 0x13;

pub const BANK_SELECT_LSB: usize = 0x20;
pub const MODULATION_LSB: usize = 0x21;
pub const BREATH_LSB: usize = 0x22;
pub const CONTROLLER_0X03_LSB: usize = 0x23;
pub const FOOT_LSB: usize = 0x24;
pub const PORTAMENTO_TIME_LSB: usize = 0x25;
pub const DATA_ENTRY_LSB: usize = 0x26;
pub const CHANNEL_VOLUME_LSB: usize = 0x27;
pub const BALANCE_LSB: usize = 0x28;
pub const CONTROLLER_0X09_LSB: usize = 0x29;
pub const PAN_LSB: usize = 0x2a;
pub const EXPRESSION_LSB: usize = 0x2b;
pub const EFFECT_1_LSB: usize = 0x2c;
pub const EFFECT_2_LSB: usize = 0x2d;
pub const CONTROLLER_0X0E_LSB: usize = 0x2e;
pub const CONTROLLER_0X0F_LSB: usize = 0x2f;
pub const GENERAL_PURPOSE_1_LSB: usize = 0x30;
pub const GENERAL_PURPOSE_2_LSB: usize = 0x31;
pub const GENERAL_PURPOSE_3_LSB: usize = 0x32;
pub const GENERAL_PURPOSE_4_LSB: usize = 0x33;
pub const CONTROLLER_0X14_LSB: usize = 0x34;
pub const CONTROLLER_0X15_LSB: usize = 0x35;
pub const CONTROLLER_0X16_LSB: usize = 0x36;
pub const CONTROLLER_0X17_LSB: usize = 0x37;
pub const CONTROLLER_0X18_LSB: usize = 0x38;
pub const CONTROLLER_0X19_LSB: usize = 0x39;
pub const CONTROLLER_0X1A_LSB: usize = 0x3a;
pub const CONTROLLER_0X1B_LSB: usize = 0x3b;
pub const CONTROLLER_0X1C_LSB: usize = 0x3c;
pub const CONTROLLER_0X1D_LSB: usize = 0x3d;
pub const CONTROLLER_0X1E_LSB: usize = 0x3e;
pub const CONTROLLER_0X1F_LSB: usize = 0x3f;

pub const SUSTAIN_ONOFF: usize = 0x40;
pub const PORTAMENTO_ONOFF: usize = 0x41;
pub const SOSTENUTO_ONOFF: usize = 0x42;
pub const SOFT_PEDAL_ONOFF: usize = 0x43;
pub const LEGATO_FOOTSWITCH: usize = 0x44;
pub const HOLD_2: usize = 0x45;

// sound controller 1~9(일종의 alias)
pub const SOUND_VARIATION: usize = 0x46; // sound controller 1
pub const TIMBRE_HARMONIC_INTENS: usize = 0x47; // sound controller 2(LPF resonance)
pub const RELEASE_TIME: usize = 0x48; // sound controller 3
pub const ATTACK_TIME: usize = 0x49; // sound controller 4
pub const LPF_CUTOFF_FREQUENCY: usize = 0x4a; // sound controller 5
pub const DECAY_TIME: usize = 0x4b; // sound controller 6
pub const VIBRATO_RATE: usize = 0x4c; // sound controller 7
pub const VIBRATO_DEPTH: usize = 0x4d; // sound controller 8
pub const VIBRATO_DELAY: usize = 0x4e; // sound controller 9
pub const SUSTAIN_LEVEL: usize = 0x4f; // sound controller 10

pub const SOUND_CONTROLLER_1: usize = 0x46;
pub const SOUND_CONTROLLER_2: usize = 0x47;
pub const SOUND_CONTROLLER_3: usize = 0x48;
pub const SOUND_CONTROLLER_4: usize = 0x49;
pub const SOUND_CONTROLLER_5: usize = 0x4a;
pub const SOUND_CONTROLLER_6: usize = 0x4b;
pub const SOUND_CONTROLLER_7: usize = 0x4c;
pub const SOUND_CONTROLLER_8: usize = 0x4d;
pub const SOUND_CONTROLLER_9: usize = 0x4e;
pub const SOUND_CONTROLLER_10: usize = 0x4f;

pub const GENERAL_PURPOSE_5: usize = 0x50;
pub const GENERAL_PURPOSE_6: usize = 0x51;
pub const HPF_CUTOFF_FREQUENCY: usize = 0x51;
pub const GENERAL_PURPOSE_7: usize = 0x52;
pub const GENERAL_PURPOSE_8: usize = 0x53;
pub const PORTAMENTO_CONTROL: usize = 0x54;

pub const HIGH_RESOLUTION_VELOCITY_PREFIX: usize = 0x58;

pub const REVERB_SEND_LEVEL: usize = 0x5b;
pub const TREMOLO_DEPTH: usize = 0x5c;
pub const CHORUS_SEND_LEVEL: usize = 0x5d;
pub const DELAY_SEND_LEVEL: usize = 0x5e;
pub const PHASER_DEPTH: usize = 0x5f;
pub const DATA_INCREMENT: usize = 0x60;
pub const DATA_DECREMENT: usize = 0x61;
pub const NRPN_LSB: usize = 0x62;
pub const NRPN_MSB: usize = 0x63;
pub const RPN_LSB: usize = 0x64;
pub const RPN_MSB: usize = 0x65;

pub const ALL_SOUND_OFF: usize = 0x78;
pub const RESET_ALL_CONTROLLERS: usize = 0x79;
pub const LOCAL_CONTROL_ONOFF: usize = 0x7a;
pub const ALL_NOTES_OFF: usize = 0x7b;
pub const OMNI_OFF: usize = 0x7c;
pub const OMNI_ON: usize = 0x7d;
pub const MONO_ON: usize = 0x7e;
pub const POLY_ON: usize = 0x7f;