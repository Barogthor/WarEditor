use std::ffi::CString;
use std::fmt::Debug;

use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::{GameVersion, ReadError};

pub mod ability;
pub mod buff;
pub mod destructable;
pub mod doodad;
pub mod item;
pub mod unit;
pub mod upgrade;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct ObjectIdCode(pub [u8; 4]);
#[derive(Debug)]
pub struct MetaId([u8; 4]);
pub type OriginalIdCode = ObjectIdCode;
pub type CustomIdCode = ObjectIdCode;

#[derive(Debug)]
pub enum VariableValue {
    Integer(i32),
    Real(f32),
    Unreal(f32),
    String(String),
    Bool(bool),
    Char(char),
    UnitList(String),
    ItemList(String),
    RegenType(String),
    AttackType(String),
    WeaponType(String),
    TargetType(String),
    MoveType(String),
    DefenseType(String),
    PathingTexture(String),
    UpgradeList(String),
    StringList(String),
    AbilityList(String),
    HeroAbilityList(String),
    MissileArt(String),
    AttributeType(String),
    AttackBits(String),
}

#[derive(Debug)]
pub enum ObjectId {
    Original(OriginalIdCode),
    Custom(OriginalIdCode, CustomIdCode),
}

impl ObjectId {
    pub fn for_custom(original_id: [u8; 4], custom_id: [u8; 4]) -> Self {
        Self::Custom(ObjectIdCode(original_id), ObjectIdCode(custom_id))
    }
    pub fn for_original(original_id: [u8; 4]) -> Self {
        Self::Original(ObjectIdCode(original_id))
    }
}

#[derive(Debug)]
pub struct MetaModification {
    id: MetaId,
    value: VariableValue,
    level: i32,
    data_pointer: i32,
}

#[derive(Debug)]
pub struct ObjectDefinition {
    id: ObjectId,
    modified_datas: Vec<MetaModification>,
}

impl ObjectDefinition {
    pub fn with_optional(reader: &mut BinaryReader, id: ObjectId) -> ReadResult<Self> {
        let modif_count = reader.read_u32()?;
        let mut meta_modified = vec![];
        for _i in 0..modif_count {
            let meta = read_meta_opts(reader, &id)?;
            meta_modified.push(meta);
        }
        Ok(Self {
            id,
            modified_datas: meta_modified,
        })
    }
    pub fn without_optional(
        reader: &mut BinaryReader,
        id: ObjectId,
        game_version: &GameVersion,
    ) -> ReadResult<Self> {
        let modif_count = reader.read_u32()?;
        let mut meta_modified = vec![];
        for _i in 0..modif_count {
            let meta = read_meta_no_opts(reader, &id, game_version)?;
            meta_modified.push(meta);
        }
        Ok(Self {
            id,
            modified_datas: meta_modified,
        })
    }
}

fn cstring_to_string_meta(
    cstr: ReadResult<CString>,
    id: &ObjectId,
    meta_id: &[u8; 4],
) -> ReadResult<String> {
    let cstr = cstr?;
    cstr.into_string().map_err(|e| {
        ReadError::InvalidCString(format!(
            "Invalid object '{id:?}' of meta '{}'. Reason '{e}",
            String::from_utf8_lossy(meta_id)
        ))
    })
}

