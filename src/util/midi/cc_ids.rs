/**
 * control change 상수 모음
 */

pub const BANK_SELECT: u8 = 0x00;
pub const MODULATION: u8 = 0x01;
pub const BREATH: u8 = 0x02;

pub const FOOT: u8 = 0x04;
pub const PORTAMENTO_TIME: u8 = 0x05;
pub const DATA_ENTRY_MSB: u8 = 0x06;
pub const CHANNEL_VOLUME: u8 = 0x07;
pub const BALANCE: u8 = 0x08;

pub const PAN: u8 = 0x0a;
pub const EXPRESSION: u8 = 0x0b;
pub const EFFECT_1: u8 = 0x0c;
pub const EFFECT_2: u8 = 0x0d;

pub const GENERAL_PURPOSE_1: u8 = 0x10;
pub const TONE_PARAMETER_1: u8 = 0x10;
pub const GENERAL_PURPOSE_2: u8 = 0x11;
pub const TONE_PARAMETER_2: u8 = 0x11;
pub const GENERAL_PURPOSE_3: u8 = 0x12;
pub const TONE_PARAMETER_3: u8 = 0x12;
pub const GENERAL_PURPOSE_4: u8 = 0x13;
pub const TONE_PARAMETER_4: u8 = 0x13;

pub const BANK_SELECT_LSB: u8 = 0x20;
pub const MODULATION_LSB: u8 = 0x21;
pub const BREATH_LSB: u8 = 0x22;
pub const CONTROLLER_0X03_LSB: u8 = 0x23;
pub const FOOT_LSB: u8 = 0x24;
pub const PORTAMENTO_TIME_LSB: u8 = 0x25;
pub const DATA_ENTRY_LSB: u8 = 0x26;
pub const CHANNEL_VOLUME_LSB: u8 = 0x27;
pub const BALANCE_LSB: u8 = 0x28;
pub const CONTROLLER_0X09_LSB: u8 = 0x29;
pub const PAN_LSB: u8 = 0x2a;
pub const EXPRESSION_LSB: u8 = 0x2b;
pub const EFFECT_1_LSB: u8 = 0x2c;
pub const EFFECT_2_LSB: u8 = 0x2d;
pub const CONTROLLER_0X0E_LSB: u8 = 0x2e;
pub const CONTROLLER_0X0F_LSB: u8 = 0x2f;
pub const GENERAL_PURPOSE_1_LSB: u8 = 0x30;
pub const GENERAL_PURPOSE_2_LSB: u8 = 0x31;
pub const GENERAL_PURPOSE_3_LSB: u8 = 0x32;
pub const GENERAL_PURPOSE_4_LSB: u8 = 0x33;
pub const CONTROLLER_0X14_LSB: u8 = 0x34;
pub const CONTROLLER_0X15_LSB: u8 = 0x35;
pub const CONTROLLER_0X16_LSB: u8 = 0x36;
pub const CONTROLLER_0X17_LSB: u8 = 0x37;
pub const CONTROLLER_0X18_LSB: u8 = 0x38;
pub const CONTROLLER_0X19_LSB: u8 = 0x39;
pub const CONTROLLER_0X1A_LSB: u8 = 0x3a;
pub const CONTROLLER_0X1B_LSB: u8 = 0x3b;
pub const CONTROLLER_0X1C_LSB: u8 = 0x3c;
pub const CONTROLLER_0X1D_LSB: u8 = 0x3d;
pub const CONTROLLER_0X1E_LSB: u8 = 0x3e;
pub const CONTROLLER_0X1F_LSB: u8 = 0x3f;

pub const SUSTAIN_ONOFF: u8 = 0x40;
pub const PORTAMENTO_ONOFF: u8 = 0x41;
pub const SOSTENUTO_ONOFF: u8 = 0x42;
pub const SOFT_PEDAL_ONOFF: u8 = 0x43;
pub const LEGATO_FOOTSWITCH: u8 = 0x44;
pub const HOLD_2: u8 = 0x45;

// sound controller 1~9(일종의 alias)
pub const SOUND_VARIATION: u8 = 0x46; // sound controller 1
pub const TIMBRE_HARMONIC_INTENS: u8 = 0x47; // sound controller 2(LPF resonance)
pub const RELEASE_TIME: u8 = 0x48; // sound controller 3
pub const ATTACK_TIME: u8 = 0x49; // sound controller 4
pub const LPF_CUTOFF_FREQUENCY: u8 = 0x4a; // sound controller 5
pub const DECAY_TIME: u8 = 0x4b; // sound controller 6
pub const VIBRATO_RATE: u8 = 0x4c; // sound controller 7
pub const VIBRATO_DEPTH: u8 = 0x4d; // sound controller 8
pub const VIBRATO_DELAY: u8 = 0x4e; // sound controller 9
pub const SUSTAIN_LEVEL: u8 = 0x4f; // sound controller 10

pub const SOUND_CONTROLLER_1: u8 = 0x46;
pub const SOUND_CONTROLLER_2: u8 = 0x47;
pub const SOUND_CONTROLLER_3: u8 = 0x48;
pub const SOUND_CONTROLLER_4: u8 = 0x49;
pub const SOUND_CONTROLLER_5: u8 = 0x4a;
pub const SOUND_CONTROLLER_6: u8 = 0x4b;
pub const SOUND_CONTROLLER_7: u8 = 0x4c;
pub const SOUND_CONTROLLER_8: u8 = 0x4d;
pub const SOUND_CONTROLLER_9: u8 = 0x4e;
pub const SOUND_CONTROLLER_10: u8 = 0x4f;

pub const GENERAL_PURPOSE_5: u8 = 0x50;
pub const GENERAL_PURPOSE_6: u8 = 0x51;
pub const HPF_CUTOFF_FREQUENCY: u8 = 0x51;
pub const GENERAL_PURPOSE_7: u8 = 0x52;
pub const GENERAL_PURPOSE_8: u8 = 0x53;
pub const PORTAMENTO_CONTROL: u8 = 0x54;

pub const HIGH_RESOLUTION_VELOCITY_PREFIX: u8 = 0x58;

pub const REVERB_SEND_LEVEL: u8 = 0x5b;
pub const TREMOLO_DEPTH: u8 = 0x5c;
pub const CHORUS_SEND_LEVEL: u8 = 0x5d;
pub const DELAY_SEND_LEVEL: u8 = 0x5e;
pub const PHASER_DEPTH: u8 = 0x5f;
pub const DATA_INCREMENT: u8 = 0x60;
pub const DATA_DECREMENT: u8 = 0x61;
pub const NRPN_LSB: u8 = 0x62;
pub const NRPN_MSB: u8 = 0x63;
pub const RPN_LSB: u8 = 0x64;
pub const RPN_MSB: u8 = 0x65;

pub const ALL_SOUND_OFF: u8 = 0x78;
pub const RESET_ALL_CONTROLLERS: u8 = 0x79;
pub const LOCAL_CONTROL_ONOFF: u8 = 0x7a;
pub const ALL_NOTES_OFF: u8 = 0x7b;
pub const OMNI_OFF: u8 = 0x7c;
pub const OMNI_ON: u8 = 0x7d;
pub const MONO_ON: u8 = 0x7e;
pub const POLY_ON: u8 = 0x7f;