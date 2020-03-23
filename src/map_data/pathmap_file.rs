
use mpq::Archive;

use crate::globals::MAP_PATH_MAP;
use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use crate::map_data::binary_writer::BinaryWriter;

type Flag = u8;
#[derive(Debug)]
pub struct Path {
    flags: Flag,
}
impl Path{
    pub fn walkable(&self) -> bool{
        self.flags & 0x02 == 0
    }
    pub fn flyable(&self) -> bool{
        self.flags & 0x04 == 0
    }
    pub fn buildable(&self) -> bool{
        self.flags & 0x08 == 0
    }
    pub fn blight(&self) -> bool{
        self.flags & 0x20 == 0
    }
    pub fn water(&self) -> bool{
        self.flags & 0x40 == 0
    }
    pub fn normal(&self) -> bool{
        self.flags & 0x80 == 0 || !self.blight()
    }

    pub fn update_flags(&mut self, value: Flag){
        self.flags = value;
    }
}

#[derive(Debug)]
pub struct PathMapFile {
    id: String,
    version: u32,
    pathmap_width: u32,
    pathmap_height: u32,
    pathing: Vec<Path>,
}

impl PathMapFile {
    pub fn read_file(mpq: &mut Archive) -> Self{
        let file = mpq.open_file(MAP_PATH_MAP).unwrap();

        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(mpq, &mut buffer).unwrap();
//        let mut f = File::open(concat_path("war3map.wpm")).unwrap();
//        let mut buffer: Vec<u8> = Vec::new();
//        f.read_to_end(&mut buffer).unwrap();
//        let buffer_size = buffer.len();
        let mut reader = BinaryReader::new(buffer);
        reader.read::<PathMapFile>()
    }
    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

impl BinaryConverter for PathMapFile {
    fn read(reader: &mut BinaryReader) -> Self {
        let id = String::from_utf8(reader.read_bytes(4)).unwrap();
        let version = reader.read_u32();
        let pathmap_width = reader.read_u32();
        let pathmap_height = reader.read_u32();
        let mut pathing: Vec<Path> = Vec::new();
        for _i in 0..pathmap_width*pathmap_height{
            let flags= reader.read_u8();

//            println!("{:x}",flags);
            pathing.push(Path{ flags});
        }
        assert_eq!(reader.size(), reader.pos() as usize, "reader for {} hasn't reached EOF. Missing {} bytes", MAP_PATH_MAP, reader.size() - reader.pos() as usize);
        PathMapFile {
            id,
            version,
            pathmap_width,
            pathmap_height,
            pathing
        }
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}