use std::convert::TryFrom;
use std::ffi::CString;
use std::fmt::Debug;
use std::marker::PhantomData;

use wce_formats::binary_reader::{BinaryReader, ReadResult};
use wce_formats::binary_writer::{BinaryWriter, WriteResult};
use wce_formats::{GameVersion, MapArchive, ReadError, WriteError};

use crate::MapError;

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

impl VariableValue {
    pub fn get_type_value(&self) -> i32 {
        match self {
            VariableValue::Integer(_) => 0,
            VariableValue::Real(_) => 1,
            VariableValue::Unreal(_) => 2,
            VariableValue::String(_) => 3,
            VariableValue::Bool(_) => 4,
            VariableValue::Char(_) => 5,
            VariableValue::UnitList(_) => 6,
            VariableValue::ItemList(_) => 7,
            VariableValue::RegenType(_) => 8,
            VariableValue::AttackType(_) => 9,
            VariableValue::WeaponType(_) => 10,
            VariableValue::TargetType(_) => 11,
            VariableValue::MoveType(_) => 12,
            VariableValue::DefenseType(_) => 13,
            VariableValue::PathingTexture(_) => 14,
            VariableValue::UpgradeList(_) => 15,
            VariableValue::StringList(_) => 16,
            VariableValue::AbilityList(_) => 17,
            VariableValue::HeroAbilityList(_) => 18,
            VariableValue::MissileArt(_) => 19,
            VariableValue::AttributeType(_) => 20,
            VariableValue::AttackBits(_) => 21,
        }
    }
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
    pub fn read_with_optional(reader: &mut BinaryReader, id: ObjectId) -> ReadResult<Self> {
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
    pub fn read_without_optional(
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

    pub fn write_without_optional(
        &self,
        writer: &mut BinaryWriter,
        game_version: &GameVersion,
    ) -> WriteResult<()> {
        match self.id {
            ObjectId::Original(original_id) => {
                writer.write_bytes(original_id.0.as_slice())?;
                writer.write_bytes(&[0; 4])?;
            }
            ObjectId::Custom(original_id, custom_id) => {
                writer.write_bytes(original_id.0.as_slice())?;
                writer.write_bytes(custom_id.0.as_slice())?;
            }
        }
        let modif_count = self.modified_datas.len();
        writer.write_u32(modif_count as u32)?;
        for i in 0..modif_count {
            write_meta_no_opts(
                writer,
                &self.id,
                self.modified_datas.get(i).unwrap(),
                game_version,
            )?;
        }
        Ok(())
    }

    pub fn write_with_optional(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        match self.id {
            ObjectId::Original(original_id) => {
                writer.write_bytes(original_id.0.as_slice())?;
                writer.write_bytes(&[0; 4])?;
            }
            ObjectId::Custom(original_id, custom_id) => {
                writer.write_bytes(original_id.0.as_slice())?;
                writer.write_bytes(custom_id.0.as_slice())?;
            }
        }
        let modif_count = self.modified_datas.len();
        writer.write_u32(modif_count as u32)?;
        for i in 0..modif_count {
            write_meta_opts(writer, &self.id, self.modified_datas.get(i).unwrap())?;
        }
        Ok(())
    }
}

/// One custom-object table kind (`war3map.w3a`, `.w3u`, …): archive file
/// name, on-disk entry layout, and mapping into the kind's `MapError` variant.
pub trait CustomObjectKind: Debug {
    /// Archive file name (e.g. `war3map.w3a`).
    const FILE_NAME: &'static str;
    /// `true` when entries carry the level/data-pointer fields
    /// (`*_with_optional` layout: abilities, doodads, upgrades).
    const HAS_LEVEL_DATA: bool;
    /// Wrap a reader-initialization failure into this kind's `MapError`.
    fn init_error(e: ReadError) -> MapError;
    /// Wrap a parsing failure into this kind's `MapError`.
    fn parsing_error(e: ReadError) -> MapError;
    /// Wrap a serialization failure into this kind's `MapError`.
    fn save_error(e: WriteError) -> MapError;
}

/// Generic reader/writer shared by all seven custom-object tables; the
/// per-kind differences live entirely in [`CustomObjectKind`].
#[derive(Debug)]
pub struct CustomObjectsFile<K: CustomObjectKind> {
    version: u32,
    original_objects: Vec<ObjectDefinition>,
    custom_objects: Vec<ObjectDefinition>,
    _kind: PhantomData<K>,
}

impl<K: CustomObjectKind> CustomObjectsFile<K> {
    pub const FILE_NAME: &'static str = K::FILE_NAME;

    /// Read this kind's table from the map archive; `Ok(None)` when the file
    /// is absent or empty.
    pub fn read_file(
        map: &mut MapArchive,
        game_version: &GameVersion,
    ) -> Result<Option<Self>, MapError> {
        match map.read_file(K::FILE_NAME) {
            Ok(buffer) => {
                let mut reader = BinaryReader::try_from(buffer).map_err(K::init_error)?;
                Self::read_opt(&mut reader, game_version)
            }
            _ => Ok(None),
        }
    }

    fn read_opt(
        reader: &mut BinaryReader,
        game_version: &GameVersion,
    ) -> Result<Option<Self>, MapError> {
        if reader.size() > 0 {
            let parsed = Self::parse(reader, game_version).map_err(K::parsing_error)?;
            Ok(Some(parsed))
        } else {
            Ok(None)
        }
    }

    fn parse(reader: &mut BinaryReader, game_version: &GameVersion) -> ReadResult<Self> {
        let version = reader.read_u32()?;
        let original_count = reader.read_u32()?;
        let mut original_objects = vec![];
        for _ in 0..original_count {
            original_objects.push(Self::read_object(reader, game_version)?);
        }
        let custom_count = reader.read_u32()?;
        let mut custom_objects = vec![];
        for _ in 0..custom_count {
            custom_objects.push(Self::read_object(reader, game_version)?);
        }
        if reader.size() != reader.pos() as usize {
            return Err(ReadError::Reason(format!(
                "reader for {} hasn't reached EOF. Missing {} bytes",
                K::FILE_NAME,
                reader.size() - reader.pos() as usize
            )));
        }
        Ok(Self {
            version,
            original_objects,
            custom_objects,
            _kind: PhantomData,
        })
    }

    fn read_object(
        reader: &mut BinaryReader,
        game_version: &GameVersion,
    ) -> ReadResult<ObjectDefinition> {
        let original_id = reader.read_bytes(4)?;
        let original_id = [
            original_id[0],
            original_id[1],
            original_id[2],
            original_id[3],
        ];
        let custom_id = reader.read_bytes(4)?;
        let id = if custom_id.iter().all(|c| *c == 0) {
            ObjectId::for_original(original_id)
        } else {
            ObjectId::for_custom(
                original_id,
                [custom_id[0], custom_id[1], custom_id[2], custom_id[3]],
            )
        };
        if K::HAS_LEVEL_DATA {
            ObjectDefinition::read_with_optional(reader, id)
        } else {
            ObjectDefinition::read_without_optional(reader, id, game_version)
        }
    }

    /// Serialize this table into a fresh writer; an empty table (no original
    /// nor custom object) produces an empty buffer.
    pub fn prepare_write(&self, game_version: &GameVersion) -> Result<BinaryWriter, MapError> {
        let mut writer = BinaryWriter::new();
        self.write(&mut writer, game_version)
            .map_err(K::save_error)?;
        Ok(writer)
    }

    fn write(&self, writer: &mut BinaryWriter, game_version: &GameVersion) -> WriteResult<()> {
        if self.original_objects.is_empty() && self.custom_objects.is_empty() {
            return Ok(());
        }
        writer.write_u32(self.version)?;
        writer.write_u32(self.original_objects.len() as u32)?;
        for obj in &self.original_objects {
            Self::write_object(writer, obj, game_version)?;
        }
        writer.write_u32(self.custom_objects.len() as u32)?;
        for obj in &self.custom_objects {
            Self::write_object(writer, obj, game_version)?;
        }
        Ok(())
    }

    fn write_object(
        writer: &mut BinaryWriter,
        obj: &ObjectDefinition,
        game_version: &GameVersion,
    ) -> WriteResult<()> {
        if K::HAS_LEVEL_DATA {
            obj.write_with_optional(writer)
        } else {
            obj.write_without_optional(writer, game_version)
        }
    }

    pub fn debug(&self) {
        println!("{self:#?}");
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

fn write_meta_no_opts(
    writer: &mut BinaryWriter,
    id: &ObjectId,
    meta: &MetaModification,
    game_version: &GameVersion,
) -> WriteResult<()> {
    writer.write_bytes(meta.id.0.as_slice())?;
    writer.write_i32(meta.value.get_type_value())?;
    match (game_version, &meta.value) {
        (_, VariableValue::Integer(int)) => writer.write_i32(*int)?,
        (_, VariableValue::Real(real)) => writer.write_f32(*real)?,
        (_, VariableValue::Unreal(ureal)) => writer.write_f32(*ureal)?,
        (_, VariableValue::String(s)) => writer.write_c_string_converted(&s)?,
        (GameVersion::RoC, VariableValue::Bool(b)) => writer.write_u8(*b as u8)?,
        (GameVersion::RoC, VariableValue::Char(c)) => writer.write_char(*c)?,
        (GameVersion::RoC, VariableValue::UnitList(s)) => writer.write_c_string_converted(s)?,
        (GameVersion::RoC, VariableValue::ItemList(s)) => writer.write_c_string_converted(s)?,
        (GameVersion::RoC, VariableValue::RegenType(s)) => writer.write_c_string_converted(s)?,
        (GameVersion::RoC, VariableValue::AttackType(s)) => writer.write_c_string_converted(s)?,
        (GameVersion::RoC, VariableValue::WeaponType(s)) => writer.write_c_string_converted(s)?,
        (GameVersion::RoC, VariableValue::TargetType(s)) => writer.write_c_string_converted(s)?,
        (GameVersion::RoC, VariableValue::MoveType(s)) => writer.write_c_string_converted(s)?,
        (GameVersion::RoC, VariableValue::DefenseType(s)) => writer.write_c_string_converted(s)?,
        (GameVersion::RoC, VariableValue::PathingTexture(s)) => {
            writer.write_c_string_converted(s)?
        }
        (GameVersion::RoC, VariableValue::UpgradeList(s)) => writer.write_c_string_converted(s)?,
        (GameVersion::RoC, VariableValue::StringList(s)) => writer.write_c_string_converted(s)?,
        (GameVersion::RoC, VariableValue::AbilityList(s)) => writer.write_c_string_converted(s)?,
        (GameVersion::RoC, VariableValue::HeroAbilityList(s)) => {
            writer.write_c_string_converted(s)?
        }
        (GameVersion::RoC, VariableValue::MissileArt(s)) => writer.write_c_string_converted(s)?,
        (GameVersion::RoC, VariableValue::AttributeType(s)) => {
            writer.write_c_string_converted(s)?
        }
        (GameVersion::RoC, VariableValue::AttackBits(s)) => writer.write_c_string_converted(s)?,
        (_, vv) => {
            return Err(WriteError::Reason(format!(
                "Unsupported vtype '{}' for object {id:?} on meta '{}' version '{:?}'",
                vv.get_type_value(),
                String::from_utf8_lossy(&meta.id.0),
                game_version
            )))
        }
    };

    writer.write_u32(0)?;
    Ok(())
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

pub fn write_meta_opts(
    writer: &mut BinaryWriter,
    id: &ObjectId,
    meta: &MetaModification,
) -> WriteResult<()> {
    writer.write_bytes(meta.id.0.as_slice())?;
    writer.write_i32(meta.value.get_type_value())?;
    writer.write_i32(meta.level)?;
    writer.write_i32(meta.data_pointer)?;
    match &meta.value {
        VariableValue::Integer(int) => writer.write_i32(*int)?,
        VariableValue::Real(real) => writer.write_f32(*real)?,
        VariableValue::Unreal(ureal) => writer.write_f32(*ureal)?,
        VariableValue::String(s) => writer.write_c_string_converted(s)?,
        vv => {
            return Err(WriteError::Reason(format!(
                "Unsupported vtype '{}' for object {id:?} on meta '{}'",
                vv.get_type_value(),
                String::from_utf8_lossy(&meta.id.0),
            )))
        }
    };
    writer.write_u32(0)?;
    Ok(())
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
