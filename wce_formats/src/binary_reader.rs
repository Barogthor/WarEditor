use std::ffi::CString;
use std::fs::File;
use std::io::{BufRead, Cursor, Read, Seek, SeekFrom};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::{BinaryConverter, BinaryConverterVersion, GameVersion};

pub struct BinaryReader{
    buffer: Cursor<Vec<u8>>,
    size: usize,
}

impl BinaryReader{
    pub fn new(buffer: Vec<u8>) -> BinaryReader{
        BinaryReader{size:buffer.len(), buffer: Cursor::new(buffer)}
    }

    pub fn from(file: &mut File) -> BinaryReader{
        let mut buffer: Vec<u8> = vec![];
        file.read_to_end(&mut buffer).unwrap();
        BinaryReader{
            size: buffer.len(), buffer: Cursor::new(buffer)
        }
    }

    pub fn read_char(&mut self) -> char{
        char::from(self.read_u8())
    }
    pub fn read_u8(&mut self) -> u8{
        self.buffer.read_u8().unwrap()
    }

    pub fn read_i16(&mut self) -> i16{
        self.buffer.read_i16::<LittleEndian>().unwrap()
    }
    pub fn read_u16(&mut self) -> u16{
        self.buffer.read_u16::<LittleEndian>().unwrap()
    }

    pub fn read_i24(&mut self) -> i32{
        self.buffer.read_i24::<LittleEndian>().unwrap()
    }
    pub fn read_u24(&mut self) -> u32{
        self.buffer.read_u24::<LittleEndian>().unwrap()
    }

    pub fn read_i32(&mut self) -> i32{
        self.buffer.read_i32::<LittleEndian>().unwrap()
    }
    pub fn read_i32_big(&mut self) -> i32{
        self.buffer.read_i32::<BigEndian>().unwrap()
    }
    pub fn read_u32(&mut self) -> u32{
        self.buffer.read_u32::<LittleEndian>().unwrap()
    }
    pub fn read_u32_big(&mut self) -> u32{
        self.buffer.read_u32::<BigEndian>().unwrap()
    }

    pub fn read_u64(&mut self) -> u64{
        self.buffer.read_u64::<LittleEndian>().unwrap()
    }

    pub fn read_f32(&mut self) -> f32{
        self.buffer.read_f32::<LittleEndian>().unwrap()
    }
    pub fn read_f64(&mut self) -> f64{
        self.buffer.read_f64::<LittleEndian>().unwrap()
    }

    pub fn read_c_string(&mut self) -> CString{
        let mut result_buf: Vec<u8> = Vec::new();
        self.buffer.read_until('\0' as u8, &mut result_buf).unwrap();
        result_buf.pop();
        CString::new(result_buf).unwrap()
    }
    pub fn read_c_string_sized(&mut self, size: usize) -> CString{
        let v = self.read_bytes(size);
//        println!("pos: {}",self.pos());
        CString::new(v).unwrap()
    }

    pub fn read_string_utf8(&mut self, bytes_to_read: usize) -> String{
        let v = self.read_bytes(bytes_to_read);
        String::from_utf8(v).expect(&format!("Error around byte : {}", self.pos()-(bytes_to_read as u64)))
    }


    pub fn read_chars(&mut self, size: usize) -> Vec<char>{
        let mut chars = Vec::new();
        for _i in 0..size{
            chars.push(self.read_char());
        }
        chars
    }

    pub fn skip(&mut self, bytes_to_skip: i64){
        self.buffer.seek(SeekFrom::Current(bytes_to_skip)).unwrap();
    }

    pub fn read<T: BinaryConverter>(&mut self) -> T{
        T::read(self)
    }

    pub fn read_version<T: BinaryConverterVersion>(&mut self, game_version: &GameVersion) -> T{
        T::read_version(self, game_version)
    }

    pub fn read_vec<T: BinaryConverter>(&mut self, size: usize) -> Vec<T>{
        let mut vec: Vec<T> = vec![];
        for _i in 0..size{
            vec.push(T::read(self));
        }
        vec
    }

    pub fn read_vec_version<T: BinaryConverterVersion>(&mut self, size: usize, game_version: &GameVersion) -> Vec<T>{
        let mut vec: Vec<T> = vec![];
        for _i in 0..size{
            vec.push(T::read_version(self, game_version));
        }
        vec
    }

    pub fn read_vec_i32(&mut self, size: usize) -> Vec<i32>{
        let mut vec: Vec<i32> = vec![];
        for _i in 0..size{
            vec.push(self.read_i32());
        }
        vec
    }

    pub fn read_vec_u32(&mut self, size: usize) -> Vec<u32>{
        let mut vec: Vec<u32> = vec![];
        for _i in 0..size{
            let v = self.read_u32();
            vec.push(v);
        }
        vec
    }

    pub fn read_vec_u32_be(&mut self, size: usize) -> Vec<u32>{
        let mut vec: Vec<u32> = vec![];
        for _i in 0..size{
            let v = self.read_u32_big();
            vec.push(v);
        }
        vec
    }

    pub fn read_vec_f32(&mut self, size: usize) -> Vec<f32>{
        let mut vec: Vec<f32> = vec![];
        for _i in 0..size{
            vec.push(self.read_f32());
        }
        vec
    }

    pub fn read_bytes(&mut self, size: usize) -> Vec<u8>{
        let mut vec: Vec<u8> = vec![];
        for _i in 0..size{
            vec.push(self.read_u8());
        }
        vec
    }
    pub fn seek_begin(&mut self){
        self.buffer.seek(SeekFrom::Start(0)).expect("Failed to move cursor at start.");
    }

    pub fn pos(&self) -> u64{
        self.buffer.position()
    }
    pub fn size(&self) -> usize{
        self.size
    }

}

