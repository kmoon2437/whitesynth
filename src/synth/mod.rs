pub mod settings;
pub mod effects;

use crate::soundbank::sf2::SF2;

pub struct Synth{
    pub soundfonts:Vec<SF2>
}