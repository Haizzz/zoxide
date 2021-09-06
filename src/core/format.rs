// defining the zstd format based on the official specification

pub struct FrameHeader {
    pub single_segment_flag: bool,
    pub content_checksum_flag: bool,
    pub did_field_size: usize,
    pub window_size: u64,
    pub dictionary_id: u32,
    pub frame_content_size: u64,
}

#[derive(Debug)]
pub enum BlockType {
    Raw,
    Rle,
    Compressed,
    Reserved,
}

pub struct BlockHeader {
    pub last_block: bool,
    pub block_type: u8,
    pub block_size: u32,
}

pub struct Block {
    pub block_header: BlockType,
    pub block_content: Vec<u8>,
}

struct Frame {
    pub magic_number: [u8; 4],
    pub header: FrameHeader,
    pub data_blocks: Vec<Block>,
    pub content_checksum: Option<u32>,
}

struct Format {
    frames: Vec<Frame>,
}
