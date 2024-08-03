/**
 * 자체 사운드뱅크 파일 포맷
 * 개인적으로 big endian을 선호하지만 riff에서 little endian을 쓰는 관계로 일관성을 위해 little endian을 사용함
 */

use std::sync::Arc;
use std::io::{ Read, Seek, Cursor, Write };
use std::collections::HashMap;
use anyhow::bail;
use riff::{ Chunk, ChunkContents };

use crate::util;

pub mod consts;
pub mod fourcc;

fn make_name(name: &str) -> ChunkContents {
    return ChunkContents::Data(fourcc::NAME, name.as_bytes().to_vec());
}

// 해당 샘플이 mono인지 stereo인지 정의함
pub enum SampleType {
    Mono, Stereo
}

impl SampleType {
    fn from_byte(val: u8) -> anyhow::Result<Self> {
        return Ok(match val {
            0x00 => Self::Mono,
            0x01 => Self::Stereo,
            _ => bail!("Invalid sample type")
        });
    }

    fn as_byte(&self) -> u8 {
        return match self {
            Self::Mono => 0x00,
            Self::Stereo => 0x01
        };
    }
}

// 해당 샘플의 루프 방식을 정의함
pub enum LoopType {
    NoLoop, Infinite, UntilReleased
}

impl LoopType {
    fn from_byte(val: u8) -> anyhow::Result<Self> {
        return Ok(match val {
            0x00 => Self::NoLoop,
            0x01 => Self::Infinite,
            0x02 => Self::UntilReleased,
            _ => bail!("Invalid loop type")
        });
    }

    fn as_byte(&self) -> u8 {
        return match self {
            Self::NoLoop => 0x00,
            Self::Infinite => 0x01,
            Self::UntilReleased => 0x02
        };
    }
}

pub struct Sample {
    // 샘플 이름
    pub name: String,

    // 샘플 1개의 형태 + bit depth
    // 16, 24면 정수 샘플이고 32, 64면 부동소수점 샘플
    pub bit_depth: u16,

    // 초당 샘플 수
    pub sample_rate: u32,

    // SampleType 정의 참조
    pub sample_type: SampleType,

    // 루프 시작/끝(1샘플 단위)
    pub loop_start: u32,
    pub loop_end: u32,

    // 루프 방식 
    pub loop_type: LoopType,

    // 기본 키
    pub base_key: u8,

    // cent 단위(100 = 1키)
    pub cent_correction: i8,

    // 샘플 데이터
    // sample_type이 Stereo인 경우에는 L R L R 순서로 들어감
    pub data: Arc<Vec<u8>>
}

