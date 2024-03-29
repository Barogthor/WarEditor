use wce_formats::binary_reader::BinaryReader;
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::BinaryConverter;
use wce_formats::MapArchive;

use crate::globals::MAP_MENU_MINIMAP;
use crate::OpeningError;

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

    fn write(&self, _writer: &mut BinaryWriter) {
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
        let datasets = reader.read_vec::<MMPDataset>(count_dataset);
        MMPFile{unknown, datasets}
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

impl MMPFile{
    pub fn read_file(map: &mut MapArchive) -> Result<Self, OpeningError>{
        let file = map.open_file(MAP_MENU_MINIMAP).map_err(|e| OpeningError::MenuMinimap(format!("{}",e)))?;
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(map, &mut buffer).map_err(|e| OpeningError::MenuMinimap(format!("{}",e)))?;
        let mut reader = BinaryReader::new(buffer);
        Ok(reader.read::<MMPFile>())
    }

    pub fn debug(&self){
        println!("{:#?}",self);
    }
}