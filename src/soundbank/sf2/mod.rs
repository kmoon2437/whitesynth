/**
 * 사운드폰트 파일(sf2)
 * 오래된 파일 형식이라 그런지 최대한 단순하게 만든 느낌이다
 * 참고문헌: https://www.utsbox.com/?p=2068
 * 그리고 저기서 이어지는 모든 글들
 */
use crate::util;
use std::convert::TryInto;
use std::sync::Arc;
use std::io::{ Read, Seek, SeekFrom };
use anyhow::*;

pub mod gen_ids;
pub mod structure;
pub mod to_wsbk;
use structure::*;

// 에러 처리용 구조체
#[derive(Debug, Clone)]
pub struct SF2Error {
    message: String
}

impl SF2Error {
    fn new(message: &str) -> Self {
        return Self {
            message: message.to_owned()
        };
    }

    fn new_box(message: &str) -> Box<Self> {
        return Box::new(Self::new(message));
    }
}

impl std::error::Error for SF2Error {}

impl std::fmt::Display for SF2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self.message);
    }
}

type SF2SampleData = Arc<Vec<i16>>;

pub struct SF2 {
    // 사운드폰트 파일 정보
    pub info: SF2Info,

    // 사운드폰트 파일에 있는 모든 샘플의 정보
    pub sample_headers: Vec<SF2SampleHeader>,

    // 샘플 데이터 전체를 담고 있음
    sample_data: SF2SampleData,

    // 악기(instrument) 데이터
    pub instruments: Vec<SF2Instrument>,

    // 프리셋(preset) 데이터
    // program 번호와 bank select 번호가 지정되어 있다
    pub presets: Vec<SF2Preset>
}

