use std::io::Read;

use log::{debug, error, info, trace, warn};
use mpq::Archive;

use crate::GameData;
use crate::globals::GameVersion::{self, RoC, TFT};
use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
use crate::map_data::binary_writer::BinaryWriter;
use crate::map_data::data_ini::DataIni;
use crate::map_data::triggers::enums::{ConditionType, ECAType, ParameterType, SubParameterType};
use crate::map_data::triggers::WtgError::{self, UnknownProp};

#[derive(Debug)]
pub struct ECADefinition {
    ftype: ECAType,
    condition_group: Option<ConditionType>,
    name: String,
    enabled: bool,
    parameters: Vec<Parameter>,
    childs_eca: Option<Vec<ECADefinition>>
}

impl ECADefinition {
    pub fn from(reader: &mut BinaryReader, game_version: &GameVersion, trigger_data: &DataIni, is_child_eca: bool) -> Result<Self, WtgError>{
        let ftype = reader.read_u32();
        let ftype = ECAType::from(ftype)?;
        let condition_group = match (game_version, is_child_eca){
            (RoC, _) | (_, false)  => None,
            (_, true) => {
                let condition = ConditionType::from(reader.read_u32())?;
                Some(condition)
            }
        };
        let name = reader.read_c_string().into_string().unwrap();
        let eca_info = trigger_data.get_prop(ftype.get_sector(), &name)
            .ok_or_else(|| UnknownProp(format!("ECA Prop isnt known: [{}]", name)))?;

        let info_split = eca_info.split(",").collect::<Vec<&str>>();
        let count_parameters = match info_split[1]{
            "nothing" => 0,
            _ => info_split.len() - 1
        };
        let mut parameters = vec![];
        let enabled = reader.read_u32() == 1;
        for _ in 0..count_parameters{
            parameters.push(Parameter::from(reader, game_version, trigger_data)?);
        }
        let childs_eca = match game_version {
            (RoC) => None,
            (_) => {
                let count_childs = reader.read_u32();
                let mut v = vec![];
                for _ in 0..count_childs{
                    v.push(ECADefinition::from(reader, game_version, trigger_data, true)?);
                };
                Some(v)
            }
        };
        Ok(Self{
            ftype, condition_group, name, enabled, parameters, childs_eca
        })


    }
}

#[derive(Debug)]
pub struct Parameter {
    ptype: ParameterType,
    value: String,
    sub_parameters: Option<SubParameters>,
    unknown: Option<i32>,
    array_parameter: Option<Box<Self>>,
}

impl Parameter {
    pub fn from(reader: &mut BinaryReader, game_version: &GameVersion, trigger_data: &DataIni) -> Result<Self, WtgError>{
        let ptype = reader.read_i32();
        let ptype = ParameterType::from(ptype)?;
        let value = reader.read_c_string().into_string().unwrap();
        let has_sub_parameters = reader.read_u32() == 1;
        let sub_parameters = match has_sub_parameters {
            false => None,
            true => Some(SubParameters::from(reader, game_version, trigger_data)?)
        };

        let unknown = match (game_version, ptype, has_sub_parameters) {
            (RoC, ParameterType::FUNCTION, _) => Some(reader.read_i32()),
            (RoC, _, _) | (_, _, false) => None,
            (_, _, true) => Some(reader.read_i32())
        };

        let array_parameter = match (game_version, ptype){
            (RoC, ParameterType::FUNCTION) => None,
            (RoC, _) | (_, _) => {
                let is_array = reader.read_u32() == 1;
                match is_array{
                    true => {
                        let p = Parameter::from(reader, game_version, trigger_data).unwrap();
                        Some(Box::new(p))
                    },
                    _ => None,
                }
            }
        };

        Ok(Self{
            ptype, value, sub_parameters, unknown, array_parameter
        })
    }
}

#[derive(Debug)]
pub struct SubParameters {
    ptype: SubParameterType,
    name: String,
    parameters: Vec<Parameter>,
}

impl SubParameters {
    pub fn from(reader: &mut BinaryReader, game_version: &GameVersion, trigger_data: &DataIni) -> Result<Self, WtgError>{
        let ptype = reader.read_u32();
        let ptype = SubParameterType::from(ptype)?;
        let name = reader.read_c_string().into_string().unwrap();
        let info_parameters = trigger_data.get_prop(ptype.get_sector(), &name)
            .ok_or_else(|| UnknownProp(format!("Sub parameter prop isnt known: [{}]", name)))?;

        let substract = match ptype{
            SubParameterType::Call => 3,
            _ => 1
        };
        let info_split = info_parameters.split(",").collect::<Vec<&str>>();
        let count_parameters = if info_split.len() <=substract || info_split[substract] == "nothing"{
             0
        } else {
            info_split.len() - substract
        };
        let mut parameters = vec![];
        // if count_parameters > 0 {
            let begin_parameters = reader.read_u32() != 0;
            if begin_parameters {
                for _ in 0..count_parameters {
                    parameters.push(Parameter::from(reader, game_version, trigger_data)?);
                }
            }
        // }
        Ok(Self{
            ptype, name, parameters
        })
    }
}

