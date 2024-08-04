/**
 * midi 관련 보조 함수/상수
 */

pub mod cc_ids;
pub mod cc_ids_i;

// 모든 파라미터가 기본값을 갖는 cc 파라미터 배열을 반환
// 주석 처리된 부분은 현재는 사용하지 않으나 추후 사용할 수도 있다는 뜻임
pub fn get_initial_cc() -> [u8; 128] {
    let mut cc_values = [0; 128];

    cc_values[cc_ids_i::BANK_SELECT] = 0;
    cc_values[cc_ids_i::MODULATION] = 0;

    cc_values[cc_ids_i::BREATH] = 0;
    cc_values[cc_ids_i::FOOT] = 0;

    cc_values[cc_ids_i::PORTAMENTO_TIME] = 0;
    cc_values[cc_ids_i::DATA_ENTRY_MSB] = 0;
    cc_values[cc_ids_i::CHANNEL_VOLUME] = 100;

    //cc_values[cc_ids_i::BALANCE] = 0;

    cc_values[cc_ids_i::PAN] = 64;
    cc_values[cc_ids_i::EXPRESSION] = 127;

    /*cc_values[cc_ids_i::EFFECT_1] = 0;
    cc_values[cc_ids_i::EFFECT_2] = 0;

    cc_values[cc_ids_i::GENERAL_PURPOSE_1] = 0;
    cc_values[cc_ids_i::GENERAL_PURPOSE_2] = 0;
    cc_values[cc_ids_i::GENERAL_PURPOSE_3] = 0;
    cc_values[cc_ids_i::GENERAL_PURPOSE_4] = 0;*/

    cc_values[cc_ids_i::BANK_SELECT_LSB] = 0;

    /*cc_values[cc_ids_i::MODULATION_LSB] = 0;
    cc_values[cc_ids_i::BREATH_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X03_LSB] = 0;
    cc_values[cc_ids_i::FOOT_LSB] = 0;
    cc_values[cc_ids_i::PORTAMENTO_TIME_LSB] = 0;*/

    cc_values[cc_ids_i::DATA_ENTRY_LSB] = 0;

    /*cc_values[cc_ids_i::CHANNEL_VOLUME_LSB] = 0;
    cc_values[cc_ids_i::BALANCE_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X09_LSB] = 0;
    cc_values[cc_ids_i::PAN_LSB] = 0;
    cc_values[cc_ids_i::EXPRESSION_LSB] = 0;
    cc_values[cc_ids_i::EFFECT_1_LSB] = 0;
    cc_values[cc_ids_i::EFFECT_2_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X0E_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X0F_LSB] = 0;
    cc_values[cc_ids_i::GENERAL_PURPOSE_1_LSB] = 0;
    cc_values[cc_ids_i::GENERAL_PURPOSE_2_LSB] = 0;
    cc_values[cc_ids_i::GENERAL_PURPOSE_3_LSB] = 0;
    cc_values[cc_ids_i::GENERAL_PURPOSE_4_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X14_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X15_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X16_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X17_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X18_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X19_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X1A_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X1B_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X1C_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X1D_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X1E_LSB] = 0;
    cc_values[cc_ids_i::CONTROLLER_0X1F_LSB] = 0;*/

    cc_values[cc_ids_i::SUSTAIN_ONOFF] = 0;
    cc_values[cc_ids_i::PORTAMENTO_ONOFF] = 0;
    cc_values[cc_ids_i::SOSTENUTO_ONOFF] = 0;
    cc_values[cc_ids_i::SOFT_PEDAL_ONOFF] = 0;

    /*cc_values[cc_ids_i::LEGATO_FOOTSWITCH] = 0;
    cc_values[cc_ids_i::HOLD_2] = 0;*/

    //cc_values[cc_ids_i::SOUND_VARIATION] = 0; // sound controller 1
    cc_values[cc_ids_i::TIMBRE_HARMONIC_INTENS] = 64; // sound controller 2

    cc_values[cc_ids_i::RELEASE_TIME] = 64; // sound controller 3
    cc_values[cc_ids_i::ATTACK_TIME] = 64; // sound controller 4
    cc_values[cc_ids_i::LPF_CUTOFF_FREQUENCY] = 64; // sound controller 5
    cc_values[cc_ids_i::DECAY_TIME] = 64; // sound controller 6
    cc_values[cc_ids_i::VIBRATO_RATE] = 64; // sound controller 7
    cc_values[cc_ids_i::VIBRATO_DEPTH] = 64; // sound controller 8
    cc_values[cc_ids_i::VIBRATO_DELAY] = 64; // sound controller 9
    cc_values[cc_ids_i::SUSTAIN_LEVEL] = 64; // sound controller 10

    //cc_values[cc_ids_i::GENERAL_PURPOSE_5] = 0;

    cc_values[cc_ids_i::HPF_CUTOFF_FREQUENCY] = 64; // general purpose 6

    /*cc_values[cc_ids_i::GENERAL_PURPOSE_7] = 0;
    cc_values[cc_ids_i::GENERAL_PURPOSE_8] = 0;*/

    cc_values[cc_ids_i::PORTAMENTO_CONTROL] = 0;

    //cc_values[cc_ids_i::HIGH_RESOLUTION_VELOCITY_PREFIX] = 0;

    cc_values[cc_ids_i::REVERB_SEND_LEVEL] = 20;

    //cc_values[cc_ids_i::TREMOLO_DEPTH] = 0;

    cc_values[cc_ids_i::CHORUS_SEND_LEVEL] = 0;
    cc_values[cc_ids_i::DELAY_SEND_LEVEL] = 0;

    /*cc_values[cc_ids_i::PHASER_DEPTH] = 0;
    cc_values[cc_ids_i::DATA_INCREMENT] = 0;
    cc_values[cc_ids_i::DATA_DECREMENT] = 0;*/

    cc_values[cc_ids_i::NRPN_LSB] = 0;
    cc_values[cc_ids_i::NRPN_MSB] = 0;
    cc_values[cc_ids_i::RPN_LSB] = 0;
    cc_values[cc_ids_i::RPN_MSB] = 0;

    return cc_values;
}