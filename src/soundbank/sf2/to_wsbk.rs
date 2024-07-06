use crate::soundbank::wsbk;
//use crate::soundbank::sf2;

impl super::SF2 {
    pub fn to_wsbk(&self) -> wsbk::WSBK {
        let wsbk_bank = wsbk::WSBK::new();

        for sf2_smpl_header in self.sample_headers.iter() {
            let mut wsbk_smpl = wsbk::Sample::new(&sf2_smpl_header.name);
            wsbk_smpl.bit_depth = 16;
            wsbk_smpl.sample_rate = sf2_smpl_header.sample_rate;
            wsbk_smpl.loop_start = sf2_smpl_header.loop_start - sf2_smpl_header.smpl_start;
            wsbk_smpl.loop_end = sf2_smpl_header.loop_end - sf2_smpl_header.smpl_start;
        }

        return wsbk_bank;
    }
}