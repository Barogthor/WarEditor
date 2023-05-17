#[cfg(test)]
use pretty_assertions::assert_eq;

use wce_formats::binary_reader::BinaryReader;
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::BinaryConverter;
use wce_formats::MapArchive;

use crate::globals::MAP_CAMERAS;

type Degree = f32;

#[derive(Debug, Derivative)]
#[derivative(PartialEq, Default)]
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
    #[derivative(Default(value="100.0"))]
    unknown: f32,
    name: String,
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
        camera.name = reader.read_c_string().into_string().unwrap();
        camera
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}


#[derive(Debug)]
pub struct CameraFile {
    version: u32,
    cameras: Vec<Camera>,
}

impl CameraFile {
    pub fn read_file(map: &mut MapArchive) -> Option<Self>{
        let file = map.open_file(MAP_CAMERAS);
        match file {
            Ok(file) => {
                let mut buffer: Vec<u8> = vec![0; file.size() as usize];

                file.read(map, &mut buffer).unwrap();
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

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[cfg(test)]
mod w3c_test{
    use std::fs::File;

    use wce_formats::binary_reader::BinaryReader;

    use crate::camera_file::{Camera, CameraFile};

    fn mock_cameras() -> Vec<Camera>{
        vec![
        Camera{
            x: 758.24,
            y: 178.15,
            z: 13.5,
            rotation: 90.0,
            aoa: 304.0,
            dist: 1996.5,
            roll: 2.4,
            fov: 70.0,
            far_clip: 5000.0,
            unknown: 100.0,
            name: "Camera 001".to_string()
        }]
    }

    #[test]
    fn no_failure(){
        let mut w3c = File::open("../resources/Scenario/Sandbox_roc/war3map.w3c").unwrap();
        let mut reader = BinaryReader::from(&mut w3c);
        reader.read::<CameraFile>();
    }

    #[test]
    fn check_values(){
        let mut w3c = File::open("../resources/Scenario/Sandbox_roc/war3map.w3c").unwrap();
        let mut reader = BinaryReader::from(&mut w3c);
        let camera_file = reader.read::<CameraFile>();
        let mock_cameras = mock_cameras();
        assert_eq!(camera_file.cameras, mock_cameras);
    }
}