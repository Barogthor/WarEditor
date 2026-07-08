//! `ECADefinition` (event/condition/action tree nodes) and their `Parameter`/sub-parameter
//! types, parsed against `TriggerData.txt` while reading/writing `war3map.wtg`.

// use log::{debug, error, info, trace, warn};
use wce_formats::binary_reader::BinaryReader;
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::GameVersion::{self, RoC};

use crate::data_ini::DataIni;
use crate::triggers::enums::WtgError::{self, UnknownProp};
use crate::triggers::enums::{ConditionType, ECAType, ParameterType, SubParameterType};

#[derive(Debug)]
pub struct ECADefinition {
    pub(super) ftype: ECAType,
    pub(super) condition_group: Option<ConditionType>,
    pub(super) name: String,
    pub(super) enabled: bool,
    pub(super) parameters: Vec<Parameter>,
    pub(super) childs_eca: Option<Vec<ECADefinition>>,
}

impl ECADefinition {
    pub fn from(
        reader: &mut BinaryReader,
        game_version: &GameVersion,
        trigger_data: &DataIni,
        is_child_eca: bool,
    ) -> Result<Self, WtgError> {
        let ftype = reader.read_u32().map_err(WtgError::ErrorReader)?;
        let ftype = ECAType::from(ftype)?;
        let condition_group = match (game_version, is_child_eca) {
            (RoC, _) | (_, false) => None,
            (_, true) => {
                let condition =
                    ConditionType::from(reader.read_u32().map_err(WtgError::ErrorReader)?)?;
                Some(condition)
            }
        };
        let name = reader
            .read_c_string_converted()
            .map_err(WtgError::ErrorReader)?;
        let eca_info = trigger_data
            .get_prop(ftype.get_sector(), &name)
            .ok_or(UnknownProp(name.clone()))?;
        let info_split = eca_info.split(",").collect::<Vec<&str>>();
        // println!("{} : {}, split : {:?}", name, eca_info, info_split);
        let count_parameters = match info_split.get(1) {
            Some(&"nothing") | None => 0,
            _ => info_split.len() - 1,
        };
        let mut parameters = vec![];
        let enabled = reader.read_u32().map_err(WtgError::ErrorReader)? == 1;
        for _ in 0..count_parameters {
            parameters.push(Parameter::from(reader, game_version, trigger_data)?);
        }
        let childs_eca = match game_version {
            RoC => None,
            _ => {
                let count_childs = reader.read_u32().map_err(WtgError::ErrorReader)?;
                let mut v = vec![];
                for _ in 0..count_childs {
                    v.push(ECADefinition::from(
                        reader,
                        game_version,
                        trigger_data,
                        true,
                    )?);
                }
                Some(v)
            }
        };
        Ok(Self {
            ftype,
            condition_group,
            name,
            enabled,
            parameters,
            childs_eca,
        })
    }

    pub fn write(
        &self,
        writer: &mut BinaryWriter,
        game_version: &GameVersion,
        trigger_data: &DataIni,
        is_child_eca: bool,
    ) -> Result<(), WtgError> {
        writer.write_u32(self.ftype as u32)?;
        match (game_version, is_child_eca) {
            (RoC, _) | (_, false) => {}
            (_, true) => {
                if let Some(condition_type) = self.condition_group {
                    writer.write_u32(condition_type as u32)?;
                }
            }
        }
        writer.write_c_string_converted(&self.name)?;
        writer.write_u32(self.enabled as u32)?;
        for param in &self.parameters {
            param.write(writer, game_version, trigger_data)?;
        }
        match game_version {
            RoC => {}
            _ => {
                if let Some(childs_eca) = &self.childs_eca {
                    writer.write_u32(childs_eca.len() as u32)?;
                    for eca in childs_eca {
                        eca.write(writer, game_version, trigger_data, true)?;
                    }
                }
            }
        };
        Ok(())
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub(super) ptype: ParameterType,
    pub(super) value: String,
    pub(super) sub_parameters: Option<SubParameters>,
    pub(super) unknown: Option<i32>,
    pub(super) array_parameter: Option<Box<Self>>,
}

impl Parameter {
    pub fn from(
        reader: &mut BinaryReader,
        game_version: &GameVersion,
        trigger_data: &DataIni,
    ) -> Result<Self, WtgError> {
        let ptype = reader.read_i32().map_err(WtgError::ErrorReader)?;
        let ptype = ParameterType::from(ptype, reader.pos())?;
        let value = reader
            .read_c_string_converted()
            .map_err(WtgError::ErrorReader)?;
        let has_sub_parameters = reader.read_u32().map_err(WtgError::ErrorReader)? == 1;
        let sub_parameters = match has_sub_parameters {
            false => None,
            true => Some(SubParameters::from(reader, game_version, trigger_data)?),
        };

        let unknown = match (game_version, ptype, has_sub_parameters) {
            (RoC, ParameterType::Function, _) => {
                Some(reader.read_i32().map_err(WtgError::ErrorReader)?)
            }
            (RoC, _, _) | (_, _, false) => None,
            (_, _, true) => Some(reader.read_i32().map_err(WtgError::ErrorReader)?),
        };

        let array_parameter = match (game_version, ptype) {
            (RoC, ParameterType::Function) => None,
            (RoC, _) | (_, _) => {
                let is_array = reader.read_u32().map_err(WtgError::ErrorReader)? == 1;
                match is_array {
                    true => {
                        let p = Parameter::from(reader, game_version, trigger_data)?;
                        Some(Box::new(p))
                    }
                    _ => None,
                }
            }
        };

        Ok(Self {
            ptype,
            value,
            sub_parameters,
            unknown,
            array_parameter,
        })
    }

