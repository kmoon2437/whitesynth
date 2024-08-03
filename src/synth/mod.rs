pub mod envelope;
pub mod lfo;
pub mod settings;
pub mod vendors;
pub mod param_smoother;

use crate::soundbank::wsbk::WSBK;
use vendors::VendorId;
use settings::SynthCreateSettings;

#[derive(PartialEq, Eq, Debug)]
pub struct FXType(pub u8, pub u8, pub u8);

pub struct Synth {
    soundbanks: Vec<WSBK>,
    buffer_left: Vec<u8>,
    buffer_right: Vec<u8>
}

#[allow(unused)] // 모든 기능이 완성될 즈음에 제거 예정
impl Synth {
    pub fn new(settings: SynthCreateSettings) -> Self {
        return Self {
            soundbanks: vec![],
            buffer_left: vec![],
            buffer_right: vec![]
        };
    }

    // midi 기본기능
    pub fn handle_midi_message(&mut self, msg: &[u8]) {
        let msg_category = msg[0] >> 4;
        let channel = msg[0] - (msg_category << 4);
        match msg_category {
            0x8 => self.note_off(channel, msg[1] as i32, msg[2] as i32),
            0x9 => self.note_on(channel, msg[1] as i32, msg[2] as i32),
            0xa => self.note_aftertouch(channel, msg[1] as i32, msg[2] as i32),
            0xb => self.control_change(channel, msg[1], msg[2]),
            0xc => self.program_change(channel, msg[1] as i32),
            0xd => self.channel_aftertouch(channel, msg[1] as i32),
            0xe => self.pitch_bend(channel, (msg[1] as i32) + ((msg[2] as i32) << 7)),
            0xf => {
                if channel == 0x0 {
                    self.handle_sysex(msg);
                }
            },
            _ => {}
        }
    }

    // sysex 메세지를 해석해 그에 해당하는 기능 수행
    pub fn handle_sysex(&mut self, msg: &[u8]) {
        let vendor_id: VendorId = if msg[1] == 0x00 {
            VendorId::Extended(msg[2], msg[3])
        } else {
            VendorId::Standard(msg[1])
        };

        match vendor_id {
            vendors::STD_NON_REALTIME => self.handle_gm_non_realtime_sysex(msg),
            vendors::STD_REALTIME => self.handle_gm_realtime_sysex(msg),
            vendors::ROLAND => self.handle_gs_sysex(msg),
            vendors::YJ => self.handle_wsn_sysex(msg),
            _ => log::error!("Invalid sysex vendor id")
        }
    }

    pub fn note_on(&mut self, channel_no: u8, note: i32, velocity: i32) {
        if velocity <= 0 { return self.note_off(channel_no, note, velocity); }
        
    //
    }

    pub fn note_off(&mut self, channel_no: u8, note: i32, _velocity: i32) {
        //
    }

    pub fn note_aftertouch(&mut self, channel_no: u8, note: i32, pressure: i32) {
        //
    }

    pub fn control_change(&mut self, channel_no: u8, cc: u8, val: u8) {
        //
    }

    pub fn program_change(&mut self, channel_no: u8, program_no: i32) {
        //
    }

    pub fn channel_aftertouch(&mut self, channel_no: u8, pressure: i32) {
        //
    }

    // offset => 0 - 16383
    pub fn pitch_bend(&mut self, channel_no: u8, offset: i32) {
        //
    }

    // 어떤 reset 메세지가 들어와도 공통으로 수행하는 reset
    pub fn system_reset(&mut self) {}

    /**
     * 표준 sysex로 제어하는 기능
     */
    pub fn handle_gm_non_realtime_sysex(&mut self, msg: &[u8]) {
        //
    }

    pub fn handle_gm_realtime_sysex(&mut self, msg: &[u8]) {
        //
    }

    pub fn gm_reset(&mut self) {}

    pub fn set_master_volume(&mut self, volume: i32) {
        //
    }

    pub fn set_master_fine_tuning(&mut self, cent: f64) {
        //
    }

    pub fn set_master_coarse_tuning(&mut self, key: i32) {
        //
    }

    /**
     * gs 확장 sysex로 제어하는 기능
     */
    pub fn handle_gs_sysex(&mut self, msg: &[u8]) {
        //
    }

    pub fn gs_reset(&mut self) {}

    /**
     * 자체 확장 sysex로 제어하는 기능
     */
    pub fn handle_wsn_sysex(&mut self, msg: &[u8]) {
        //
    }

    pub fn wsn_reset(&mut self) {}

    // -1.0(왼쪽) - 0.0(가운데) - 1.0(오른쪽)
    pub fn set_master_pan(&mut self, pan: f64) {
        //
    }

    // multi effect(이하 mfx) 관련 기능
    pub fn set_mfx_type(&mut self, channel_no: u8, unit: u8, mfx_type: FXType) {
        //
    }

    pub fn set_mfx_parameter(&mut self, channel_no: u8, unit: u8, param_no: i32, val: i32) {
        //
    }

    // variation effect 관련 기능
    pub fn set_variation_fx_type(&mut self, unit: u8, vfx_type: FXType) {
        //
    }

    pub fn set_variation_fx_parameter(&mut self, unit: u8, param_no: i32, val: i32) {
        //
    }

    pub fn render(&mut self, left: &mut [f64], right: &mut [f64]) {}

    pub fn render_as_one_array(&mut self, left: &mut [f64], right: &mut [f64]) {}

    fn render_buffer(&mut self) {}
}