impl SF2 {
    // 기존의 파일을 사용해 SF2 개체를 생성
    pub fn new<T: Read + Seek>(
        stream: &mut T
    ) -> anyhow::Result<Self> {
        let riff_data = riff::Chunk::read(stream, 0)?;
        let mut smpl_chunk: Option<riff::Chunk> = None;
        //let mut sm24_chunk: Option<riff::Chunk> = None; // 호환용이므로 일단 sm24 청크는 안 쓰는 걸로
        let mut shdr: Vec<u8> = vec![];
        let mut phdr: Vec<u8> = vec![];
        let mut pbag: Vec<u8> = vec![];
        let mut pmod: Vec<u8> = vec![];
        let mut pgen: Vec<u8> = vec![];
        let mut inst: Vec<u8> = vec![];
        let mut ibag: Vec<u8> = vec![];
        let mut imod: Vec<u8> = vec![];
        let mut igen: Vec<u8> = vec![];
        let mut info = SF2Info::new();
        let mut sample_headers: Vec<SF2SampleHeader> = vec![];
        let mut inst_zone = SF2Zone::new();
        let mut inst_bags: Vec<SF2Bag> = vec![];
        let mut instruments: Vec<SF2Instrument> = vec![];
        let mut preset_zone = SF2Zone::new();
        let mut preset_bags: Vec<SF2Bag> = vec![];
        let mut presets: Vec<SF2Preset> = vec![];

        let lists_iter = util::unwrap_result_iter(riff_data.iter(stream))?;
        for child in lists_iter {
            let child_type = child.read_type(stream)?.as_str().to_owned();
            let chunks_iter = util::unwrap_result_iter(child.iter(stream))?;
            for chunk in chunks_iter {
                if child_type == "INFO" {
                    match chunk.id().as_str() {
                        "ifil" => {
                            let contents = chunk.read_contents(stream)?;
                            let major = u16::from_le_bytes(contents[0..=1].try_into()?);
                            let minor = u16::from_le_bytes(contents[2..=3].try_into()?);
                            info.sf_version = [major, minor];
                        }
                        "isng" => info
                            .target_sound_engine
                            .push_str(std::str::from_utf8(&chunk.read_contents(stream)?)?),
                        "INAM" => info
                            .bank_name
                            .push_str(std::str::from_utf8(&chunk.read_contents(stream)?)?),
                        "irom" => info
                            .rom_name
                            .push_str(std::str::from_utf8(&chunk.read_contents(stream)?)?),
                        "iver" => {
                            let contents = chunk.read_contents(stream)?;
                            let major = u16::from_le_bytes(contents[0..=1].try_into()?);
                            let minor = u16::from_le_bytes(contents[2..=3].try_into()?);
                            info.rom_version = [major, minor];
                        }
                        "ICRD" => info
                            .created_date
                            .push_str(std::str::from_utf8(&chunk.read_contents(stream)?)?),
                        "IENG" => info
                            .engineers
                            .push_str(std::str::from_utf8(&chunk.read_contents(stream)?)?),
                        "IPRD" => info
                            .target_hardware
                            .push_str(std::str::from_utf8(&chunk.read_contents(stream)?)?),
                        "ICOP" => info
                            .copyright
                            .push_str(std::str::from_utf8(&chunk.read_contents(stream)?)?),
                        "ICMT" => info
                            .comments
                            .push_str(std::str::from_utf8(&chunk.read_contents(stream)?)?),
                        "ISFT" => info
                            .created_software
                            .push_str(std::str::from_utf8(&chunk.read_contents(stream)?)?),
                        &_ => {}
                    }
                } else if child_type == "sdta" {
                    match chunk.id().as_str() {
                        "smpl" => smpl_chunk = Some(chunk),
                        //"sm24" => sm24_chunk = Some(chunk),
                        &_ => {}
                    }
                } else if child_type == "pdta" {
                    let contents = chunk.read_contents(stream)?;
                    match chunk.id().as_str() {
                        "shdr" => {
                            shdr = contents;
                            if shdr.len() % 46 != 0 {
                                bail!(SF2Error::new("invalid length of chunk 'shdr'"));
                            }
                        }
                        "inst" => {
                            inst = contents;
                            if inst.len() % 22 != 0 {
                                bail!(SF2Error::new("invalid length of chunk 'inst'"));
                            }
                        }
                        "ibag" => {
                            ibag = contents;
                            if ibag.len() % 4 != 0 {
                                bail!(SF2Error::new("invalid length of chunk 'ibag'"));
                            }
                        }
                        "imod" => {
                            imod = contents;
                            if imod.len() % 10 != 0 {
                                bail!(SF2Error::new("invalid length of chunk 'imod'"));
                            }
                        }
                        "igen" => {
                            igen = contents;
                            if igen.len() % 4 != 0 {
                                bail!(SF2Error::new("invalid length of chunk 'igen'"));
                            }
                        }
                        "phdr" => {
                            phdr = contents;
                            if phdr.len() % 38 != 0 {
                                bail!(SF2Error::new("invalid length of chunk 'phdr'"));
                            }
                        }
                        "pbag" => {
                            pbag = contents;
                            if pbag.len() % 4 != 0 {
                                bail!(SF2Error::new("invalid length of chunk 'pbag'"));
                            }
                        }
                        "pmod" => {
                            pmod = contents;
                            if pmod.len() % 10 != 0 {
                                bail!(SF2Error::new("invalid length of chunk 'pmod'"));
                            }
                        }
                        "pgen" => {
                            pgen = contents;
                            if pgen.len() % 4 != 0 {
                                bail!(SF2Error::new("invalid length of chunk 'pgen'"));
                            }
                        }
                        &_ => {}
                    }
                }
            }
        }

        // 샘플 헤더
        let sample_headers_len = shdr.len() / 46;
        for ii in 0..sample_headers_len {
            let i = ii * 46;
            let mut end_of_name = 0;
            for j in 0..=19 {
                if shdr[i + j] == 0 { // \0
                    end_of_name = j - 1;
                    break;
                }
            }
            let sample_header = SF2SampleHeader {
                index: ii,
                name: String::from_utf8(shdr[i..=(i + end_of_name)].try_into()?)?,
                smpl_start: u32::from_le_bytes(shdr[(i + 20)..=(i + 23)].try_into()?),
                smpl_end: u32::from_le_bytes(shdr[(i + 24)..=(i + 27)].try_into()?),
                loop_start: u32::from_le_bytes(shdr[(i + 28)..=(i + 31)].try_into()?),
                loop_end: u32::from_le_bytes(shdr[(i + 32)..=(i + 35)].try_into()?),
                sample_rate: u32::from_le_bytes(shdr[(i + 36)..=(i + 39)].try_into()?),
                base_key: shdr[i + 40],
                correction: i8::from_le_bytes([shdr[i + 41]]),
                linked_sample_index: u16::from_le_bytes(shdr[(i + 42)..=(i + 43)].try_into()?),
                sample_type: u16::from_le_bytes(shdr[(i + 44)..=(i + 45)].try_into()?)
            };
            sample_headers.push(sample_header);
        }

        // instrument generator 처리
        // operator가 무엇이냐에 따라 처리할 파라미터가 달라짐
        // 중요: 여기서 operator가 53번이면 amount는 샘플 헤더 index를 가리킴
        let inst_generators_len = igen.len() / 4;
        for ii in 0..inst_generators_len {
            let i = ii * 4;
            inst_zone.generators.push(SF2Generator {
                operator: u16::from_le_bytes(igen[i..=(i + 1)].try_into()?),
                amount: SF2GeneratorAmount::new(igen[(i + 2)..=(i + 3)].try_into()?)
            });
        }

        // instrument modulator 처리
        let inst_modulators_len = imod.len() / 10;
        for ii in 0..inst_modulators_len {
            let i = ii * 10;
            inst_zone.modulators.push(SF2Modulator {
                src_operator: u16::from_le_bytes(imod[i..=(i + 1)].try_into()?),
                dest_operator: u16::from_le_bytes(imod[(i + 2)..=(i + 3)].try_into()?),
                mod_amount: i16::from_le_bytes(imod[(i + 4)..=(i + 5)].try_into()?),
                amount_src_operator: u16::from_le_bytes(imod[(i + 6)..=(i + 7)].try_into()?),
                mod_trans_operator: u16::from_le_bytes(imod[(i + 8)..=(i + 9)].try_into()?)
            });
        }

        // instrument bag 처리
        // generator와 modulator들은 bag이라는 단위로 묶여 있음
        let inst_bags_len = ibag.len() / 4;
        for ii in 0..inst_bags_len {
            let i = ii * 4;
            inst_bags.push(SF2Bag {
                is_generator: true,
                generator_start: u16::from_le_bytes(ibag[i..=(i + 1)].try_into()?),
                generator_end: 0,
                is_modulator: true,
                modulator_start: u16::from_le_bytes(ibag[(i + 2)..=(i + 3)].try_into()?),
                modulator_end: 0
            });
        }

        // 각각의 bag에 generator와 modulator가 있는지 검사
        for i in 0..inst_bags_len {
            inst_bags[i].generator_end = if i == inst_bags_len - 1 {
                (inst_generators_len - 1) as u16
            } else if inst_bags[i + 1].generator_start > inst_bags[i].generator_start {
                inst_bags[i + 1].generator_start - 1
            } else {
                inst_bags[i].is_generator = false;
                0
            };

            inst_bags[i].modulator_end = if i == inst_bags_len - 1 {
                (inst_modulators_len - 1) as u16
            } else if inst_bags[i + 1].modulator_start > inst_bags[i].modulator_start {
                inst_bags[i + 1].modulator_start - 1
            } else {
                inst_bags[i].is_modulator = false;
                0
            };
        }

        // instrument 처리
        let instruments_len = inst.len() / 22;
        for ii in 0..instruments_len {
            let i = ii * 22;
            let mut end_of_name = 0;
            for j in 0..=19 {
                if inst[i + j] == 0 {
                    end_of_name = j - 1;
                    break;
                }
            }
            let instrument = SF2Instrument {
                name: String::from_utf8(inst[i..=(i + end_of_name)].try_into()?)?,
                ibag_index: u16::from_le_bytes(inst[(i + 20)..=(i + 21)].try_into()?),
                zones: vec![] // 이 다음 반복문에서 채울 거임
            };
            instruments.push(instrument);
        }

        // instrument별로 bag을 풀어서 zone 분류
        for i in 0..instruments_len {
            let mut is_ibag = true;
            let ibag_start = instruments[i].ibag_index as usize;
            let ibag_end = if i == instruments_len - 1 {
                inst_bags_len - 1
            } else if instruments[i + 1].ibag_index > ibag_start as u16 {
                (instruments[i + 1].ibag_index - 1) as usize
            } else {
                is_ibag = false;
                0
            };

            if !is_ibag {
                continue;
            }

            for j in ibag_start..=ibag_end {
                let mut zone = SF2Zone::new();
                if inst_bags[j].is_generator {
                    for k in (inst_bags[j].generator_start as usize)
                        ..=(inst_bags[j].generator_end as usize)
                    {
                        zone.generators.push(inst_zone.generators[k]);
                        if inst_zone.generators[k].operator == gen_ids::SAMPLE_ID {
                            zone.target_sample_index = Some(inst_zone.generators[k].amount.get_u16() as usize);
                        }
                    }
                    zone.update_generators_lookup(); // generator를 빠르게 찾을 수 있도록 함
                }

                if inst_bags[j].is_modulator {
                    for k in (inst_bags[j].modulator_start as usize)
                        ..=(inst_bags[j].modulator_end as usize)
                    {
                        zone.modulators.push(inst_zone.modulators[k]);
                    }
                }
                instruments[i].zones.push(Arc::new(zone));
            }
        }

        // preset generator 처리
        // 중요: 여기서 operator가 41번이면 amount는 instrument 헤더 index를 가리킴
        let preset_generators_len = pgen.len() / 4;
        for ii in 0..preset_generators_len {
            let i = ii * 4;
            preset_zone.generators.push(SF2Generator {
                operator: u16::from_le_bytes(pgen[i..=(i + 1)].try_into()?),
                amount: SF2GeneratorAmount::new(pgen[(i + 2)..=(i + 3)].try_into()?)
            });
        }

        // preset modulator 처리
        let preset_modulators_len = pmod.len() / 10;
        for ii in 0..preset_modulators_len {
            let i = ii * 10;
            preset_zone.modulators.push(SF2Modulator {
                src_operator: u16::from_le_bytes(pmod[i..=(i + 1)].try_into()?),
                dest_operator: u16::from_le_bytes(pmod[(i + 2)..=(i + 3)].try_into()?),
                mod_amount: i16::from_le_bytes(pmod[(i + 4)..=(i + 5)].try_into()?),
                amount_src_operator: u16::from_le_bytes(pmod[(i + 6)..=(i + 7)].try_into()?),
                mod_trans_operator: u16::from_le_bytes(pmod[(i + 8)..=(i + 9)].try_into()?)
            });
        }

        // preset bag 처리
        // instrument 쪽에서 하는 것과 대부분 같음
        let preset_bags_len = pbag.len() / 4;
        for ii in 0..preset_bags_len {
            let i = ii * 4;
            preset_bags.push(SF2Bag {
                is_generator: true,
                generator_start: u16::from_le_bytes(pbag[i..=(i + 1)].try_into()?),
                generator_end: 0,
                is_modulator: true,
                modulator_start: u16::from_le_bytes(pbag[(i + 2)..=(i + 3)].try_into()?),
                modulator_end: 0
            });
        }

        for i in 0..preset_bags_len {
            preset_bags[i].generator_end = if i == preset_bags_len - 1 {
                (preset_generators_len - 1) as u16
            } else if preset_bags[i + 1].generator_start > preset_bags[i].generator_start {
                preset_bags[i + 1].generator_start - 1
            } else {
                preset_bags[i].is_generator = false;
                0
            };

            preset_bags[i].modulator_end = if i == preset_bags_len - 1 {
                (preset_modulators_len - 1) as u16
            } else if preset_bags[i + 1].modulator_start > preset_bags[i].modulator_start {
                preset_bags[i + 1].modulator_start - 1
            } else {
                preset_bags[i].is_modulator = false;
                0
            };
        }

        // preset 처리
        let presets_len = phdr.len() / 38;
        for ii in 0..presets_len {
            let i = ii * 38;
            let mut end_of_name = 0;
            for j in 0..=19 {
                if phdr[i + j] == 0 {
                    end_of_name = j - 1;
                    break;
                }
            }
            let preset = SF2Preset {
                name: String::from_utf8(phdr[i..=(i + end_of_name)].try_into()?)?,
                program_no: u16::from_le_bytes(phdr[(i + 20)..=(i + 21)].try_into()?),
                bank: u16::from_le_bytes(phdr[(i + 22)..=(i + 23)].try_into()?),
                pbag_index: u16::from_le_bytes(phdr[(i + 24)..=(i + 25)].try_into()?),
                zones: vec![],
                library: u32::from_le_bytes(phdr[(i + 26)..=(i + 29)].try_into()?),
                genre: u32::from_le_bytes(phdr[(i + 30)..=(i + 33)].try_into()?),
                morph: u32::from_le_bytes(phdr[(i + 34)..=(i + 37)].try_into()?)
            };
            presets.push(preset);
        }

        for i in 0..presets_len {
            let mut is_pbag = true;
            let pbag_start = presets[i].pbag_index as usize;
            let pbag_end = if i == presets_len - 1 {
                preset_bags_len - 1
            } else if presets[i + 1].pbag_index > pbag_start as u16 {
                (presets[i + 1].pbag_index - 1) as usize
            } else {
                is_pbag = false;
                0
            };

            if !is_pbag {
                continue;
            };

            for j in pbag_start..=pbag_end {
                let mut zone = SF2Zone::new();
                if preset_bags[j].is_generator {
                    for k in (preset_bags[j].generator_start as usize)
                        ..=(preset_bags[j].generator_end as usize)
                    {
                        zone.generators.push(preset_zone.generators[k]);
                        if preset_zone.generators[k].operator == gen_ids::INSTRUMENT {
                            zone.target_instrument_index = Some(preset_zone.generators[k].amount.get_u16() as usize);
                        }
                    }
                    zone.update_generators_lookup(); // generator를 빠르게 찾을 수 있도록 함
                }

                if preset_bags[j].is_modulator {
                    for k in (preset_bags[j].modulator_start as usize)
                        ..=(preset_bags[j].modulator_end as usize)
                    {
                        zone.modulators.push(preset_zone.modulators[k]);
                    }
                }
                presets[i].zones.push(Arc::new(zone));
            }
        }

        // 샘플 처리
        // 참조 소스 코드: https://github.com/sinshu/rustysynth/blob/main/rustysynth/src/binary_reader.rs
        let smpl_chunk_info = match smpl_chunk {
            Some(chunk) => chunk,
            None => bail!(SF2Error::new("No smpl chunk"))
        };
        let sample_len = smpl_chunk_info.len() as usize / 2;
        let mut sample_data: Vec<i16> = vec![0; sample_len];
        let data = unsafe {
            // 속도를 위해 안전성을 희생하는 부분
            // 이 부분 때문에 문제가 생기면 다시 아래 코드로 돌아갈 예정
            let ptr = sample_data.as_mut_ptr() as *mut u8;
            std::slice::from_raw_parts_mut(ptr, smpl_chunk_info.len() as usize)
        };
        stream.seek(SeekFrom::Start(smpl_chunk_info.offset() + 8))?;
        stream.read_exact(data)?;

        // 기존 샘플 처리 코드(모조리 i32형으로 변환)
        // 여기서 시간이 좀 많이 걸려서 위에 있는 처리 코드로 바꿨음
        /*let smpl_contents = match smpl_chunk {
            Some(chunk) => chunk.read_contents(stream)?,
            None => bail!(SF2Error::new("No smpl chunk"))
        };
        let sm24_contents = match sm24_chunk {
            Some(chunk) => Some(chunk.read_contents(stream)?),
            None => None
        };
        let sample_len = smpl_contents.len() / 2;
        let mut sample_data: Vec<i32> = vec![];
        match sm24_contents {
            Some(sm24_contents) => {
                for i in 0..sample_len {
                    sample_data.push(i32::from_le_bytes([
                        0, sm24_contents[i],
                        smpl_contents[i * 2],
                        smpl_contents[i * 2 + 1]
                    ]));
                }
            },
            None => {
                for i in 0..sample_len {
                    sample_data.push(i32::from_le_bytes([
                        0, 0,
                        smpl_contents[i * 2],
                        smpl_contents[i * 2 + 1]
                    ]));
                }
            }
        }*/

        return Ok(Self {
            info: info,
            sample_headers: sample_headers,
            sample_data: Arc::new(sample_data),
            instruments: instruments,
            presets: presets
        });
    }
}