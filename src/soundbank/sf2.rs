/**
 * 사운드폰트 파일(sf2)
 * 오래된 파일 형식이라 그런지 최대한 단순하게 만든 느낌이다
 * 참고문헌: https://www.utsbox.com/?p=2068
 * 그리고 저기서 이어지는 모든 글들
 */
use std::convert::TryInto;

pub struct SF2Info{
    pub sf_version:[u16;2], // ifil
    pub target_sound_engine:String, // isng
    pub bank_name:String, // INAM
    pub rom_name:String, // irom
    pub rom_version:[u16;2], // iver
    pub created_date:String, // ICRD
    pub engineers:String, // IENG
    pub target_hardware:String, // IPRD
    pub copyright:String, // ICOP
    pub comments:String, // ICMT
    pub created_software:String // ISFT
}

impl SF2Info{
    // 텅 빈 SF2Info 개체를 생성
    // 몇몇 정보는 무시해도 되기 때문에 이렇게 기본값으로 설정해둠
    fn new() -> Self{
        return Self{
            sf_version:[0,0],
            target_sound_engine:String::new(),
            bank_name:String::new(),
            rom_name:String::new(),
            rom_version:[0,0],
            created_date:String::new(),
            engineers:String::new(),
            target_hardware:String::new(),
            copyright:String::new(),
            comments:String::new(),
            created_software:String::new()
        };
    }
}

pub struct SF2SampleHeader{
    pub name:String,
    pub smpl_start:u32,
    pub smpl_end:u32,
    pub loop_start:u32,
    pub loop_end:u32,
    pub sample_rate:u32,
    pub base_key:u8,
    pub correction:i8,
    pub linked_sample_index:u16,
    pub sample_type:u16
}

pub struct SF2Bag{
    pub generator_start:u16,
    pub generator_end:u16,
    pub modulator_start:u16,
    pub modulator_end:u16
}

#[derive(Copy,Clone)]
pub struct SF2Modulator{
    pub src_operator:u16,
    pub dest_operator:u16,
    pub mod_amount:i16,
    pub amount_src_operator:u16,
    pub mod_trans_operator:u16
}

#[derive(Copy,Clone)]
pub struct SF2Generator{
    pub operator:u16,
    pub amount:i16
}

pub struct SF2Zone{
    pub modulators:Vec<SF2Modulator>,
    pub generators:Vec<SF2Generator>
}

impl SF2Zone{
    fn new() -> Self{
        return Self{
            modulators:vec![],
            generators:vec![]
        };
    }
}

pub struct SF2Instrument{
    pub name:String,
    pub ibag_index:u16,
    pub zones:Vec<SF2Zone>
}

pub struct SF2Preset{
    pub name:String,
    pub program_no:u16,
    pub bank:u16,
    pub pbag_index:u16,
    pub zones:Vec<SF2Zone>,
    pub library:u32,
    pub genre:u32,
    pub morph:u32
}

// 에러 처리용 구조체
#[derive(Debug,Clone)]
pub struct SF2Error{
    message:String
}

impl SF2Error{
    fn new(message:&str) -> Self{
        return Self{ message:message.to_string() };
    }
}

impl std::error::Error for SF2Error{}

impl std::fmt::Display for SF2Error{
    fn fmt(&self,f:&mut std::fmt::Formatter) -> std::fmt::Result{
        return write!(f,"{}",self.message);
    }
}

pub struct SF2{
    pub info:SF2Info,
    pub sample_headers:Vec<SF2SampleHeader>,
    pub sample_data:Vec<u8>,
    pub sm24_data:Vec<u8>,
    pub instruments:Vec<SF2Instrument>,
    pub presets:Vec<SF2Preset>
}