fn read_meta_no_opts(
    reader: &mut BinaryReader,
    id: &ObjectId,
    game_version: &GameVersion,
) -> ReadResult<MetaModification> {
    let meta_id = reader.read_bytes(4)?;
    let meta_id = [meta_id[0], meta_id[1], meta_id[2], meta_id[3]];
    let vtype = reader.read_i32()?;

    let value = match (game_version, vtype) {
        (_, 0) => Ok(VariableValue::Integer(reader.read_i32()?)),
        (_, 1) => Ok(VariableValue::Real(reader.read_f32()?)),
        (_, 2) => Ok(VariableValue::Unreal(reader.read_f32()?)),
        (_, 3) => Ok(VariableValue::String(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 4) => Ok(VariableValue::Bool(reader.read_u8()? == 1)),
        (GameVersion::RoC, 5) => Ok(VariableValue::Char(reader.read_char()?)),
        (GameVersion::RoC, 6) => Ok(VariableValue::UnitList(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 7) => Ok(VariableValue::ItemList(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 8) => Ok(VariableValue::RegenType(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 9) => Ok(VariableValue::AttackType(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 10) => Ok(VariableValue::WeaponType(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 11) => Ok(VariableValue::TargetType(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 12) => Ok(VariableValue::MoveType(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 13) => Ok(VariableValue::DefenseType(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 14) => Ok(VariableValue::PathingTexture(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 15) => Ok(VariableValue::UpgradeList(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 16) => Ok(VariableValue::StringList(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 17) => Ok(VariableValue::AbilityList(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 18) => Ok(VariableValue::HeroAbilityList(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 19) => Ok(VariableValue::MissileArt(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 20) => Ok(VariableValue::AttributeType(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        (GameVersion::RoC, 21) => Ok(VariableValue::AttackBits(cstring_to_string_meta(
            reader.read_c_string(),
            id,
            &meta_id,
        )?)),
        _ => Err(ReadError::Reason(format!(
            "Unsupported vtype '{vtype}' for object {id:?} on meta '{}'",
            String::from_utf8_lossy(&meta_id)
        ))),
    }?;
    reader.skip(4);
    Ok(MetaModification {
        id: MetaId(meta_id),
        value,
        level: 0,
        data_pointer: 0,
    })
}

fn read_meta_opts(reader: &mut BinaryReader, id: &ObjectId) -> ReadResult<MetaModification> {
    let meta_id = reader.read_bytes(4)?;
    let meta_id = [meta_id[0], meta_id[1], meta_id[2], meta_id[3]];
    let vtype = reader.read_i32()?;
    let level = reader.read_i32()?;
    let data_pointer = reader.read_i32()?;
    let value = match vtype {
        0 => Ok(VariableValue::Integer(reader.read_i32()?)),
        1 => Ok(VariableValue::Real(reader.read_f32()?)),
        2 => Ok(VariableValue::Unreal(reader.read_f32()?)),
        3 => Ok(VariableValue::String(
            reader.read_c_string()?.into_string().map_err(|_| {
                ReadError::Reason(format!(
                    "Failed to read cstring for object '{id:?}' of meta '{}' (byte position {})",
                    String::from_utf8_lossy(&meta_id),
                    reader.pos()
                ))
            })?,
        )),
        _ => Err(ReadError::Reason(format!(
            "Unsupported vtype '{vtype}' for object {id:?} on meta '{}'",
            String::from_utf8_lossy(&meta_id)
        ))),
    }?;
    reader.skip(4);
    Ok(MetaModification {
        id: MetaId(meta_id),
        value,
        level,
        data_pointer,
    })
}

fn assert_meta_end_format(reader: &BinaryReader, id: &ObjectId, end_meta_id: Vec<u8>) {
    let end_format_zero = true;
    match (end_format_zero, id) {
        (false,ObjectId::Original(code)) => assert_eq!(code.0, end_meta_id.as_slice(),
                                                       "format reading went wrong meta object end '{}' not equal to object id '{}' (byte position {})",
                                                       String::from_utf8_lossy(end_meta_id.as_slice()), String::from_utf8_lossy(&code.0), reader.pos()),
        (false,ObjectId::Custom(_, code)) => assert_eq!(code.0, end_meta_id.as_slice(),
                                                        "format reading went wrong meta object end '{}' not equal to object id '{}' (byte position {})",
                                                        String::from_utf8_lossy(end_meta_id.as_slice()), String::from_utf8_lossy(&code.0), reader.pos()),
        _ => ()
    }
}