impl Sample {
    const SMHD_LEN: u32 = 18;

    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_owned(),
            bit_depth: 32,
            sample_rate: 48000,
            sample_type: SampleType::Stereo,
            loop_start: 0,
            loop_end: 0,
            loop_type: LoopType::NoLoop,
            base_key: 60,
            cent_correction: 0,
            data: Arc::new(vec![])
        };
    }

    fn parse_smls<T: Read + Seek>(list: &Chunk, stream: &mut T) -> anyhow::Result<Vec<Self>> {
        let mut samples = vec![];
        for chunk in util::unwrap_result_iter(list.iter(stream))? {
            if chunk.read_type(stream)? == fourcc::SMPL {
                samples.push(Self::parse_smpl(&chunk, stream)?);
            }
        }
        return Ok(samples);
    }

    fn parse_smpl<T: Read + Seek>(list: &Chunk, stream: &mut T) -> anyhow::Result<Self> {
        let mut name = String::new();
        let mut bit_depth = 32;
        let mut sample_rate = 48000;
        let mut sample_type = SampleType::Stereo;
        let mut loop_start = 0;
        let mut loop_end = 0;
        let mut loop_type = LoopType::NoLoop;
        let mut base_key = 60;
        let mut cent_correction = 0;
        let mut data = Arc::new(vec![]);

        for chunk in util::unwrap_result_iter(list.iter(stream))? {
            let chunk_id = chunk.id();
            if chunk_id == fourcc::NAME {
                name.push_str(std::str::from_utf8(&chunk.read_contents(stream)?)?);
            } else if chunk_id == fourcc::SMHD {
                if chunk.len() < Self::SMHD_LEN {
                    bail!("Invalid sample header(smhd) length");
                }
                let contents = chunk.read_contents(stream)?;
                bit_depth = u16::from_le_bytes(contents[0..2].try_into()?);
                sample_rate = u32::from_le_bytes(contents[2..6].try_into()?);
                sample_type = SampleType::from_byte(contents[6])?;
                loop_start = u32::from_le_bytes(contents[7..11].try_into()?);
                loop_end = u32::from_le_bytes(contents[11..15].try_into()?);
                loop_type = LoopType::from_byte(contents[15])?;
                base_key = contents[16];
                cent_correction = i8::from_le_bytes(contents[17..18].try_into()?);
            } else if chunk_id == fourcc::SMDT {
                let contents = chunk.read_contents(stream)?;
                data = Arc::new(contents);
            }
        }

        return Ok(Self {
            name, bit_depth,
            sample_rate, sample_type,
            loop_start, loop_end,
            loop_type, base_key,
            cent_correction, data
        });
    }

    fn make_smhd(&self) -> anyhow::Result<ChunkContents> {
        let mut stream = Cursor::new(vec![]);
        stream.write_all(&self.bit_depth.to_le_bytes())?;
        stream.write_all(&self.sample_rate.to_le_bytes())?;
        stream.write_all(&[self.sample_type.as_byte()])?;
        stream.write_all(&self.loop_start.to_le_bytes())?;
        stream.write_all(&self.loop_end.to_le_bytes())?;
        stream.write_all(&[self.loop_type.as_byte()])?;
        stream.write_all(&[self.base_key])?;
        stream.write_all(&self.cent_correction.to_le_bytes())?;
        return Ok(ChunkContents::Data(fourcc::SMHD, stream.into_inner()));
    }

    fn make_smdt(&self) -> ChunkContents {
        return ChunkContents::Data(fourcc::SMDT, Vec::clone(&self.data));
    }

    fn to_smpl(&self) -> anyhow::Result<ChunkContents> {
        let chunks = vec![
            make_name(&self.name),
            self.make_smhd()?,
            self.make_smdt()
        ];
        return Ok(ChunkContents::Children(riff::LIST_ID, fourcc::SMPL, chunks));
    }

    fn make_smls(samples: &Vec<Self>) -> anyhow::Result<ChunkContents> {
        let mut chunks = vec![];
        for sample in samples.iter() {
            chunks.push(sample.to_smpl()?);
        }
        return Ok(ChunkContents::Children(riff::LIST_ID, fourcc::SMLS, chunks));
    }
}

pub struct Articulator {
    pub src: u32,
    pub src_transform: u8,
    pub control: u32,
    pub control_transform: u8,
    pub destination: u32,
    pub main_transform: u8,
    pub scale: f64
}

impl Articulator {
    fn parse_artc(content: Vec<u8>) -> anyhow::Result<Vec<Self>> {
        let mut stream = Cursor::new(content);

        let mut count_bytes = [0; 4];
        stream.read_exact(&mut count_bytes)?;
        let count = u32::from_le_bytes(count_bytes);

        let mut articulators = vec![];
        for _ in 0..count {
            let mut src_bytes = [0; 4];
            stream.read_exact(&mut src_bytes)?;
            let src = u32::from_le_bytes(src_bytes);

            let mut src_transform = [0];
            stream.read_exact(&mut src_transform)?;
            let src_transform = src_transform[0];

            let mut control_bytes = [0; 4];
            stream.read_exact(&mut control_bytes)?;
            let control = u32::from_le_bytes(control_bytes);

            let mut control_transform = [0];
            stream.read_exact(&mut control_transform)?;
            let control_transform = control_transform[0];

            let mut destination_bytes = [0; 4];
            stream.read_exact(&mut destination_bytes)?;
            let destination = u32::from_le_bytes(destination_bytes);

            let mut main_transform = [0];
            stream.read_exact(&mut main_transform)?;
            let main_transform = main_transform[0];

            let mut scale = [0; 8];
            stream.read_exact(&mut scale)?;
            let scale = f64::from_le_bytes(scale);

            articulators.push(Self {
                src, src_transform,
                control, control_transform,
                destination,
                main_transform,
                scale
            });
        }
        return Ok(articulators);
    }