    pub fn write(
        &self,
        writer: &mut BinaryWriter,
        game_version: &GameVersion,
        trigger_data: &DataIni,
    ) -> Result<(), WtgError> {
        writer.write_i32(self.ptype as i32)?;
        writer.write_c_string_converted(&self.value)?;
        writer.write_u32(self.sub_parameters.is_some() as u32)?;
        if let Some(sub_param) = &self.sub_parameters {
            sub_param.write(writer, game_version, trigger_data)?;
        }
        match (game_version, self.ptype, self.sub_parameters.is_some()) {
            (RoC, ParameterType::Function, _) => {
                writer.write_i32(self.unknown.expect("Parameter unknown value missing"))?;
            }
            (RoC, _, _) | (_, _, false) => {}
            (_, _, _) => {
                writer.write_i32(self.unknown.expect("Parameter unknown value missing"))?
            }
        }
        match (game_version, self.ptype) {
            (RoC, ParameterType::Function) => {}
            (RoC, _) | (_, _) => {
                writer.write_u32(self.array_parameter.is_some() as u32)?;
                if let Some(array_param) = &self.array_parameter {
                    array_param.write(writer, game_version, trigger_data)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct SubParameters {
    pub(super) ptype: SubParameterType,
    pub(super) name: String,
    /// On-disk `beginParameters` flag. Can be 1 with an empty `parameters`
    /// list (when trigger_data yields zero parameters for the call), so it
    /// cannot be derived from `parameters.len()` at write time.
    pub(super) begin_parameters: bool,
    pub(super) parameters: Vec<Parameter>,
}

impl SubParameters {
    pub fn from(
        reader: &mut BinaryReader,
        game_version: &GameVersion,
        trigger_data: &DataIni,
    ) -> Result<Self, WtgError> {
        let ptype = reader.read_u32().map_err(WtgError::ErrorReader)?;
        let ptype = SubParameterType::from(ptype)?;
        let name = reader
            .read_c_string_converted()
            .map_err(WtgError::ErrorReader)?;
        let info_parameters = trigger_data
            .get_prop(ptype.get_sector(), &name)
            .ok_or(WtgError::UnknownSubProp(name.clone()))?;

        let substract = match ptype {
            SubParameterType::Call => 3,
            _ => 1,
        };
        let info_split = info_parameters.split(",").collect::<Vec<&str>>();
        let count_parameters =
            if info_split.len() <= substract || info_split[substract] == "nothing" {
                0
            } else {
                info_split.len() - substract
            };
        let mut parameters = vec![];
        let begin_parameters = reader.read_u32().map_err(WtgError::ErrorReader)? != 0;
        if begin_parameters {
            for _ in 0..count_parameters {
                parameters.push(Parameter::from(reader, game_version, trigger_data)?);
            }
        }
        Ok(Self {
            ptype,
            name,
            begin_parameters,
            parameters,
        })
    }

    pub fn write(
        &self,
        writer: &mut BinaryWriter,
        game_version: &GameVersion,
        trigger_data: &DataIni,
    ) -> Result<(), WtgError> {
        writer.write_i32(self.ptype as i32)?;
        writer.write_c_string_converted(&self.name)?;
        writer.write_u32((self.begin_parameters || !self.parameters.is_empty()) as u32)?;
        for param in &self.parameters {
            param.write(writer, game_version, trigger_data)?;
        }
        Ok(())
    }
}
