use std::fs::File;
use byteorder::{ReadBytesExt, LittleEndian, BigEndian};
use std::ffi::{CString, CStr, NulError};
use std::io::{Seek, SeekFrom, Read};
use std::error::Error;
use war_editor::map_data::w3i_file::W3iFile;
//use byteorder::{LittleEndian,BigEndian,ReadBytesExt};
//use std::io::Cursor;

struct Unit {
}

fn read_w3u_file(){
    let mut f = File::open("resources/war3map.w3u").unwrap();
    let version = f.read_i32::<LittleEndian>().unwrap();
    let num_origin_units = f.read_i32::<LittleEndian>().unwrap();
    let num_custom_units = f.read_i32::<LittleEndian>().unwrap();
    println!("version {}", version);
    println!("num origin units {}", num_origin_units);
    println!("num origin units {}", num_custom_units);
}


fn main() {
    println!("Hello, world!");
    W3iFile::read_file();

}
