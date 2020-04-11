#![allow(dead_code)]

use std::io::Cursor;

use jpeg_decoder::Decoder;

use wce_map::binary_reader::{BinaryConverter, BinaryReader};
use wce_map::binary_writer::BinaryWriter;

type RGBA = u32;
type Mipmaps = Vec<Vec<u8>>;
pub const PALETTE_SIZE: usize = 256;
pub const JPG_BLP: bool = false;
pub const PALETTED_BLP: bool = true;
pub const MAX_MIPMAP: usize = 16;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum BlpFlag {
    RGB,
    RGBA,
    NoTeamColor,
}

impl BlpFlag {
    pub fn from(n: u32) -> Result<Self, &str>{
        match n{ //TODO faire conversion slim (regarder jpeg_decoder marker)
            3 | 4 => Ok(BlpFlag::RGBA),
            flag if flag >= 5 => Ok(BlpFlag::RGB),
            _ => Err("Unknown or unsupported blp flag"),
        }
    }
}

#[derive(Debug)]
pub enum BlpData {
//    JpgBlp(JpgBlpData),
//    PalettedBlp(PalettedBlpData),
}

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum Compression {
    JPEG,
    PALETTE,
}

impl Compression {
    pub fn from(n: u32) -> Result<Self, &str>{
        match n{ //TODO faire conversion slim (regarder jpeg_decoder marker)
            0 => Ok(Compression::JPEG),
            1 => Ok(Compression::PALETTE),
            _ => Err("Unknown BLP type")
        }
    }
}

pub struct BLP{
    magic_num: String,
    compression: Compression,
    has_alpha: bool,
    width: u32,
    height: u32,
    flag: BlpFlag,
    smooth: bool, // u32
    mipmap_offsets: Vec<u32>,
    mipmap_sizes: Vec<u32>,

    jpeg_header_size: u32,
    jpeg_header: Vec<u8>,
    jpeg_mipmaps: Mipmaps,

    palette_colors: Vec<RGBA>,
    palette_rgb_indexes: Mipmaps,
    palette_alpha_indexes: Mipmaps,
}

impl BLP {
    fn parse_jpeg_mipmaps(&mut self, reader: &mut BinaryReader){
        self.jpeg_header_size = reader.read_u32();
        self.jpeg_header = reader.read_bytes(self.jpeg_header_size as usize);
        for i in 0..MAX_MIPMAP {
            let size = self.mipmap_sizes[i] as usize;
            let offset = self.mipmap_offsets[i] as i64;
            if size == 0 { break; }
            reader.seek_begin();
            reader.skip(offset);
            let mut jpeg_buffer = self.jpeg_header.clone();
            jpeg_buffer.reserve(size + 10);
            let mut raw = reader.read_bytes(size);
            jpeg_buffer.append(&mut raw);

            let mut reader = Cursor::new(jpeg_buffer);
            let mut decoder = Decoder::new(reader);
            let res = decoder.decode().expect("error while decoding");
            self.jpeg_mipmaps.push(res);
        }
    }

    fn parse_palette(&mut self, reader: &mut BinaryReader){
        blp.palette_colors = reader.read_vec_u32(PALETTE_SIZE);
        for i in 0..MAX_MIPMAP{
            let size = blp.mipmap_sizes[i] as usize;
            let offset = blp.mipmap_offsets[i] as i64;
            if size == 0 {continue;}
            reader.seek_begin();
            reader.skip(offset);

            blp.palette_rgb_indexes.push(reader.read_bytes(size));
            if blp.flag == BlpFlag::RGBA{
                blp.palette_alpha_indexes.push(reader.read_bytes(size));
            }
        }
    }

    pub fn get_jpeg_header(&self) -> &Vec<u8>{
        &self.jpeg_header
    }
    pub fn get_jpeg_mipmaps(&self) -> &Mipmaps{
        &self.jpeg_mipmaps
    }
}

impl BinaryConverter for BLP{
    fn read(reader: &mut BinaryReader) -> Self {
        let magic_num = String::from_utf8(reader.read_bytes(4)).unwrap();
        let compression = reader.read_u32();
        let compression = Compression::from(compression).unwrap();
        let has_alpha = reader.read_u32() == 0x0000_0008;
        let width = reader.read_u32();
        let height = reader.read_u32();
        let flag =  reader.read_u32();
        let flag = BlpFlag::from(flag).unwrap();
        let smooth = reader.read_u32() == 1;
        let mipmap_offsets = reader.read_vec_u32(MAX_MIPMAP);
        let mipmap_sizes = reader.read_vec_u32(MAX_MIPMAP);
        let mut blp = BLP {
            magic_num,
            compression,
            has_alpha,
            width,
            height,
            flag,
            smooth,
            mipmap_offsets,
            mipmap_sizes,
            jpeg_header_size: 0,
            jpeg_header: Vec::with_capacity(MAX_MIPMAP),
            jpeg_mipmaps: Vec::with_capacity(MAX_MIPMAP),
            palette_colors: vec![],
            palette_rgb_indexes: Vec::with_capacity(MAX_MIPMAP),
            palette_alpha_indexes: Vec::with_capacity(MAX_MIPMAP),
        };
        match blp.compression {
            Compression::JPEG => blp.parse_jpeg_mipmaps(reader),
            Compression::PALETTE => blp.parse_palette(reader)
        };
        println!("file cursor pos {} / {}", reader.pos(),reader.size());
        blp
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}
