/**
 * 참고 문헌:
 * - https://www.utsbox.com/?p=2390
 * - https://static.choyunjin.kr/files/pdf/sfspec/sfspec24.pdf
 */

pub const START_ADDRS_OFFSET: u16 = 0;
pub const END_ADDRS_OFFSET: u16 = 1;
pub const START_LOOP_ADDRS_OFFSET: u16 = 2;
pub const END_LOOP_ADDRS_OFFSET: u16 = 3;
pub const START_ADDRS_COARSE_OFFSET: u16 = 4;
pub const MOD_LFO_TO_PITCH: u16 = 5;
pub const VIB_LFO_TO_PITCH: u16 = 6;
pub const MOD_ENV_TO_PITCH: u16 = 7;
pub const INITIAL_FILTER_FC: u16 = 8;
pub const INITIAL_FILTER_Q: u16 = 9;
pub const MOD_LFO_TO_FILTER_FC: u16 = 10;
pub const MOD_ENV_TO_FILTER_FC: u16 = 11;
pub const END_ADDRS_COARSE_OFFSET: u16 = 12;
pub const MOD_LFO_TO_VOLUME: u16 = 13;
pub const UNUSED_1: u16 = 14;
pub const CHORUS_EFFECTS_SEND: u16 = 15;
pub const REVERB_EFFECTS_SEND: u16 = 16;
pub const PAN: u16 = 17;
pub const UNUSED_2: u16 = 18;
pub const UNUSED_3: u16 = 19;
pub const UNUSED_4: u16 = 20;
pub const DELAY_MOD_LFO: u16 = 21;
pub const FREQ_MOD_LFO: u16 = 22;
pub const DELAY_VIB_LFO: u16 = 23;
pub const FREQ_VIB_LFO: u16 = 24;
pub const DELAY_MOD_ENV: u16 = 25;
pub const ATTACK_MOD_ENV: u16 = 26;
pub const HOLD_MOD_ENV: u16 = 27;
pub const DECAY_MOD_ENV: u16 = 28;
pub const SUSTAIN_MOD_ENV: u16 = 29;
pub const RELEASE_MOD_ENV: u16 = 30;
pub const KEYNUM_TO_MOD_ENV_HOLD: u16 = 31;
pub const KEYNUM_TO_MOD_ENV_DECAY: u16 = 32;
pub const DELAY_VOL_ENV: u16 = 33;
pub const ATTACK_VOL_ENV: u16 = 34;
pub const HOLD_VOL_ENV: u16 = 35;
pub const DECAY_VOL_ENV: u16 = 36;
pub const SUSTAIN_VOL_ENV: u16 = 37;
pub const RELEASE_VOL_ENV: u16 = 38;
pub const KEYNUM_TO_VOL_ENV_HOLD: u16 = 39;
pub const KEYNUM_TO_VOL_ENV_DELAY: u16 = 40;
pub const INSTRUMENT: u16 = 41;
pub const RESERVED_1: u16 = 42;
pub const KEY_RANGE: u16 = 43;
pub const VEL_RANGE: u16 = 44;
pub const START_LOOP_ADDRS_COARSE_OFFSET: u16 = 45;
pub const KEYNUM: u16 = 46;
pub const VELOCITY: u16 = 47;
pub const INITIAL_ATTEUNATION: u16 = 48;
pub const RESERVED_2: u16 = 49;
pub const END_LOOP_ADDRS_COARSE_OFFSET: u16 = 50;
pub const COARSE_TUNE: u16 = 51;
pub const FINE_TUNE: u16 = 52;
pub const SAMPLE_ID: u16 = 53;
pub const SAMPLE_MODES: u16 = 54;
pub const RESERVED_3: u16 = 55;
pub const SCALE_TUNING: u16 = 56;
pub const EXCLUSIVE_CLASS: u16 = 57;
pub const OVERRIDING_ROOT_KEY: u16 = 58;
pub const UNUSED_5: u16 = 59;
pub const END_OPER: u16 = 60;