pub mod effects;
pub mod settings;

use crate::soundbank::sf2::SF2;

pub struct Synth {
    pub soundfonts: Vec<SF2>
}