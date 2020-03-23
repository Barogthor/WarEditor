
use std::ffi::CString;

use mpq::Archive;

use crate::globals::MAP_IMPORT_LIST;
use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use crate::map_data::binary_writer::BinaryWriter;

type ImportPath = Vec<(ImportPathType, CString)>;

#[derive(Debug)]
pub struct ImportFile {
    version: u32,
    files: ImportPath
}

impl ImportFile {
    pub fn read_file(mpq: &mut Archive) -> Option<Self>{
        let file = mpq.open_file(MAP_IMPORT_LIST);
        match file {
            Ok(file) => {
                let mut buffer: Vec<u8> = vec![0; file.size() as usize];

                file.read(mpq, &mut buffer).unwrap();
                let mut reader = BinaryReader::new(buffer);
                let v = reader.read::<ImportFile>();
                Some(v)
            },
            _ => None
        }
    }
    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

impl BinaryConverter for ImportFile {
    fn read(reader: &mut BinaryReader) -> Self {
        let version = reader.read_u32();
        let count = reader.read_u32();
        let mut files: ImportPath = vec![];
        for _ in 0..count{
            let path_type = ImportPathType::from_u8(reader.read_u8()).unwrap();
            let path = reader.read_c_string();
            files.push((path_type,path));
        }

        assert_eq!(reader.size(), reader.pos() as usize, "reader for {} hasn't reached EOF. Missing {} bytes", MAP_IMPORT_LIST, reader.size() - reader.pos() as usize);
        ImportFile {
            version,
            files
        }
    }

    fn write(&self, writer: &mut BinaryWriter) {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImportPathType{
    STANDARD(u8),
    CUSTOM(u8),
}

impl ImportPathType{
    pub fn from_u8(n: u8) -> Option<ImportPathType> {
        match n{
            5 | 8 => Some(ImportPathType::STANDARD(n)),
            10 | 13 => Some(ImportPathType::CUSTOM(n)),
            _ => None
        }
    }
}