    fn make_artc(articulators: &Vec<Self>) -> anyhow::Result<ChunkContents> {
        let mut stream = Cursor::new(vec![]);
        stream.write_all(&u32::to_le_bytes(articulators.len() as u32))?;
        for articulator in articulators.iter() {
            stream.write_all(&articulator.src.to_le_bytes())?;
            stream.write_all(&[articulator.src_transform])?;
            stream.write_all(&articulator.control.to_le_bytes())?;
            stream.write_all(&[articulator.control_transform])?;
            stream.write_all(&articulator.destination.to_le_bytes())?;
            stream.write_all(&[articulator.main_transform])?;
            stream.write_all(&articulator.scale.to_le_bytes())?;
        }
        return Ok(ChunkContents::Data(fourcc::ARTC, stream.into_inner()));
    }
}

pub struct Region {
    // key 범위
    pub key_range: (u8, u8),

    // velocity 범위
    pub velocity_range: (u8, u8),

    // 대상 index
    // instrument region이면 샘플, preset region이면 instrument를 가리킴
    pub target_index: u32,

    // generator
    pub generators: HashMap<u16, i32>,

    // articulator
    pub articulators: Vec<Articulator>
}

impl Region {
    const RGNH_LEN: u32 = 8;

    fn parse_lrgn<T: Read + Seek>(list: &Chunk, stream: &mut T) -> anyhow::Result<Vec<Self>> {
        let mut regions = vec![];
        for chunk in util::unwrap_result_iter(list.iter(stream))? {
            if chunk.read_type(stream)? == fourcc::RGNI {
                regions.push(Self::parse_rgni(&chunk, stream)?);
            }
        }
        return Ok(regions);
    }

    fn parse_rgni<T: Read + Seek>(list: &Chunk, stream: &mut T) -> anyhow::Result<Self> {
        let mut key_range = (0, 127);
        let mut velocity_range = (0, 127);
        let mut target_index = 0;
        let mut articulators = vec![];

        for chunk in util::unwrap_result_iter(list.iter(stream))? {
            let chunk_id = chunk.id();
            if chunk_id == fourcc::RGNH {
                if chunk.len() < Self::RGNH_LEN {
                    bail!("Invalid region header(rgnh) length");
                }
                let contents = chunk.read_contents(stream)?;
                key_range.0 = contents[0];
                key_range.1 = contents[1];
                velocity_range.0 = contents[2];
                velocity_range.1 = contents[3];
                target_index = u32::from_le_bytes(contents[4..8].try_into()?);
            } else if chunk_id == fourcc::ARTC {
                articulators.append(&mut Articulator::parse_artc(chunk.read_contents(stream)?)?);
            }
        }

        return Ok(Self {
            key_range, velocity_range,
            target_index,
            generators: HashMap::new(), articulators
        });
    }

    fn make_rgnh(&self) -> anyhow::Result<ChunkContents> {
        let mut stream = Cursor::new(vec![]);
        stream.write_all(&[
            self.key_range.0,
            self.key_range.1,
            self.velocity_range.0,
            self.velocity_range.1
        ])?;
        stream.write_all(&self.target_index.to_le_bytes())?;
        return Ok(ChunkContents::Data(fourcc::RGNH, stream.into_inner()));
    }

