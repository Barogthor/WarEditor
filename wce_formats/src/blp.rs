#![allow(dead_code)]

use std::io::Cursor;

use jpeg_decoder::Decoder;
use rgb::{RGB8, RGBA8};
use crate::binary_reader::BinaryReader;

type MipmapPixels = Vec<Vec<RGB8>>;
type MipmapIndexes = Vec<Vec<u8>>;
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
    pub fn from(n: u32) -> Result<Self, String>{
        match n{ //TODO faire conversion slim (regarder jpeg_decoder marker)
            3 | 4 => Ok(BlpFlag::RGBA),
            flag if flag >= 5 => Ok(BlpFlag::RGB),
            _ => Err(format!("Unknown or unsupported blp flag")),
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
    pub fn from(n: u32) -> Result<Self, String>{
        match n{ //TODO faire conversion slim (regarder jpeg_decoder marker)
            0 => Ok(Compression::JPEG),
            1 => Ok(Compression::PALETTE),
            _ => Err(format!("Unknown BLP type"))
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
    jpeg_mipmaps: MipmapPixels,

    palette_colors: Vec<RGBA8>,
    palette_rgb_indexes: MipmapIndexes,
    palette_alpha_indexes: MipmapIndexes,
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

            let reader = Cursor::new(jpeg_buffer);
            let mut decoder = Decoder::new(reader);
            let mut res = decoder.decode().expect("error while decoding");
            let pixels: Vec<RGB8> = res.chunks_mut(4).map(|cmyk| cmyk_to_rgb(cmyk) ).collect();
            self.jpeg_mipmaps.push(pixels);

        }
    }

    fn parse_palette(&mut self, reader: &mut BinaryReader){
        self.palette_colors = reader.read_bytes(PALETTE_SIZE * 4).chunks(4)
            .map(|bgra| RGBA8{
                r: bgra[2],
                g: bgra[1],
                b: bgra[0],
                a: 255 - bgra[3]
            } ).collect();
        for i in 0..MAX_MIPMAP{
            let size = self.mipmap_sizes[i] as usize;
            let offset = self.mipmap_offsets[i] as i64;
            if size == 0 {continue;}
            reader.seek_begin();
            reader.skip(offset);

            self.palette_rgb_indexes.push(reader.read_bytes(size));
            if self.flag == BlpFlag::RGBA{
                self.palette_alpha_indexes.push(reader.read_bytes(size));
            }
        }
    }

    pub fn get_jpeg_header(&self) -> &Vec<u8>{
        &self.jpeg_header
    }
    pub fn get_jpeg_mipmaps(&self) -> &MipmapPixels{
        &self.jpeg_mipmaps
    }
}

impl BLP{
    fn from(reader: &mut BinaryReader) -> Self {
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
}

fn cmyk_to_rgb(cmyk: &mut [u8]) -> RGB8{
        let c = cmyk[0] as f32 / 255.0;
        let y = cmyk[1] as f32 / 255.0;
        let m = cmyk[2] as f32 / 255.0;
        let k = cmyk[3] as f32 / 255.0;
        let red = 255.0 * (1. - c) * (1. - k);
//        let red = 255.0 - c;
        let green = 255.0 * (1. - y) * (1. - k);
//        let green = 255.0 - y;
        let blue = 255.0 * (1. - m) * (1. - k);
//        let blue = 255.0 - m;
        RGB8 {r: red as u8, b: blue as u8, g: green as u8 }
}

//     data.chunks(4).for_each(|cmyk| {

//         println!("[{:.0}, {:.0}, {:.0}] or [{:.0}, {:.0}, {:.0}, {:.0}]", red, green, blue, c*100., m*100. , y*100., k*100.);
//     });