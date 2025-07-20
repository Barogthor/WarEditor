use std::ffi::CString;
use std::fs::File;
use std::io::{BufRead, Cursor, Error, Read, Seek, SeekFrom};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::{BinaryConverter, BinaryConverterVersion, GameVersion, ReadError};

pub type ReadResult<T> = Result<T, ReadError>;

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

    pub fn read_char(&mut self) -> ReadResult<char>{
        Ok(char::from(self.read_u8()?))
    }
    pub fn read_u8(&mut self) -> ReadResult<u8>{
        self.buffer.read_u8().map_err(|e| to_read_error(self, e))
    }

    pub fn read_i16(&mut self) -> ReadResult<i16>{
        self.buffer.read_i16::<LittleEndian>().map_err(|e| to_read_error(self, e))
    }
    pub fn read_u16(&mut self) -> ReadResult<u16>{
        self.buffer.read_u16::<LittleEndian>().map_err(|e| to_read_error(self, e))
    }

    pub fn read_i24(&mut self) -> ReadResult<i32>{
        self.buffer.read_i24::<LittleEndian>().map_err(|e| to_read_error(self, e))
    }
    pub fn read_u24(&mut self) -> ReadResult<u32>{
        self.buffer.read_u24::<LittleEndian>().map_err(|e| to_read_error(self, e))
    }

    pub fn read_i32(&mut self) -> ReadResult<i32>{
        self.buffer.read_i32::<LittleEndian>().map_err(|e| to_read_error(self, e))
    }
    pub fn read_i32_big(&mut self) -> ReadResult<i32>{
        self.buffer.read_i32::<BigEndian>().map_err(|e| to_read_error(self, e))
    }
    pub fn read_u32(&mut self) -> ReadResult<u32> {
        self.buffer.read_u32::<LittleEndian>().map_err(|e| to_read_error(self, e))
    }
    pub fn read_u32_big(&mut self) -> ReadResult<u32> {
        self.buffer.read_u32::<BigEndian>().map_err(|e| to_read_error(self, e))
    }

    pub fn read_u64(&mut self) -> ReadResult<u64>{
        self.buffer.read_u64::<LittleEndian>().map_err(|e| to_read_error(self, e))
    }

    pub fn read_f32(&mut self) -> ReadResult<f32>{
        self.buffer.read_f32::<LittleEndian>().map_err(|e| to_read_error(self, e))
    }
    pub fn read_f64(&mut self) -> ReadResult<f64>{
        self.buffer.read_f64::<LittleEndian>().map_err(|e| to_read_error(self, e))
    }

    pub fn read_c_string(&mut self) -> ReadResult<CString>{
        let mut result_buf: Vec<u8> = Vec::new();
        self.buffer.read_until('\0' as u8, &mut result_buf).map_err(|e| to_read_error(self, e))?;
        result_buf.pop();
        Ok(CString::new(result_buf).unwrap())
    }
    pub fn read_c_string_sized(&mut self, size: usize) -> ReadResult<CString>{
        let v = self.read_bytes(size)?;
//        println!("pos: {}",self.pos());
        Ok(CString::new(v).map_err(|_| cstring_null(self, size))?)
    }

    pub fn read_string_utf8(&mut self, bytes_to_read: usize) -> ReadResult<String>{
        let v = self.read_bytes(bytes_to_read)?;
        Ok(String::from_utf8_lossy(&v).to_string())
    }


    pub fn read_chars(&mut self, size: usize) -> ReadResult<Vec<char>>{
        let mut chars = Vec::new();
        for _i in 0..size{
            chars.push(self.read_char()?);
        }
        Ok(chars)
    }

    pub fn skip(&mut self, bytes_to_skip: i64){
        self.buffer.seek(SeekFrom::Current(bytes_to_skip)).unwrap();
    }

    pub fn read<T: BinaryConverter>(&mut self) -> ReadResult<T>{
        T::read(self)
    }

    pub fn read_version<T: BinaryConverterVersion>(&mut self, game_version: &GameVersion) -> ReadResult<T>{
        T::read_version(self, game_version)
    }

    pub fn read_vec<T: BinaryConverter>(&mut self, size: usize) -> ReadResult<Vec<T>>{
        let mut vec: Vec<T> = vec![];
        for _i in 0..size{
            vec.push(T::read(self)?);
        }
        Ok(vec)
    }

    pub fn read_vec_version<T: BinaryConverterVersion>(&mut self, size: usize, game_version: &GameVersion) -> ReadResult<Vec<T>>{
        let mut vec: Vec<T> = vec![];
        for _i in 0..size{
            vec.push(T::read_version(self, game_version)?);
        }
        Ok(vec)
    }

    pub fn read_vec_i32(&mut self, size: usize) -> ReadResult<Vec<i32>>{
        let mut vec: Vec<i32> = vec![];
        for _i in 0..size{
            vec.push(self.read_i32()?);
        }
        Ok(vec)
    }

    pub fn read_vec_u32(&mut self, size: usize) -> ReadResult<Vec<u32>>{
        let mut vec: Vec<u32> = vec![];
        for _i in 0..size{
            let v = self.read_u32()?;
            vec.push(v);
        }
        Ok(vec)
    }

    pub fn read_vec_u32_be(&mut self, size: usize) -> ReadResult<Vec<u32>>{
        let mut vec: Vec<u32> = vec![];
        for _i in 0..size{
            let v = self.read_u32_big()?;
            vec.push(v);
        }
        Ok(vec)
    }

    pub fn read_vec_f32(&mut self, size: usize) -> ReadResult<Vec<f32>>{
        let mut vec: Vec<f32> = vec![];
        for _i in 0..size{
            vec.push(self.read_f32()?);
        }
        Ok(vec)
    }

    pub fn read_bytes(&mut self, size: usize) -> ReadResult<Vec<u8>>{
        let mut vec: Vec<u8> = vec![];
        for _i in 0..size{
            vec.push(self.read_u8()?);
        }
        Ok(vec)
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

fn to_read_error(reader: &BinaryReader, error: Error) -> ReadError {
    let pos = reader.pos();
    let size = reader.size;
    match error.kind() {
        std::io::ErrorKind::UnexpectedEof => ReadError::EOF(pos, size),
        _ => ReadError::Other(error)
    }
}

fn cstring_null(reader: &BinaryReader, string_size: usize) -> ReadError{
    let pos = reader.pos() - string_size as u64;
    ReadError::InvalidCString(pos, string_size)
}
