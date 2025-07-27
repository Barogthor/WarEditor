use thiserror::Error;
use wce_formats::ReadError;

use crate::triggers::enums::WtgError::{
    ConditionConversionError, ECAConversionError, ParameterConversionError,
    SubParameterConversionError,
};

#[derive(Debug, Error)]
pub enum WtgError {
    #[error("Unknown parameter '{kind}' at {position}.")]
    ParameterConversionError { position: u64, kind: i32 },
    #[error("Unknown sub parameter type: '{0}'")]
    SubParameterConversionError(u32),
    #[error("Unknown function type: '{0}'")]
    ECAConversionError(u32),
    #[error("Unknown condition type: {0}")]
    ConditionConversionError(u32),
    #[error("Unknown ECA property: [{0}]")]
    UnknownProp(String),
    #[error("Unknown sub-parameter property: [{0}]")]
    UnknownSubProp(String),
    #[error("Unknown game version: '{0}'")]
    UnknownGameVersion(u32),
    #[error("Binary reader error : {0}")]
    ErrorReader(ReadError),
}
impl From<ReadError> for WtgError {
    fn from(value: ReadError) -> Self {
        Self::ErrorReader(value)
    }
}

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub enum ParameterType {
    Preset,
    Variable,
    Function,
    String,
    Invalid,
}

impl ParameterType {
    pub fn from(n: i32, bin_pos: u64) -> Result<ParameterType, WtgError> {
        match n {
            0 => Ok(ParameterType::Preset),
            1 => Ok(ParameterType::Variable),
            2 => Ok(ParameterType::Function),
            3 => Ok(ParameterType::String),
            -1 => Ok(ParameterType::Invalid),
            _ => Err(ParameterConversionError {
                position: bin_pos,
                kind: n,
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ECAType {
    Event,
    Condition,
    Action,
}
impl ECAType {
    pub fn from(n: u32) -> Result<ECAType, WtgError> {
        match n {
            0 => Ok(ECAType::Event),
            1 => Ok(ECAType::Condition),
            2 => Ok(ECAType::Action),
            _ => Err(ECAConversionError(n)),
        }
    }
    pub fn get_sector(&self) -> &str {
        match self {
            ECAType::Event => "TriggerEvents",
            ECAType::Condition => "TriggerConditions",
            ECAType::Action => "TriggerActions",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum SubParameterType {
    Event,
    Condition,
    Action,
    Call,
}
impl SubParameterType {
    pub fn from(n: u32) -> Result<SubParameterType, WtgError> {
        match n {
            0 => Ok(SubParameterType::Event),
            1 => Ok(SubParameterType::Condition),
            2 => Ok(SubParameterType::Action),
            3 => Ok(SubParameterType::Call),
            _ => Err(SubParameterConversionError(n)),
        }
    }
    pub fn get_sector(&self) -> &str {
        match self {
            SubParameterType::Event => "TriggerEvents",
            SubParameterType::Condition => "TriggerConditions",
            SubParameterType::Action => "TriggerActions",
            SubParameterType::Call => "TriggerCalls",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ConditionType {
    Condition,
    Then,
    Else,
}
impl ConditionType {
    pub fn from(n: u32) -> Result<Self, WtgError> {
        match n {
            0 => Ok(ConditionType::Condition),
            1 => Ok(ConditionType::Then),
            2 => Ok(ConditionType::Else),
            _ => Err(ConditionConversionError(n)),
        }
    }
}
