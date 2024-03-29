use crate::OpeningError;
use crate::triggers::enums::WtgError::{ConditionConversionError, ECAConversionError, ParameterConversionError, SubParameterConversionError};

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum WtgError {
    ParameterConversionError(String),
    SubParameterConversionError(String),
    ECAConversionError(String),
    ConditionConversionError(String),
    WtgParsingIsntCompleteError(String),
    UnknownProp(String),
    UnknownGameVersion(String)
}

impl Into<OpeningError> for WtgError {
    fn into(self) -> OpeningError {
        OpeningError::Triggers(format!("{:?}", self))
    }
}


#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub enum ParameterType {
    PRESET,
    VARIABLE,
    FUNCTION,
    STRING,
    INVALID,
}

impl ParameterType {
    pub fn from(n: i32, bin_pos: u64) -> Result<ParameterType, WtgError> {
        match n{
            0 => Ok(ParameterType::PRESET),
            1 => Ok(ParameterType::VARIABLE),
            2 => Ok(ParameterType::FUNCTION),
            3 => Ok(ParameterType::STRING),
            // -1 => {
            //     info!("Parameter type invalid was found");
            //     Ok(ParameterType::INVALID)
            // },
            _ => Err(ParameterConversionError(format!("Failure on byte '{}' : Unknown Parameter type {} was found", bin_pos, n)))

        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ECAType {
    Event,
    Condition,
    Action
}
impl ECAType {
    pub fn from(n: u32) -> Result<ECAType, WtgError> {
        match n{
            0 => Ok(ECAType::Event),
            1 => Ok(ECAType::Condition),
            2 => Ok(ECAType::Action),
            _ => Err(ECAConversionError(format!("Unknown function type {}",n)))
        }
    }
    pub fn get_sector(&self) -> &str {
        match self{
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
    Call
}
impl SubParameterType {
    pub fn from(n: u32) -> Result<SubParameterType, WtgError> {
        match n{
            0 => Ok(SubParameterType::Event),
            1 => Ok(SubParameterType::Condition),
            2 => Ok(SubParameterType::Action),
            3 => Ok(SubParameterType::Call),
            _ => Err(SubParameterConversionError(format!("Unknown sub parameter type {}",n)))
        }
    }
    pub fn get_sector(&self) -> &str {
        match self{
            SubParameterType::Event => "TriggerEvents",
            SubParameterType::Condition => "TriggerConditions",
            SubParameterType::Action => "TriggerActions",
            SubParameterType::Call => "TriggerCalls",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ConditionType{
    Condition,
    Then,
    Else
}
impl ConditionType{
    pub fn from(n: u32) -> Result<Self, WtgError> {
        match n{
            0 => Ok(ConditionType::Condition),
            1 => Ok(ConditionType::Then),
            2 => Ok(ConditionType::Else),
            _ => Err(ConditionConversionError(format!("Unknown Condition type {}", n)))
        }
    }
}