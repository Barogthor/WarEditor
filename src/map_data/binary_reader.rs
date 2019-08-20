use std::io::{Cursor, BufRead, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use std::mem::size_of;
use std::ffi::{CString, CStr};

pub struct BinaryReader{
    buffer: Cursor<Vec<u8>>
}

impl BinaryReader{
    pub fn new(buffer: Vec<u8>) -> BinaryReader{
        BinaryReader{buffer: Cursor::new(buffer)}
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

    pub fn read_i32(&mut self) -> i32{
        self.buffer.read_i32::<LittleEndian>().unwrap()
    }

    pub fn read_u32(&mut self) -> u32{
        self.buffer.read_u32::<LittleEndian>().unwrap()
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
        self.buffer.read_until('\0' as u8, &mut result_buf);
        result_buf.pop();
        CString::new(result_buf).unwrap()
    }

    pub fn read_chars(&mut self, size: usize) -> Vec<char>{
        let mut chars = Vec::new();
        for i in 0..size{
            chars.push(self.read_char());
        }
        chars
    }

    pub fn skip(&mut self, count_bytes_to_skip: i64){
        self.buffer.seek(SeekFrom::Current(count_bytes_to_skip));
    }


}

