use wce_formats::{BinaryConverter, GameVersion};
use wce_formats::binary_reader::BinaryReader;
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::GameVersion::{RoC, TFT};
use wce_formats::MapArchive;

use crate::globals::MAP_TRIGGERS_SCRIPT;
use crate::OpeningError;

type TextScript = String;

#[derive(Debug)]
pub struct TriggerJassFile {
    version: GameVersion,
    global_comment: String,
    global_script: TextScript,
    triggers_script: Vec<TextScript>,
}

impl TriggerJassFile {
    pub fn read_file(map: &mut MapArchive) -> Result<Self, OpeningError>{
        let file = map.open_file(MAP_TRIGGERS_SCRIPT).map_err(|e| OpeningError::CustomTextTrigger(format!("{}",e)))?;
        let mut buffer: Vec<u8> = vec![0; file.size() as usize];

        file.read(map, &mut buffer).map_err(|e| OpeningError::CustomTextTrigger(format!("{}",e)))?;
        let mut reader = BinaryReader::new(buffer);
        Ok(reader.read::<TriggerJassFile>())
    }
    pub fn debug(&self){
        println!("{:#?}",self);
    }
}

impl BinaryConverter for TriggerJassFile {
    fn read(reader: &mut BinaryReader) -> Self {
        let version = reader.read_u32();
        let version = to_game_version(version);
        let mut global_comment: String = Default::default();
        let mut global_script: String = Default::default();
        let mut text_triggers: Vec<TextScript> = Vec::new();
        match version {
            RoC => (),
            _  => {
                global_comment = reader.read_c_string().into_string().unwrap();
                let s = reader.read_u32() as usize;
                global_script =  reader.read_string_utf8(s);
            }
        }
        let count_triggers = reader.read_u32() as usize;
        for _ in 0..count_triggers{
            let length = reader.read_u32() as usize;
            if length == 0 { continue; }
            text_triggers.push(reader.read_string_utf8(length));
        }
        assert_eq!(reader.size(), reader.pos() as usize, "reader for {} hasn't reached EOF. Missing {} bytes", MAP_TRIGGERS_SCRIPT, reader.size() - reader.pos() as usize);

        TriggerJassFile {
            version,
            global_comment,
            global_script,
            triggers_script: text_triggers,

        }
    }

    fn write(&self, _writer: &mut BinaryWriter) {
        unimplemented!()
    }
}


fn to_game_version(value: u32) -> GameVersion{
    match value{
        0 => RoC,
        1 => TFT,
        _ => panic!("Unknown or unsupported game version '{}'", value)
    }
}