impl SF2{
    pub fn new<T:std::io::Read + std::io::Seek>(data:&mut T) -> Result<Self,Box<dyn std::error::Error>>{
        let riff_data = riff::Chunk::read(data,0)?;
        let mut sample_data:Vec<u8> = vec![];
        let mut sm24_data:Vec<u8> = vec![];
        let mut shdr:Vec<u8> = vec![];
        let mut phdr:Vec<u8> = vec![];
        let mut pbag:Vec<u8> = vec![];
        let mut pmod:Vec<u8> = vec![];
        let mut pgen:Vec<u8> = vec![];
        let mut inst:Vec<u8> = vec![];
        let mut ibag:Vec<u8> = vec![];
        let mut imod:Vec<u8> = vec![];
        let mut igen:Vec<u8> = vec![];
        let mut info = SF2Info::new();
        let mut sample_headers:Vec<SF2SampleHeader> = vec![];
        let mut inst_zone = SF2Zone::new();
        let mut inst_bags:Vec<SF2Bag> = vec![];
        let mut instruments:Vec<SF2Instrument> = vec![];
        let mut preset_zone = SF2Zone::new();
        let mut preset_bags:Vec<SF2Bag> = vec![];
        let mut presets:Vec<SF2Preset> = vec![];

        let lists:Vec<_> = riff_data.iter(data).map(|a| a.unwrap()).collect();
        for child in lists.iter() {
            let child_type = child.read_type(data)?.as_str().to_string();
            let chunks:Vec<_> = child.iter(data).map(|a| a.unwrap()).collect();
            for chunk in chunks.iter() {
                if child_type == "INFO" {
                    match chunk.id().as_str() {
                        "ifil" => {
                            let contents = &chunk.read_contents(data)?;
                            let major = u16::from_le_bytes(contents[0..=1].try_into()?);
                            let minor = u16::from_le_bytes(contents[2..=3].try_into()?);
                            info.sf_version = [major,minor];
                        },
                        "isng" => info.target_sound_engine.push_str(std::str::from_utf8(&chunk.read_contents(data)?)?),
                        "INAM" => info.bank_name.push_str(std::str::from_utf8(&chunk.read_contents(data)?)?),
                        "irom" => info.rom_name.push_str(std::str::from_utf8(&chunk.read_contents(data)?)?),
                        "iver" => {
                            let contents = &chunk.read_contents(data)?;
                            let major = u16::from_le_bytes(contents[0..=1].try_into()?);
                            let minor = u16::from_le_bytes(contents[2..=3].try_into()?);
                            info.rom_version = [major,minor];
                        },
                        "ICRD" => info.created_date.push_str(std::str::from_utf8(&chunk.read_contents(data)?)?),
                        "IENG" => info.engineers.push_str(std::str::from_utf8(&chunk.read_contents(data)?)?),
                        "IPRD" => info.target_hardware.push_str(std::str::from_utf8(&chunk.read_contents(data)?)?),
                        "ICOP" => info.copyright.push_str(std::str::from_utf8(&chunk.read_contents(data)?)?),
                        "ICMT" => info.comments.push_str(std::str::from_utf8(&chunk.read_contents(data)?)?),
                        "ISFT" => info.created_software.push_str(std::str::from_utf8(&chunk.read_contents(data)?)?),
                        &_ => {}
                    }
                }else if child_type == "sdta" {
                    let contents = chunk.read_contents(data)?;
                    match chunk.id().as_str() {
                        "smpl" => sample_data = contents,
                        "sm24" => sm24_data = contents,
                        &_ => {}
                    }
                }else if child_type == "pdta" {
                    let contents = chunk.read_contents(data)?;
                    match chunk.id().as_str() {
                        "shdr" => {
                            shdr = contents;
                            if shdr.len() % 46 != 0 {
                                return Err(Box::new(SF2Error::new("invalid length of chunk 'shdr'")));
                            }
                        },
                        "inst" => {
                            inst = contents;
                            if inst.len() % 22 != 0 {
                                return Err(Box::new(SF2Error::new("invalid length of chunk 'inst'")));
                            }
                        },
                        "ibag" => {
                            ibag = contents;
                            if ibag.len() % 4 != 0 {
                                return Err(Box::new(SF2Error::new("invalid length of chunk 'ibag'")));
                            }
                        },
                        "imod" => {
                            imod = contents;
                            if imod.len() % 10 != 0 {
                                return Err(Box::new(SF2Error::new("invalid length of chunk 'imod'")));
                            }
                        },
                        "igen" => {
                            igen = contents;
                            if igen.len() % 4 != 0 {
                                return Err(Box::new(SF2Error::new("invalid length of chunk 'igen'")));
                            }
                        },
                        "phdr" => {
                            phdr = contents;
                            if phdr.len() % 38 != 0 {
                                return Err(Box::new(SF2Error::new("invalid length of chunk 'phdr'")));
                            }
                        },
                        "pbag" => {
                            pbag = contents;
                            if pbag.len() % 4 != 0 {
                                return Err(Box::new(SF2Error::new("invalid length of chunk 'pbag'")));
                            }
                        },
                        "pmod" => {
                            pmod = contents;
                            if pmod.len() % 10 != 0 {
                                return Err(Box::new(SF2Error::new("invalid length of chunk 'pmod'")));
                            }
                        },
                        "pgen" => {
                            pgen = contents;
                            if pgen.len() % 4 != 0 {
                                return Err(Box::new(SF2Error::new("invalid length of chunk 'pgen'")));
                            }
                        },
                        &_ => {}
                    }
                }
            }
        }

        let sample_headers_len = shdr.len() / 46;
        for ii in 0..sample_headers_len {
            let i = ii * 46;
            let mut end_of_name = 0;
            for j in 0..=19 {
                if shdr[i+j] == 0 {
                    end_of_name = j-1;
                    break;
                }
            }
            let sample_header = SF2SampleHeader{
                name:String::from_utf8(shdr[i..=(i+end_of_name)].try_into()?)?,
                smpl_start:u32::from_le_bytes(shdr[(i+20)..=(i+23)].try_into()?),
                smpl_end:u32::from_le_bytes(shdr[(i+24)..=(i+27)].try_into()?),
                loop_start:u32::from_le_bytes(shdr[(i+28)..=(i+31)].try_into()?),
                loop_end:u32::from_le_bytes(shdr[(i+32)..=(i+35)].try_into()?),
                sample_rate:u32::from_le_bytes(shdr[(i+36)..=(i+39)].try_into()?),
                base_key:shdr[i+40],
                correction:i8::from_le_bytes([shdr[i+41]]),
                linked_sample_index:u16::from_le_bytes(shdr[(i+42)..=(i+43)].try_into()?),
                sample_type:u16::from_le_bytes(shdr[(i+44)..=(i+45)].try_into()?)
            };
            sample_headers.push(sample_header);
        }

        let inst_generators_len = igen.len() / 4;
        for ii in 0..inst_generators_len {
            let i = ii * 4;
            inst_zone.generators.push(SF2Generator{
                operator:u16::from_le_bytes(igen[i..=(i+1)].try_into()?),
                amount:i16::from_le_bytes(igen[(i+2)..=(i+3)].try_into()?),
            });
        }

        let inst_modulators_len = imod.len() / 10;
        for ii in 0..inst_modulators_len {
            let i = ii * 10;
            inst_zone.modulators.push(SF2Modulator{
                src_operator:u16::from_le_bytes(imod[i..=(i+1)].try_into()?),
                dest_operator:u16::from_le_bytes(imod[(i+2)..=(i+3)].try_into()?),
                mod_amount:i16::from_le_bytes(imod[(i+4)..=(i+5)].try_into()?),
                amount_src_operator:u16::from_le_bytes(imod[(i+6)..=(i+7)].try_into()?),
                mod_trans_operator:u16::from_le_bytes(imod[(i+8)..=(i+9)].try_into()?),
            });
        }

        let inst_bags_len = ibag.len() / 4;
        for ii in 0..inst_bags_len {
            let i = ii * 4;
            inst_bags.push(SF2Bag{
                generator_start:u16::from_le_bytes(ibag[i..=(i+1)].try_into()?),
                generator_end:0,
                modulator_start:u16::from_le_bytes(ibag[(i+2)..=(i+3)].try_into()?),
                modulator_end:0
            });
        }

        for i in 0..inst_bags_len {
            inst_bags[i].generator_end = if i == inst_bags_len { inst_generators_len as u16 }else{ inst_bags[i+1].generator_start-1 };
            inst_bags[i].modulator_end = if i == inst_bags_len { inst_modulators_len as u16 }else{ inst_bags[i+1].modulator_start-1 };
        }

        let instruments_len = inst.len() / 22;
        for ii in 0..instruments_len {
            let i = ii * 22;
            let mut end_of_name = 0;
            for j in 0..=19 {
                if inst[i+j] == 0 {
                    end_of_name = j-1;
                    break;
                }
            }
            let instrument = SF2Instrument{
                name:String::from_utf8(inst[i..=(i+end_of_name)].try_into()?)?,
                ibag_index:u16::from_le_bytes(phdr[(i+20)..=(i+21)].try_into()?),
                zones:vec![]
            };
            instruments.push(instrument);
        }

        for i in 0..instruments_len {
            let ibag_start = instruments[i].ibag_index as usize;
            let ibag_end = if i == instruments_len { inst_bags_len }else{ (instruments[i+1].ibag_index-1) as usize };
            for j in ibag_start..=ibag_end {
                let mut zone = SF2Zone::new();
                for k in (inst_bags[j].generator_start as usize)..=(inst_bags[j].generator_end as usize) {
                    zone.generators.push(inst_zone.generators[k]);
                }
                for k in (inst_bags[j].modulator_start as usize)..=(inst_bags[j].modulator_end as usize) {
                    zone.modulators.push(inst_zone.modulators[k]);
                }
                instruments[i].zones.push(zone);
            }
        }

        let preset_generators_len = pgen.len() / 4;
        for ii in 0..preset_generators_len {
            let i = ii * 4;
            preset_zone.generators.push(SF2Generator{
                operator:u16::from_le_bytes(pgen[i..=(i+1)].try_into()?),
                amount:i16::from_le_bytes(pgen[(i+2)..=(i+3)].try_into()?),
            });
        }

        let preset_modulators_len = pmod.len() / 10;
        for ii in 0..preset_modulators_len {
            let i = ii * 10;
            preset_zone.modulators.push(SF2Modulator{
                src_operator:u16::from_le_bytes(pmod[i..=(i+1)].try_into()?),
                dest_operator:u16::from_le_bytes(pmod[(i+2)..=(i+3)].try_into()?),
                mod_amount:i16::from_le_bytes(pmod[(i+4)..=(i+5)].try_into()?),
                amount_src_operator:u16::from_le_bytes(pmod[(i+6)..=(i+7)].try_into()?),
                mod_trans_operator:u16::from_le_bytes(pmod[(i+8)..=(i+9)].try_into()?),
            });
        }

        let preset_bags_len = pbag.len() / 4;
        for ii in 0..preset_bags_len {
            let i = ii * 4;
            preset_bags.push(SF2Bag{
                generator_start:u16::from_le_bytes(pbag[i..=(i+1)].try_into()?),
                generator_end:0,
                modulator_start:u16::from_le_bytes(pbag[(i+2)..=(i+3)].try_into()?),
                modulator_end:0
            });
        }

        for i in 0..preset_bags_len {
            preset_bags[i].generator_end = if i == preset_bags_len { preset_generators_len as u16 }else{ preset_bags[i+1].generator_start-1 };
            preset_bags[i].modulator_end = if i == preset_bags_len { preset_modulators_len as u16 }else{ preset_bags[i+1].modulator_start-1 };
        }

        let presets_len = phdr.len() / 38;
        for ii in 0..presets_len {
            let i = ii * 38;
            let mut end_of_name = 0;
            for j in 0..=19 {
                if phdr[i+j] == 0 {
                    end_of_name = j-1;
                    break;
                }
            }
            let preset = SF2Preset{
                name:String::from_utf8(phdr[i..=(i+end_of_name)].try_into()?)?,
                program_no:u16::from_le_bytes(phdr[(i+20)..=(i+21)].try_into()?),
                bank:u16::from_le_bytes(phdr[(i+22)..=(i+23)].try_into()?),
                pbag_index:u16::from_le_bytes(phdr[(i+24)..=(i+25)].try_into()?),
                zones:vec![],
                library:u32::from_le_bytes(phdr[(i+26)..=(i+29)].try_into()?),
                genre:u32::from_le_bytes(phdr[(i+30)..=(i+33)].try_into()?),
                morph:u32::from_le_bytes(phdr[(i+34)..=(i+37)].try_into()?),
            };
            presets.push(preset);
        }

        for i in 0..presets_len {
            let pbag_start = presets[i].pbag_index as usize;
            let pbag_end = if i == presets_len { preset_bags_len }else{ (presets[i+1].pbag_index-1) as usize };
            for j in pbag_start..=pbag_end {
                let mut zone = SF2Zone::new();
                for k in (preset_bags[j].generator_start as usize)..=(preset_bags[j].generator_end as usize) {
                    zone.generators.push(preset_zone.generators[k]);
                }
                for k in (preset_bags[j].modulator_start as usize)..=(preset_bags[j].modulator_end as usize) {
                    zone.modulators.push(preset_zone.modulators[k]);
                }
                presets[i].zones.push(zone);
            }
        }

        return Ok(Self{
            info:info,
            sample_headers:sample_headers,
            sample_data:sample_data,
            sm24_data:sm24_data,
            instruments:instruments,
            presets:presets
        });
    }
}