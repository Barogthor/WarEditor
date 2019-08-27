use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use crate::map_data::binary_writer::BinaryWriter;
use std::fs::File;
use std::io::Read;
use crate::map_data::{PREFIX_SAMPLE_PATH, concat_path};
use mpq::Archive;
use crate::globals::MAP_TERRAIN;

#[derive(Debug)]
pub struct TilePoint {
    ground_height: i16,
    water_level: u16,
    ground_texture_and_flags: u8,
    texture_details: u8,
    cliff_texture_and_layer_height: u8,
}

impl TilePoint{
    pub fn boundary_flag(&self) -> bool{
        self.water_level & 0x4000 == 1
    }

    pub fn get_water_level(&self) -> i16{
        (self.water_level & 0xBFFF) as i16
    }

    pub fn ramp(&self) -> bool{
        self.ground_texture_and_flags & 0x0010 == 1
    }

    pub fn blight(&self) -> bool{
        self.ground_texture_and_flags & 0x0020 == 1
    }

    pub fn water(&self) -> bool{
        self.ground_texture_and_flags & 0x0040 == 1
    }

    pub fn ground_texture(&self) -> u8{
        self.ground_texture_and_flags >> 4
    }

    pub fn cliff_texture(&self) -> u8{
        self.cliff_texture_and_layer_height & 0x00FF
    }

    pub fn layer_height(&self) -> u8{
        self.cliff_texture_and_layer_height >> 4
    }

    pub fn set_boundary_flag(&mut self, value: bool){
        if value {
            self.water_level |= 0x4000;
        }
        else{
            self.water_level &= 0xBFFF;
        }
    }

}

impl BinaryConverter for TilePoint{
    fn read(reader: &mut BinaryReader) -> Self {
        let ground_height = reader.read_i16() ;
        let water_level = reader.read_u16() ;
        let ground_texture_and_flags = reader.read_u8() ;
        let texture_details = reader.read_u8() ;
        let cliff_texture_and_layer_height = reader.read_u8();
        TilePoint{
            ground_height,
            water_level,
            ground_texture_and_flags,
            texture_details,
            cliff_texture_and_layer_height
        }
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct EnvironmentFile {
    id: String,
    version: u32,
    main_tileset: u8,
    custom_tileset: bool,
    // from 32 bit integer
    ground_tilesets: Vec<String>,
    // max 16 [4]
    cliff_tilesets: Vec<String>,
    // max 16 [4]
    my_height: u32,
    mx_width: u32,
    center_offset_x: f32,
    center_offset_y: f32,
    tilepoints: Vec<TilePoint>, // [Mx*My]
}

impl EnvironmentFile{
    pub fn read_file(mpq: &mut Archive) -> Self{
        let file = mpq.open_file(MAP_TERRAIN).unwrap();

        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(mpq, &mut buffer).unwrap();
//        let mut f = File::open(concat_path("war3map.w3e")).unwrap();
//        let mut buffer: Vec<u8> = Vec::new();
//        f.read_to_end(&mut buffer).unwrap();
//        let buffer_size = buffer.len();
        let mut reader = BinaryReader::new(buffer);
        reader.read::<EnvironmentFile>()
    }

    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

impl BinaryConverter for EnvironmentFile{
    fn read(reader: &mut BinaryReader) -> Self {
        let id = String::from_utf8(reader.read_bytes(4)).unwrap();
        let version = reader.read_u32();
        let main_tileset = reader.read_u8();
        let custom_tileset = reader.read_u32() == 1;

        let count_ground_tiles = reader.read_u32(); //TODO Warning for > 16
        let mut ground_tilesets: Vec<String> = Vec::new();
        for _i in 0..count_ground_tiles{
            ground_tilesets.push(String::from_utf8(reader.read_bytes(4)).unwrap())
        }
        let count_cliff_tiles = reader.read_u32(); //TODO Warning for > 16
        let mut cliff_tilesets: Vec<String> = Vec::new();
        for _i in 0..count_cliff_tiles{
            cliff_tilesets.push(String::from_utf8(reader.read_bytes(4)).unwrap())
        }

        let my_height = reader.read_u32();
        let mx_width = reader.read_u32();
        let center_offset_x = reader.read_f32();
        let center_offset_y = reader.read_f32();
        let count_tilepoints: usize = (mx_width * my_height) as usize;
        let tilepoints = reader.read_vec::<TilePoint>(count_tilepoints);

        EnvironmentFile{
            id,
            version,
            main_tileset,
            custom_tileset,
            ground_tilesets,
            cliff_tilesets,
            my_height,
            mx_width,
            center_offset_x,
            center_offset_y,
            tilepoints
        }
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}