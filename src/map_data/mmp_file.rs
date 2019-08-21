use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use std::fs::File;
use std::io::Read;
use crate::map_data::binary_writer::BinaryWriter;

type RGBA = Vec<u8>;

#[derive(Debug)]
pub struct MMPDataset {
    icon_type: u32,
    x: i32,
    y: i32,
    color: RGBA,
}
impl BinaryConverter for MMPDataset{
    fn read(reader: &mut BinaryReader) -> Self {
        let icon_type = reader.read_u32();
        let x = reader.read_i32();
        let y = reader.read_i32();
        let color = reader.read_bytes(4);
        MMPDataset{
            icon_type,
            x,
            y,
            color
        }
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct MMPFile {
    unknown: i32,
    datasets: Vec<MMPDataset>,
}

impl BinaryConverter for MMPFile{
    fn read(reader: &mut BinaryReader) -> Self {
        let unknown = reader.read_i32();
        let count_dataset = reader.read_i32() as usize;
        let mut datasets = reader.read_vec::<MMPDataset>(count_dataset);
        MMPFile{unknown, datasets}
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

impl MMPFile{
    pub fn read_file() -> Self{
        let mut f = File::open("resources/war3map.mmp").unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        let buffer_size = buffer.len();
        let mut reader = BinaryReader::new(buffer);
        reader.read::<MMPFile>()
    }

    pub fn debug(&self){
        println!("{:#?}",self);
    }
}