    fn to_rgni(&self) -> anyhow::Result<ChunkContents> {
        let chunks = vec![
            self.make_rgnh()?,
            Articulator::make_artc(&self.articulators)?
        ];
        return Ok(ChunkContents::Children(riff::LIST_ID, fourcc::RGNI, chunks));
    }

    fn make_lrgn(regions: &Vec<Self>) -> anyhow::Result<ChunkContents> {
        let mut chunks = vec![];
        for region in regions.iter() {
            chunks.push(region.to_rgni()?);
        }
        return Ok(ChunkContents::Children(riff::LIST_ID, fourcc::LRGN, chunks));
    }
}

pub struct Instrument {
    pub name: String,
    pub regions: Vec<Region>
}

impl Instrument {
    fn parse_lins<T: Read + Seek>(list: &Chunk, stream: &mut T) -> anyhow::Result<Vec<Self>> {
        let mut instruments = vec![];
        for chunk in util::unwrap_result_iter(list.iter(stream))? {
            if chunk.read_type(stream)? == fourcc::INST {
                instruments.push(Self::parse_inst(&chunk, stream)?);
            }
        }
        return Ok(instruments);
    }

    fn parse_inst<T: Read + Seek>(list: &Chunk, stream: &mut T) -> anyhow::Result<Self> {
        let mut name = String::new();
        let mut regions = vec![];

        for chunk in util::unwrap_result_iter(list.iter(stream))? {
            let chunk_id = chunk.id();
            if chunk_id == fourcc::NAME {
                name.push_str(std::str::from_utf8(&chunk.read_contents(stream)?)?);
            } else if chunk_id == riff::LIST_ID {
                if chunk.read_type(stream)? == fourcc::LRGN {
                    regions.append(&mut Region::parse_lrgn(&chunk, stream)?);
                }
            }
        }

        return Ok(Self { name, regions });
    }

    fn to_inst(&self) -> anyhow::Result<ChunkContents> {
        let chunks = vec![
            make_name(&self.name),
            Region::make_lrgn(&self.regions)?
        ];
        return Ok(ChunkContents::Children(riff::LIST_ID, fourcc::INST, chunks));
    }

    fn make_lins(instruments: &Vec<Self>) -> anyhow::Result<ChunkContents> {
        let mut chunks = vec![];
        for instrument in instruments.iter() {
            chunks.push(instrument.to_inst()?);
        }
        return Ok(ChunkContents::Children(riff::LIST_ID, fourcc::LINS, chunks));
    }
}

pub enum PresetType {
    Melodic, Drum
}

impl PresetType {
    fn from_byte(val: u8) -> anyhow::Result<Self> {
        return Ok(match val {
            0x00 => Self::Melodic,
            0x01 => Self::Drum,
            _ => bail!("Invalid preset type")
        });
    }

    fn as_byte(&self) -> u8 {
        return match self {
            Self::Melodic => 0x00,
            Self::Drum => 0x01
        };
    }
}

pub struct Preset {
    pub name: String,
    pub program_no: u16,
    pub bank_msb: u8,
    pub bank_lsb: u8,
    pub type_flag: PresetType,
    pub regions: Vec<Region>
}

impl Preset {
    const PRSH_LEN: u32 = 5;

    fn parse_lprs<T: Read + Seek>(list: &Chunk, stream: &mut T) -> anyhow::Result<Vec<Self>> {
        let mut presets = vec![];
        for chunk in util::unwrap_result_iter(list.iter(stream))? {
            if chunk.read_type(stream)? == fourcc::PRST {
                presets.push(Self::parse_prst(&chunk, stream)?);
            }
        }
        return Ok(presets);
    }

