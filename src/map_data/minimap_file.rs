use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use std::fs::File;
use std::io::{Read, BufReader, Cursor};
use crate::map_data::binary_writer::BinaryWriter;
use crate::map_data::{PREFIX_SAMPLE_PATH, concat_path};
use mpq::Archive;
use crate::globals::MAP_MINIMAP;

type RGBA = Vec<u8>;
pub const JPG_BLP: bool = false;
pub const PALETTED_BLP: bool = true;
pub const MAX_MIPMAP: usize = 16;
type MinimapType1 = bool;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum MinimapFlag{
    RGB,
    RGBA,
    NoTeamColor,
}
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum MinimapType {
    JpgBlp,
    PalettedBLP,
}

#[derive(Debug)]
pub struct JpgBlpData {
    header_size: u32,
    header: Vec<u8>,

    raw_data: Vec<u8>,
}

impl BinaryConverter for JpgBlpData{
    fn read(reader: &mut BinaryReader) -> Self {

        JpgBlpData{
            header_size: 0,
            header: vec![],
            raw_data: vec![]
        }
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct PalettedBlpData {

}
impl BinaryConverter for PalettedBlpData{
    fn read(reader: &mut BinaryReader) -> Self {

        PalettedBlpData{

        }
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum MinimapData {
    JpgBlp(JpgBlpData),
    PalettedBlp(PalettedBlpData),
}

#[derive(Debug)]
pub struct MinimapFile { //TODO
    id: String,
    minimap_type: MinimapType,
    has_alpha: bool,
    width: u32,
    height: u32,
    flag: MinimapFlag,
    smooth: bool, // u32
    mipmap_offsets: Vec<u32>,
    mipmap_sizes: Vec<u32>,

    jpeg_header_size: u32,
    jpeg_header: Vec<u8>,
    jpeg_mipmaps: Vec<Vec<u8>>,

}

impl BinaryConverter for MinimapFile {
    fn read(reader: &mut BinaryReader) -> Self {
        let id = String::from_utf8(reader.read_bytes(4)).unwrap();
        let minimap_type = match reader.read_u32(){
            0 => Ok(MinimapType::JpgBlp),
            1 => Ok(MinimapType::PalettedBLP),
            _ => Err("Unknown Minimap type")
        }.unwrap();
        let has_alpha = reader.read_u32() == 0x0000_0008;
        let width = reader.read_u32();
        let height = reader.read_u32();
        let flag = match reader.read_u32(){ //TODO faire conversion slim (regarder jpeg_decoder marker)
            3 | 4 => Ok(MinimapFlag::RGBA),
            5 => Ok(MinimapFlag::RGB),
            flag if flag >= 5 => Ok(MinimapFlag::NoTeamColor),
            _ => Err("Unknown or unsupported minimap flag"),
        }.unwrap();
        let smooth = reader.read_u32() == 1;
        println!("position: {}",reader.pos());
        let mipmap_offsets = reader.read_vec_u32(MAX_MIPMAP);
        let mipmap_sizes = reader.read_vec_u32(MAX_MIPMAP);
        println!("mipmap.txt: {}, offset: {}",mipmap_sizes[0], mipmap_offsets[0]);

        let mut mmap = MinimapFile {
            id,
            minimap_type,
            has_alpha,
            width,
            height,
            flag,
            smooth,
            mipmap_offsets,
            mipmap_sizes,
            jpeg_header_size: 0,
            jpeg_header: vec![],
            jpeg_mipmaps: vec![]
        };
        match mmap.minimap_type {
            MinimapType::JpgBlp => {
                mmap.jpeg_header_size = reader.read_u32();
                for i in 0..MAX_MIPMAP{
                    let size = mmap.mipmap_sizes[i] as usize;
                    let offset = mmap.mipmap_offsets[i] as i64;
                    if size == 0 {continue;}
                    reader.seek_begin();
                    reader.skip(offset);
                    println!("mipmap.txt: {}, offset: {}, pos: {}",size, offset,reader.pos());
                    let mut raw = reader.read_bytes(size);
                    raw.insert(0,0xD8);
                    raw.insert(0,0xFF);

//                    let mut reader = Cursor::new(raw);
//                    let mut decoder = Decoder::new(reader);
//                    decoder.read_info();
//                    let info = decoder.info();
//                    let res = decoder.decode().unwrap();
                    let p = 0;
//                    println!("{:#?}", info);
//                    mmap.jpeg_mipmaps.push();
                }
            },
            MinimapType::PalettedBLP => {

            }
        };
        mmap
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

impl MinimapFile {
    pub fn read_file(mpq: &mut Archive) -> Self{
        let file = mpq.open_file(MAP_MINIMAP).unwrap();
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(mpq, &mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer);
        reader.read::<MinimapFile>()
    }

    pub fn debug(&self){
        println!("{:#?}",self);
    }
}