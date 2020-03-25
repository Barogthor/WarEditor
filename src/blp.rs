use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use crate::map_data::binary_writer::BinaryWriter;

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
        let compression = match reader.read_u32(){
            0 => Ok(Compression::JPEG),
            1 => Ok(Compression::PALETTE),
            _ => Err("Unknown BLP type")
        }.unwrap();
        let has_alpha = reader.read_u32() == 0x0000_0008;
        let width = reader.read_u32();
        let height = reader.read_u32();
        let flag = match reader.read_u32(){ //TODO faire conversion slim (regarder jpeg_decoder marker)
            3 | 4 => Ok(BlpFlag::RGBA),
            flag if flag >= 5 => Ok(BlpFlag::RGB),
            _ => Err("Unknown or unsupported blp flag"),
        }.unwrap();
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
        println!("buffer size: {}",reader.size());
        println!("position: {}",reader.pos());
        match blp.compression {
           Compression::JPEG => {
               blp.jpeg_header_size = reader.read_u32();
               blp.jpeg_header = reader.read_bytes(blp.jpeg_header_size as usize);
               println!("jpeg header: {:?}", blp.jpeg_header);
               blp.jpeg_mipmaps.reserve_exact(MAX_MIPMAP);
               for i in 0..MAX_MIPMAP{
                   let size = blp.mipmap_sizes[i] as usize;
                   let offset = blp.mipmap_offsets[i] as i64;
                   if size == 0 {continue;}
                   reader.seek_begin();
                   reader.skip(offset);
                   println!("mipmap[{:2}] | size: {:5}, offset: {:5}", i, size, offset);
//                   let mut jpeg_buffer = blp.jpeg_header.clone();
                   let mut jpeg_buffer = blp.jpeg_header.clone();
                   jpeg_buffer.reserve(size+10);
//                   jpeg_buffer.append(&mut vec![0xFF, 0xDA]);
                   let mut raw = reader.read_bytes(size);
                   blp.jpeg_mipmaps.insert(i,raw.clone());

//                   println!("raw jpeg mipmap size: {}", raw.len());
//                   jpeg_buffer.append(&mut raw);
//                   println!("jpeg mipmap size: {}", jpeg_buffer.len());

//                   let mut reader = Cursor::new(jpeg_buffer);
//                   let mut decoder = JPEGDecoder::new(reader).unwrap();
//                   let res = decoder.read_image().unwrap();
//                   let mut decoder = Decoder::new(reader);
//                   decoder.read_info();
//                   let info = decoder.info();
//                   println!("{:#?}", info);
//                   let res = decoder.decode().expect("error while decoding");
//                   mmap.jpeg_mipmaps.push();
                }
            },
            Compression::PALETTE => {
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
        };
        println!("file cursor pos {} / {}", reader.pos(),reader.size());
        blp
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}
