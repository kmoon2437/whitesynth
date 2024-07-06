use riff::ChunkId;

pub const WSBK: ChunkId = ChunkId { value: [b'w', b's', b'b', b'k'] };
pub const NAME: ChunkId = ChunkId { value: [b'n', b'a', b'm', b'e'] };

pub const SMLS: ChunkId = ChunkId { value: [b's', b'm', b'l', b's'] };
pub const SMPL: ChunkId = ChunkId { value: [b's', b'm', b'p', b'l'] };
pub const SMHD: ChunkId = ChunkId { value: [b's', b'm', b'h', b'd'] };
pub const SMDT: ChunkId = ChunkId { value: [b's', b'm', b'd', b't'] };

pub const LRGN: ChunkId = ChunkId { value: [b'l', b'r', b'g', b'n'] };
pub const RGNI: ChunkId = ChunkId { value: [b'r', b'g', b'n', b'i'] };
pub const RGNH: ChunkId = ChunkId { value: [b'r', b'g', b'n', b'h'] };

pub const ARTC: ChunkId = ChunkId { value: [b'a', b'r', b't', b'c'] };

pub const LINS: ChunkId = ChunkId { value: [b'l', b'i', b'n', b's'] };
pub const INST: ChunkId = ChunkId { value: [b'i', b'n', b's', b't'] };
pub const INSH: ChunkId = ChunkId { value: [b'i', b'n', b's', b'h'] };

pub const LPRS: ChunkId = ChunkId { value: [b'l', b'p', b'r', b's'] };
pub const PRST: ChunkId = ChunkId { value: [b'p', b'r', b's', b't'] };
pub const PRSH: ChunkId = ChunkId { value: [b'p', b'r', b's', b'h'] };