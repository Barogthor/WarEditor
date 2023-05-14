use std::fmt::Debug;
use std::marker::PhantomData;

use wce_formats::binary_reader::BinaryReader;
use wce_formats::binary_writer::BinaryWriter;
use wce_formats::GameVersion;

pub mod unit;
pub mod ability;
pub mod item;
pub mod destructable;
pub mod buff;
pub mod doodad;
pub mod upgrade;

pub trait UseOptionalInts: Debug {}
#[derive(Default, Debug)]
pub struct NeedInts;
#[derive(Default, Debug)]
pub struct Absent;
impl UseOptionalInts for NeedInts {}
impl UseOptionalInts for Absent {}


#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct ObjectIdCode(pub [u8;4]);
#[derive(Debug)]
pub struct MetaId([u8;4]);
pub type OriginalIdCode = ObjectIdCode;
pub type CustomIdCode = ObjectIdCode;

#[derive(Debug)]
pub enum VariableValueTft {
    Integer(i32),
    Real(f32),
    Unreal(f32),
    String(String)
}


//
// pub enum VariableValueRoc {
//     Integer(i32),
//     Real(f32),
//     Unreal(f32),
//     String(String),
//     Bool(bool),
//     UnitList(String),
//     ItemList(String),
//     RegenType(String),
//     AttackType(String),
//     WeaponType(String),
//     TargetType(String),
//     MoveType(String),
//     DefenseType(String),
//     PathingTexture(String),
//     UpgradeList(String),
//     StringList(String),
//     AbilityList(String),
//     HeroAbilityList(String),
//     MissileArt(String),
//     AttributeType(String),
//     AttackBits(String),
// }

#[derive(Debug)]
pub enum ObjectId{
    Original(OriginalIdCode),
    Custom(OriginalIdCode, CustomIdCode)
}

impl ObjectId {
    pub fn for_custom(original_id: [u8;4], custom_id: [u8;4]) -> Self {
        Self::Custom(ObjectIdCode(original_id),ObjectIdCode(custom_id))
    }
    pub fn for_original(original_id: [u8;4]) -> Self {
        Self::Original(ObjectIdCode(original_id))
    }
}

#[derive(Debug)]
pub struct MetaModification {
    id: MetaId,
    value: VariableValueTft,
    level: i32,
    data_pointer: i32
}


// pub trait MetaModicationBinConverter {
//     fn read(reader: &mut BinaryReader, id: ObjectId, game_data: &GameData, version: GameVersion) -> Self;
//     fn write(writer: &mut BinaryWriter, id: ObjectId, game_data: &GameData, version: GameVersion);
// }

#[derive(Debug)]
pub struct ObjectDefinition {
    id: ObjectId,
    modified_datas: Vec<MetaModification>
}

impl ObjectDefinition {
    pub fn with_optional(reader: &mut BinaryReader, id: ObjectId, game_version: &GameVersion) -> Self {
        let modif_count = reader.read_u32();
        let mut meta_modified = vec![];
        for _i in 0..modif_count {
            let meta = read_meta_opts(reader, &id);
            meta_modified.push(meta);
        }
        Self {
            id,
            modified_datas: meta_modified,
        }
    }
    pub fn without_optional(reader: &mut BinaryReader, id: ObjectId, game_version: &GameVersion) -> Self {
        let modif_count = reader.read_u32();
        let mut meta_modified = vec![];
        for _i in 0..modif_count {
            let meta = read_meta_no_opts(reader, &id);
            meta_modified.push(meta);
        }
        Self {
            id,
            modified_datas: meta_modified,
        }
    }
}

fn read_meta_no_opts(reader: &mut BinaryReader, id: &ObjectId) -> MetaModification {
    let meta_id = reader.read_bytes(4);
    let meta_id = [meta_id[0],meta_id[1],meta_id[2],meta_id[3]];
    let vtype = reader.read_i32();
    let value = match vtype {
        0 => VariableValueTft::Integer(reader.read_i32()),
        1 => VariableValueTft::Real(reader.read_f32()),
        2 => VariableValueTft::Unreal(reader.read_f32()),
        3 => VariableValueTft::String(reader.read_c_string().into_string().expect(
            &format!("Failed to read cstring for object '{:?}' of meta '{}'", id, String::from_utf8_lossy(&meta_id) )
        )),
        _ => panic!("Unsupported vtype '{}' for object {:?} on meta '{}'",vtype, id, String::from_utf8_lossy(&meta_id) )
    };
    reader.skip(4);
    MetaModification {
        id: MetaId(meta_id),
        value,
        level: 0,
        data_pointer: 0,
    }
}

fn read_meta_opts(reader: &mut BinaryReader, id: &ObjectId) -> MetaModification {
    let meta_id = reader.read_bytes(4);
    let meta_id = [meta_id[0],meta_id[1],meta_id[2],meta_id[3]];
    let vtype = reader.read_i32();
    let level = reader.read_i32();
    let data_pointer = reader.read_i32();
    let value = match vtype {
        0 => VariableValueTft::Integer(reader.read_i32()),
        1 => VariableValueTft::Real(reader.read_f32()),
        2 => VariableValueTft::Unreal(reader.read_f32()),
        3 => VariableValueTft::String(reader.read_c_string().into_string().expect(
            &format!("Failed to read cstring for object '{:?}' of meta '{}' (byte position {})", id, String::from_utf8_lossy(&meta_id), reader.pos() )
        )),
        _ => panic!("Unsupported vtype '{}' for object {:?} on meta '{}' (byte position {})",vtype, id, String::from_utf8_lossy(&meta_id), reader.pos() )
    };
    reader.skip(4);
    MetaModification {
        id: MetaId(meta_id),
        value,
        level,
        data_pointer,
    }
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