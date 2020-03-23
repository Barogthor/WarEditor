use std::ffi::CString;

use mpq::Archive;

use crate::globals::MAP_CAMERAS;
use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use crate::map_data::binary_writer::BinaryWriter;

type Degree = f32;

#[derive(Debug)]
pub struct Camera {
    x: f32,
    y: f32,
    z: f32,
    rotation: Degree,
    aoa: Degree,
    dist: f32,
    roll: f32,
    fov: Degree,
    far_clip: f32,
    unknown: f32,
    name: CString,
}
impl Default for Camera {
    fn default() -> Self {
        Camera {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rotation: 0.0,
            aoa: 0.0,
            dist: 0.0,
            roll: 0.0,
            fov: 0.0,
            far_clip: 0.0,
            unknown: 100.0,
            name: Default::default()
        }
    }
}
impl BinaryConverter for Camera {
    fn read(reader: &mut BinaryReader) -> Self {
        let mut camera = Self::default();
        camera.x = reader.read_f32();
        camera.y = reader.read_f32();
        camera.z = reader.read_f32();
        camera.rotation = reader.read_f32();
        camera.aoa = reader.read_f32();
        camera.dist = reader.read_f32();
        camera.roll = reader.read_f32();
        camera.fov = reader.read_f32();
        camera.far_clip = reader.read_f32();
        camera.unknown = reader.read_f32();
        camera.name = reader.read_c_string();
        camera
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}


#[derive(Debug)]
pub struct CameraFile {
    version: u32,
    cameras: Vec<Camera>,
}

impl CameraFile {
    pub fn read_file(mpq: &mut Archive) -> Option<Self>{
        let file = mpq.open_file(MAP_CAMERAS);
        match file {
            Ok(file) => {
                let mut buffer: Vec<u8> = vec![0; file.size() as usize];

                file.read(mpq, &mut buffer).unwrap();
                let mut reader = BinaryReader::new(buffer);
                Some(reader.read::<CameraFile>())
            }
            _ => None
        }
    }
    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

impl BinaryConverter for CameraFile {
    fn read(reader: &mut BinaryReader) -> Self {
        let version = reader.read_u32();
        let count_camera = reader.read_u32() as usize;
        let cameras = reader.read_vec::<Camera>(count_camera);

        assert_eq!(reader.size(), reader.pos() as usize, "reader for {} hasn't reached EOF. Missing {} bytes", MAP_CAMERAS, reader.size() - reader.pos() as usize);
        CameraFile {
            version,
            cameras
        }
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}