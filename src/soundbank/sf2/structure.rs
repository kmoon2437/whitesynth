use std::sync::Arc;
use std::collections::HashMap;

pub struct SF2Info {
    pub sf_version: [u16; 2], // ifil
    pub target_sound_engine: String, // isng
    pub bank_name: String, // INAM
    pub rom_name: String, // irom
    pub rom_version: [u16; 2], // iver
    pub created_date: String, // ICRD
    pub engineers: String, // IENG
    pub target_hardware: String, // IPRD
    pub copyright: String, // ICOP
    pub comments: String, // ICMT
    pub created_software: String // ISFT
}

impl SF2Info {
    // 텅 빈 SF2Info 개체를 생성
    // 몇몇 정보는 무시해도 되기 때문에 이렇게 기본값으로 설정해둠
    pub fn new() -> Self {
        return Self {
            sf_version: [0, 0],
            target_sound_engine: String::new(),
            bank_name: String::new(),
            rom_name: String::new(),
            rom_version: [0, 0],
            created_date: String::new(),
            engineers: String::new(),
            target_hardware: String::new(),
            copyright: String::new(),
            comments: String::new(),
            created_software: String::new()
        };
    }
}

pub struct SF2SampleHeader {
    pub index: usize,
    pub name: String,
    pub smpl_start: u32,
    pub smpl_end: u32,
    pub loop_start: u32,
    pub loop_end: u32,
    pub sample_rate: u32,
    pub base_key: u8,
    pub correction: i8,
    pub linked_sample_index: u16,
    pub sample_type: u16
}

pub struct SF2Bag {
    pub is_generator: bool,
    pub generator_start: u16,
    pub generator_end: u16,
    pub is_modulator: bool,
    pub modulator_start: u16,
    pub modulator_end: u16
}

#[derive(Copy, Clone)]
pub struct SF2Modulator {
    pub src_operator: u16,
    pub dest_operator: u16,
    pub mod_amount: i16, // 얘는 무조건 i16임
    pub amount_src_operator: u16,
    pub mod_trans_operator: u16
}

// i16형의 값이 쓰일 수도 있고 u16형의 값이 쓰일 수도 있음
// 이러면 값 하나가 총 6바이트를 차지하게 되는데, 현대의 컴퓨터라면 문제 없을 거라 생각함
#[derive(Copy, Clone, Debug)]
pub struct SF2GeneratorAmount {
    i16_amount: i16,
    u16_amount: u16,
    u8_array_amount: [u8; 2]
}

impl SF2GeneratorAmount {
    pub fn new(u8_array_amount: [u8; 2]) -> Self {
        return Self {
            i16_amount: i16::from_le_bytes(u8_array_amount),
            u16_amount: u16::from_le_bytes(u8_array_amount),
            u8_array_amount: u8_array_amount
        };
    }

    #[inline]
    pub fn get_i16(&self) -> i16 {
        return self.i16_amount;
    }

    #[inline]
    pub fn get_u16(&self) -> u16 {
        return self.u16_amount;
    }

    #[inline]
    pub fn get_u8_array(&self) -> [u8; 2] {
        return self.u8_array_amount;
    }

    pub fn set_i16(&mut self, val: i16) {
        self.i16_amount = val;
        self.u8_array_amount = val.to_le_bytes();
        self.u16_amount = u16::from_le_bytes(self.u8_array_amount);
    }

    pub fn set_u16(&mut self, val: u16) {
        self.u16_amount = val;
        self.u8_array_amount = val.to_le_bytes();
        self.i16_amount = i16::from_le_bytes(self.u8_array_amount);
    }

    pub fn set_u8_array(&mut self, val: [u8; 2]) {
        self.u8_array_amount = val;
        self.i16_amount = i16::from_le_bytes(self.u8_array_amount);
        self.u16_amount = u16::from_le_bytes(self.u8_array_amount);
    }
}

#[derive(Copy, Clone)]
pub struct SF2Generator {
    pub operator: u16,
    pub amount: SF2GeneratorAmount
}

pub struct SF2Zone {
    pub modulators: Vec<SF2Modulator>,
    pub generators: Vec<SF2Generator>,
    generators_lookup: HashMap<u16, SF2GeneratorAmount>,
    pub target_sample_index: Option<usize>,
    pub target_instrument_index: Option<usize>
}

impl SF2Zone {
    pub fn new() -> Self {
        return Self {
            modulators: vec![],
            generators: vec![],
            generators_lookup: HashMap::new(),
            target_sample_index: None,
            target_instrument_index: None
        };
    }

    pub fn update_generators_lookup(&mut self) {
        for gen in self.generators.iter() {
            self.generators_lookup.insert(gen.operator, gen.amount);
        }
    }

    pub fn get_gen(&self, gen_id: u16) -> Option<SF2GeneratorAmount> {
        return match self.generators_lookup.get(&gen_id) {
            Some(val) => Some(*val), None => None
        };
    }
}

pub struct SF2Instrument {
    pub name: String,
    pub ibag_index: u16,
    pub zones: Vec<Arc<SF2Zone>> // 첫 번째 zone은 instrument 전체에 적용되는 global zone이라 sample index가 없으니 주의!
}

pub struct SF2Preset {
    pub name: String,
    pub program_no: u16,
    pub bank: u16,
    pub pbag_index: u16,
    pub zones: Vec<Arc<SF2Zone>>, // 첫 번째 zone은 preset 전체에 적용되는 global zone이라 instrument index가 없으니 주의!
    pub library: u32,
    pub genre: u32,
    pub morph: u32
}