    fn parse_prst<T: Read + Seek>(list: &Chunk, stream: &mut T) -> anyhow::Result<Self> {
        let mut name = String::new();
        let mut program_no = 0;
        let mut bank_msb = 0;
        let mut bank_lsb = 0;
        let mut type_flag = PresetType::Melodic;
        let mut regions = vec![];

        for chunk in util::unwrap_result_iter(list.iter(stream))? {
            let chunk_id = chunk.id();
            if chunk_id == fourcc::NAME {
                name.push_str(std::str::from_utf8(&chunk.read_contents(stream)?)?);
            } else if chunk_id == fourcc::PRSH {
                if chunk.len() < Self::PRSH_LEN {
                    bail!("Invalid preset header(prsh) length");
                }
                let contents = chunk.read_contents(stream)?;
                program_no = u16::from_le_bytes(contents[0..2].try_into()?);
                bank_msb = contents[2];
                bank_lsb = contents[3];
                type_flag = PresetType::from_byte(contents[4])?;
            } else if chunk_id == riff::LIST_ID {
                if chunk.read_type(stream)? == fourcc::LRGN {
                    regions.append(&mut Region::parse_lrgn(&chunk, stream)?);
                }
            }
        }

        return Ok(Self {
            name, program_no,
            bank_msb, bank_lsb,
            type_flag, regions
        });
    }
    
    fn make_prsh(&self) -> anyhow::Result<ChunkContents> {
        let mut stream = Cursor::new(vec![]);
        stream.write_all(&self.program_no.to_le_bytes())?;
        stream.write_all(&[
            self.bank_msb,
            self.bank_lsb,
            self.type_flag.as_byte()
        ])?;
        return Ok(ChunkContents::Data(fourcc::PRSH, stream.into_inner()));
    }

    fn to_prst(&self) -> anyhow::Result<ChunkContents> {
        let chunks = vec![
            make_name(&self.name),
            self.make_prsh()?,
            Region::make_lrgn(&self.regions)?
        ];
        return Ok(ChunkContents::Children(riff::LIST_ID, fourcc::PRST, chunks));
    }

    fn make_lprs(presets: &Vec<Self>) -> anyhow::Result<ChunkContents> {
        let mut chunks = vec![];
        for preset in presets.iter() {
            chunks.push(preset.to_prst()?);
        }
        return Ok(ChunkContents::Children(riff::LIST_ID, fourcc::LPRS, chunks));
    }
}

pub struct WSBK {
    pub samples: Vec<Sample>,
    pub instruments: Vec<Instrument>,
    pub presets: Vec<Preset>
}

impl WSBK {
    pub fn new() -> Self {
        return Self {
            samples: vec![],
            instruments: vec![],
            presets: vec![]
        };
    }

    pub fn read<T: Read + Seek>(stream: &mut T) -> anyhow::Result<Self> {
        let wsbk = Chunk::read(stream, 0)?;
        let mut samples = vec![];
        let mut instruments = vec![];
        let mut presets = vec![];

        if wsbk.read_type(stream)? != fourcc::WSBK {
            bail!("Invalid RIFF file type");
        }

        for chunk in util::unwrap_result_iter(wsbk.iter(stream))? {
            match chunk.read_type(stream)? {
                fourcc::SMLS => samples.append(&mut Sample::parse_smls(&chunk, stream)?),
                fourcc::LINS => instruments.append(&mut Instrument::parse_lins(&chunk, stream)?),
                fourcc::LPRS => presets.append(&mut Preset::parse_lprs(&chunk, stream)?),
                _ => {}
            }
        }

        return Ok(Self {
            samples, instruments, presets
        });
    }

    fn make_wsbk(&self) -> anyhow::Result<ChunkContents> {
        let chunks = vec![
            Sample::make_smls(&self.samples)?,
            Instrument::make_lins(&self.instruments)?,
            Preset::make_lprs(&self.presets)?,
        ];
        return Ok(ChunkContents::Children(riff::RIFF_ID, fourcc::WSBK, chunks));
    }

    pub fn write<T: Write + Seek>(&self, stream: &mut T) -> anyhow::Result<()> {
        self.make_wsbk()?.write(stream)?;
        return Ok(());
    }
}