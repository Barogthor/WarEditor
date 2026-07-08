//! Parser and writer for `war3map.wtg` (GUI trigger definitions: categories, variables and
//! trigger event/condition/action trees), driven by the `TriggerData.txt` schema.

use std::convert::TryFrom;

use thiserror::Error;
use wce_formats::binary_reader::BinaryReader;
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::GameVersion::{self, RoC, TFT};
// use log::{debug, error, info, trace, warn};
use wce_formats::{MapArchive, MpqError, ReadError, WriteError};

use crate::data_ini::DataIni;
use crate::globals::MAP_TRIGGERS;
use crate::triggers::enums::WtgError::{self, UnknownGameVersion};
use crate::triggers::misc::{TriggerCategory, VariableDefinition};
use crate::triggers::trigger_data::ECADefinition;
use crate::MapError;

mod enums;
mod misc;
mod trigger_data;
mod wtg_tests;

#[derive(Debug, Error)]
pub enum TriggersError {
    #[error("MPQ opening failure. {0}")]
    MpqError(#[from] MpqError),
    #[error("Failed to initialize binary reader. {0}")]
    InitReader(#[from] ReadError),
    #[error("Failed to parse trigger data. {0}")]
    Parsing(#[from] WtgError),
    #[error("Failed to save trigger data. {0}")]
    SaveError(WriteError),
}

#[derive(Debug)]
pub struct TriggerDefinition {
    name: String,
    description: String,
    is_comment: Option<bool>,
    enabled: bool,
    is_gui: bool,
    is_on: bool,
    run_on_init: bool,
    index_category: u32,
    ecas: Vec<ECADefinition>,
}

impl TriggerDefinition {
    pub fn from(
        reader: &mut BinaryReader,
        game_version: &GameVersion,
        trigger_data: &DataIni,
    ) -> Result<Self, WtgError> {
        let name = reader
            .read_c_string_converted()
            .map_err(WtgError::ErrorReader)?;
        let description = reader
            .read_c_string_converted()
            .map_err(WtgError::ErrorReader)?;
        let is_comment = match game_version {
            RoC => None,
            _ => Some(reader.read_u32().map_err(WtgError::ErrorReader)? != 0),
        };
        let enabled = reader.read_u32().map_err(WtgError::ErrorReader)? == 1;
        let is_gui = reader.read_u32().map_err(WtgError::ErrorReader)? == 0;
        let is_on = reader.read_u32().map_err(WtgError::ErrorReader)? == 0;
        let run_on_init = reader.read_i32().map_err(WtgError::ErrorReader)? == 0;
        let index_category = reader.read_u32().map_err(WtgError::ErrorReader)?;
        let count_ecas = reader.read_u32().map_err(WtgError::ErrorReader)?;
        let mut ecas = vec![];
        for _ in 0..count_ecas {
            ecas.push(ECADefinition::from(
                reader,
                game_version,
                trigger_data,
                false,
            )?);
        }
        Ok(Self {
            name,
            description,
            is_comment,
            enabled,
            is_gui,
            is_on,
            run_on_init,
            index_category,
            ecas,
        })
    }

    pub fn write(
        &self,
        writer: &mut BinaryWriter,
        game_version: &GameVersion,
        trigger_data: &DataIni,
    ) -> Result<(), WtgError> {
        writer.write_c_string_converted(&self.name)?;
        writer.write_c_string_converted(&self.description)?;
        if game_version.is_tft() {
            if let Some(is_comment) = self.is_comment {
                writer.write_u32(is_comment as u32)?;
            } else {
                writer.write_u32(0)?;
            }
        }
        writer.write_u32(self.enabled as u32)?;
        writer.write_u32((!self.is_gui) as u32)?;
        writer.write_u32((!self.is_on) as u32)?;
        writer.write_u32((!self.run_on_init) as u32)?;
        writer.write_u32(self.index_category)?;
        writer.write_u32(self.ecas.len() as u32)?;
        for eca in &self.ecas {
            eca.write(writer, game_version, trigger_data, false)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct TriggersFile {
    id: String,
    version: GameVersion,
    categories: Vec<TriggerCategory>,
    unknown: i32,
    vars: Vec<VariableDefinition>,
    triggers: Vec<TriggerDefinition>,
}

impl TriggersFile {
    pub const FILE_NAME: &str = MAP_TRIGGERS;

    pub fn read_file(map: &mut MapArchive, trigger_data: &DataIni) -> Result<Self, MapError> {
        let buffer = map
            .read_file(MAP_TRIGGERS)
            .map_err(TriggersError::MpqError)?;
        let mut reader = BinaryReader::try_from(buffer).map_err(TriggersError::InitReader)?;
        let res = Self::from(&mut reader, trigger_data).map_err(TriggersError::Parsing)?;
        Ok(res)
    }

    pub fn prepare_write(&self, trigger_data: &DataIni) -> Result<BinaryWriter, TriggersError> {
        let mut writer = BinaryWriter::new();
        self.write(&mut writer, trigger_data)?;
        Ok(writer)
    }

    fn write(&self, writer: &mut BinaryWriter, trigger_data: &DataIni) -> Result<(), WtgError> {
        writer.write_string_utf8(&self.id)?;
        writer.write_u32(from_game_version(&self.version))?;
        writer.write_u32(self.categories.len() as u32)?;
        for cat in &self.categories {
            cat.write(writer, &self.version)?;
        }
        writer.write_i32(self.unknown)?;
        writer.write_u32(self.vars.len() as u32)?;
        for var in &self.vars {
            var.write(writer, &self.version)?;
        }
        writer.write_u32(self.triggers.len() as u32)?;
        for trigger in &self.triggers {
            trigger.write(writer, &self.version, trigger_data)?;
        }
        Ok(())
    }

    fn from(reader: &mut BinaryReader, trigger_data: &DataIni) -> Result<Self, WtgError> {
        let id = reader
            .read_string_utf8_safe(4)
            .map_err(WtgError::ErrorReader)?;
        let version = reader.read_u32().map_err(WtgError::ErrorReader)?;
        let version = to_game_version(version)?;
        let count_categories = reader.read_u32().map_err(WtgError::ErrorReader)?;
        let mut categories = vec![];
        for _ in 0..count_categories {
            categories
                .push(TriggerCategory::from(reader, &version).map_err(WtgError::ErrorReader)?);
        }
        let unknown = reader.read_i32().map_err(WtgError::ErrorReader)?;
        let count_vars = reader.read_u32().map_err(WtgError::ErrorReader)?;
        let mut vars = vec![];
        for _ in 0..count_vars {
            let var = VariableDefinition::from(reader, &version)?;
            vars.push(var);
        }
        let count_triggers = reader.read_u32().map_err(WtgError::ErrorReader)?;
        let mut triggers = vec![];
        for _ in 0..count_triggers {
            // for _ in 0..3{
            triggers.push(TriggerDefinition::from(reader, &version, trigger_data)?)
        }
        if reader.size() != reader.pos() as usize {
            return Err(WtgError::ErrorReader(ReadError::TrailingBytes {
                file: MAP_TRIGGERS.into(),
                expected: reader.size(),
                actual: reader.pos() as usize,
            }));
        }
        Ok(Self {
            id,
            version,
            categories,
            unknown,
            vars,
            triggers,
        })
    }
}

fn to_game_version(value: u32) -> Result<GameVersion, WtgError> {
    match value {
        4 => Ok(RoC),
        7 => Ok(TFT),
        _ => Err(UnknownGameVersion(value)),
    }
}

fn from_game_version(game_version: &GameVersion) -> u32 {
    match game_version {
        RoC => 4,
        TFT => 7,
        GameVersion::Reforged => unimplemented!(),
    }
}
