use std::ffi::CString;
use std::io::{Cursor, Seek, SeekFrom, Write};

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

use crate::{BinaryConverter, BinaryConverterVersion, GameVersion, WriteError};

pub type WriteResult<T> = Result<T, WriteError>;

const DEFAULT_CAPACITY: usize = 1024;

pub struct BinaryWriter {
    buffer: Cursor<Vec<u8>>,
}

impl BinaryWriter {
    pub fn new() -> BinaryWriter {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> BinaryWriter {
        Self {
            buffer: Cursor::new(Vec::with_capacity(capacity)),
        }
    }

    pub fn write_char(&mut self, value: char) -> WriteResult<()> {
        self.write_u8(value as u8)
    }

    pub fn write_u8(&mut self, value: u8) -> WriteResult<()> {
        self.buffer.write_u8(value).map_err(WriteError::IoError)
    }

    pub fn write_i16(&mut self, value: i16) -> WriteResult<()> {
        self.buffer
            .write_i16::<LittleEndian>(value)
            .map_err(WriteError::IoError)
    }

    pub fn write_u16(&mut self, value: u16) -> WriteResult<()> {
        self.buffer
            .write_u16::<LittleEndian>(value)
            .map_err(WriteError::IoError)
    }

    pub fn write_i24(&mut self, value: i32) -> WriteResult<()> {
        self.buffer
            .write_i24::<LittleEndian>(value)
            .map_err(WriteError::IoError)
    }

    pub fn write_u24(&mut self, value: u32) -> WriteResult<()> {
        self.buffer
            .write_u24::<LittleEndian>(value)
            .map_err(WriteError::IoError)
    }

    pub fn write_i32(&mut self, value: i32) -> WriteResult<()> {
        self.buffer
            .write_i32::<LittleEndian>(value)
            .map_err(WriteError::IoError)
    }

    pub fn write_i32_big(&mut self, value: i32) -> WriteResult<()> {
        self.buffer
            .write_i32::<BigEndian>(value)
            .map_err(WriteError::IoError)
    }

    pub fn write_u32(&mut self, value: u32) -> WriteResult<()> {
        self.buffer
            .write_u32::<LittleEndian>(value)
            .map_err(WriteError::IoError)
    }

    pub fn write_u32_big(&mut self, value: u32) -> WriteResult<()> {
        self.buffer
            .write_u32::<BigEndian>(value)
            .map_err(WriteError::IoError)
    }

    pub fn write_u64(&mut self, value: u64) -> WriteResult<()> {
        self.buffer
            .write_u64::<LittleEndian>(value)
            .map_err(WriteError::IoError)
    }

    pub fn write_f32(&mut self, value: f32) -> WriteResult<()> {
        self.buffer
            .write_f32::<LittleEndian>(value)
            .map_err(WriteError::IoError)
    }

    pub fn write_f64(&mut self, value: f64) -> WriteResult<()> {
        self.buffer
            .write_f64::<LittleEndian>(value)
            .map_err(WriteError::IoError)
    }

    pub fn write_c_string(&mut self, value: &CString) -> WriteResult<()> {
        self.buffer
            .write_all(value.as_bytes_with_nul())
            .map_err(WriteError::IoError)
    }

    pub fn write_c_string_converted(&mut self, value: &str) -> WriteResult<()> {
        let c_string = CString::new(value).map_err(|_| {
            WriteError::Reason(format!(
                "Failed to create C string at position {}",
                self.pos()
            ))
        })?;
        self.write_c_string(&c_string)
    }

    pub fn write_string_utf8(&mut self, value: &str) -> WriteResult<()> {
        self.buffer
            .write_all(value.as_bytes())
            .map_err(WriteError::IoError)
    }

    pub fn write_chars(&mut self, chars: &[char]) -> WriteResult<()> {
        for &ch in chars {
            self.write_char(ch)?;
        }
        Ok(())
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> WriteResult<()> {
        self.buffer.write_all(bytes).map_err(WriteError::IoError)
    }

    pub fn write<T: BinaryConverter>(&mut self, value: &T) -> WriteResult<()> {
        value.write(self)
    }

    pub fn write_version<T: BinaryConverterVersion>(
        &mut self,
        value: &T,
        game_version: &GameVersion,
    ) -> WriteResult<()> {
        value.write_version(self, game_version)
    }

    pub fn write_vec<T: BinaryConverter>(&mut self, vec: &[T]) -> WriteResult<()> {
        for item in vec {
            self.write(item)?;
        }
        Ok(())
    }

    pub fn write_vec_version<T: BinaryConverterVersion>(
        &mut self,
        vec: &[T],
        game_version: &GameVersion,
    ) -> WriteResult<()> {
        for item in vec {
            self.write_version(item, game_version)?;
        }
        Ok(())
    }

    pub fn write_vec_i32(&mut self, vec: &[i32]) -> WriteResult<()> {
        for &value in vec {
            self.write_i32(value)?;
        }
        Ok(())
    }

    pub fn write_vec_u32(&mut self, vec: &[u32]) -> WriteResult<()> {
        for &value in vec {
            self.write_u32(value)?;
        }
        Ok(())
    }

    pub fn write_vec_u32_be(&mut self, vec: &[u32]) -> WriteResult<()> {
        for &value in vec {
            self.write_u32_big(value)?;
        }
        Ok(())
    }

    pub fn write_vec_f32(&mut self, vec: &[f32]) -> WriteResult<()> {
        for &value in vec {
            self.write_f32(value)?;
        }
        Ok(())
    }

    pub fn seek(&mut self, offset: i64) -> WriteResult<()> {
        self.buffer
            .seek(SeekFrom::Current(offset))
            .map(|_| ())
            .map_err(WriteError::IoError)
    }

    pub fn seek_begin(&mut self) -> WriteResult<()> {
        self.buffer
            .seek(SeekFrom::Start(0))
            .map(|_| ())
            .map_err(WriteError::IoError)
    }

    pub fn pos(&self) -> u64 {
        self.buffer.position()
    }

    pub fn size(&self) -> usize {
        self.buffer.get_ref().len()
    }

    pub fn get_buffer(&self) -> &[u8] {
        self.buffer.get_ref()
    }

    pub fn into_buffer(self) -> Vec<u8> {
        self.buffer.into_inner()
    }

    pub fn clear(&mut self) {
        self.buffer.get_mut().clear();
        self.seek_begin()
            .expect("Failed to seek to beginning after clear");
    }
}

impl Default for BinaryWriter {
    fn default() -> Self {
        Self {
            buffer: Cursor::new(Vec::with_capacity(DEFAULT_CAPACITY)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::ReadBytesExt;
    use core::{f32, f64};
    use std::ffi::CString;

    #[test]
    fn test_write_primitive_u8() {
        let mut writer = BinaryWriter::new();
        writer.write_u8(42).unwrap();
        writer.write_u8(255).unwrap();

        let buffer = writer.buffer.into_inner();
        assert_eq!(buffer, &[42, 255]);
    }

    #[test]
    fn test_write_primitive_char() {
        let mut writer = BinaryWriter::new();
        writer.write_char('A').unwrap();
        writer.write_char('Z').unwrap();

        let buffer = writer.buffer.into_inner();
        assert_eq!(buffer, &[65, 90]); // ASCII values
    }

    #[test]
    fn test_write_primitive_i16() {
        let mut writer = BinaryWriter::new();
        writer.write_i16(1000).unwrap();
        writer.write_i16(-500).unwrap();

        let buffer = writer.buffer.into_inner();
        // LittleEndian: 1000 = 0x03E8 -> [0xE8, 0x03]
        // LittleEndian: -500 = 0xFE0C -> [0x0C, 0xFE]
        assert_eq!(buffer, &[0xE8, 0x03, 0x0C, 0xFE]);
    }

    #[test]
    fn test_write_primitive_u16() {
        let mut writer = BinaryWriter::new();
        writer.write_u16(1000).unwrap();
        writer.write_u16(65535).unwrap();

        let buffer = writer.buffer.into_inner();
        // LittleEndian: 1000 = 0x03E8 -> [0xE8, 0x03]
        // LittleEndian: 65535 = 0xFFFF -> [0xFF, 0xFF]
        assert_eq!(buffer, &[0xE8, 0x03, 0xFF, 0xFF]);
    }

    #[test]
    fn test_write_primitive_i32() {
        let mut writer = BinaryWriter::new();
        writer.write_i32(1000000).unwrap();
        writer.write_i32(-1000000).unwrap();

        let buffer = writer.buffer.into_inner();
        // LittleEndian: 1000000 = 0x000F4240 -> [0x40, 0x42, 0x0F, 0x00]
        // LittleEndian: -1000000 = 0xFFF0BDC0 -> [0xC0, 0xBD, 0xF0, 0xFF]
        assert_eq!(buffer, &[0x40, 0x42, 0x0F, 0x00, 0xC0, 0xBD, 0xF0, 0xFF]);
    }

    #[test]
    fn test_write_primitive_u32() {
        let mut writer = BinaryWriter::new();
        writer.write_u32(1000000).unwrap();
        writer.write_u32(4294967295).unwrap(); // Max u32

        let buffer = writer.buffer.into_inner();
        // LittleEndian: 1000000 = 0x000F4240 -> [0x40, 0x42, 0x0F, 0x00]
        // LittleEndian: 4294967295 = 0xFFFFFFFF -> [0xFF, 0xFF, 0xFF, 0xFF]
        assert_eq!(buffer, &[0x40, 0x42, 0x0F, 0x00, 0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn test_write_primitive_endianness() {
        let mut writer = BinaryWriter::new();
        let value: u32 = 0x12345678;

        writer.write_u32(value).unwrap(); // Little endian
        writer.write_u32_big(value).unwrap(); // Big endian

        let buffer = writer.buffer.into_inner();
        // Little: [0x78, 0x56, 0x34, 0x12]
        // Big:    [0x12, 0x34, 0x56, 0x78]
        assert_eq!(buffer, &[0x78, 0x56, 0x34, 0x12, 0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_write_primitive_u64() {
        let mut writer = BinaryWriter::new();
        writer.write_u64(0x123456789ABCDEF0).unwrap();

        let buffer = writer.buffer.into_inner();
        // LittleEndian: [0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]
        assert_eq!(buffer, &[0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn test_write_primitive_f32() {
        let mut writer = BinaryWriter::new();
        writer.write_f32(f32::consts::PI).unwrap();

        let buffer = writer.buffer.into_inner();
        assert_eq!(buffer.len(), 4);

        // Verify by reading back
        let mut cursor = Cursor::new(buffer.to_vec());
        let read_value = cursor.read_f32::<LittleEndian>().unwrap();
        assert!((read_value - f32::consts::PI).abs() < 0.0001);
    }

    #[test]
    fn test_write_primitive_f64() {
        let mut writer = BinaryWriter::new();
        writer.write_f64(f64::consts::PI).unwrap();

        let buffer = writer.buffer.into_inner();
        assert_eq!(buffer.len(), 8);

        // Verify by reading back
        let mut cursor = Cursor::new(buffer.to_vec());
        let read_value = cursor.read_f64::<LittleEndian>().unwrap();
        assert!((read_value - f64::consts::PI).abs() < 0.0000000000001);
    }

    #[test]
    fn test_write_primitive_i24_u24() {
        let mut writer = BinaryWriter::new();
        writer.write_i24(0x123456).unwrap();
        writer.write_u24(0xABCDEF).unwrap();

        let buffer = writer.buffer.into_inner();
        // i24: 0x123456 -> [0x56, 0x34, 0x12] (LittleEndian)
        // u24: 0xABCDEF -> [0xEF, 0xCD, 0xAB] (LittleEndian)
        assert_eq!(buffer, &[0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB]);
    }

    #[test]
    fn test_write_c_string() {
        let mut writer = BinaryWriter::new();
        let c_str = CString::new("Hello").unwrap();
        writer.write_c_string(&c_str).unwrap();

        let buffer = writer.buffer.into_inner();
        // "Hello" + null terminator
        assert_eq!(buffer, b"Hello\0");
    }

    #[test]
    fn test_write_c_string_converted() {
        let mut writer = BinaryWriter::new();
        writer.write_c_string_converted("World").unwrap();

        let buffer = writer.buffer.into_inner();
        // "World" + null terminator
        assert_eq!(buffer, b"World\0");
    }

    #[test]
    fn test_write_c_string_with_null_error() {
        let mut writer = BinaryWriter::new();
        let result = writer.write_c_string_converted("Hello\0World");

        assert!(result.is_err());
        match result.unwrap_err() {
            WriteError::Reason(_) => {} // Expected
            _ => panic!("Expected WriteError::Reason"),
        }
    }

    #[test]
    fn test_write_string_utf8() {
        let mut writer = BinaryWriter::new();
        writer.write_string_utf8("Hello UTF-8 🦀").unwrap();

        let buffer = writer.buffer.into_inner();
        let expected = "Hello UTF-8 🦀".as_bytes();
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_write_chars() {
        let mut writer = BinaryWriter::new();
        let chars = ['A', 'B', 'C'];
        writer.write_chars(&chars).unwrap();

        let buffer = writer.buffer.into_inner();
        assert_eq!(buffer, &[65, 66, 67]); // ASCII values
    }

    #[test]
    fn test_write_empty_string() {
        let mut writer = BinaryWriter::new();
        writer.write_string_utf8("").unwrap();

        let buffer = writer.buffer.into_inner();
        assert_eq!(buffer, &[]);
    }

    #[test]
    fn test_write_bytes() {
        let mut writer = BinaryWriter::new();
        let data = &[0x01, 0x02, 0x03, 0xFF, 0xFE];
        writer.write_bytes(data).unwrap();

        let buffer = writer.buffer.into_inner();
        assert_eq!(buffer, data);
    }

    #[test]
    fn test_large_data_exceeds_default_capacity() {
        let mut writer = BinaryWriter::new();

        // DEFAULT_CAPACITY is 1024, so write more than that
        let large_data_size = DEFAULT_CAPACITY * 2;
        let large_data: Vec<u8> = (0..large_data_size).map(|i| (i % 256) as u8).collect();

        // This should not cause an error and should auto-resize the buffer
        writer.write_bytes(&large_data).unwrap();

        let buffer = writer.buffer.into_inner();
        assert_eq!(buffer.len(), large_data_size);
        assert_eq!(buffer, &large_data[..]);
    }

    #[test]
    fn test_write_multiple_large_operations() {
        let mut writer = BinaryWriter::new();

        // Write multiple chunks that together exceed DEFAULT_CAPACITY
        for i in 0..5 {
            let chunk_size = DEFAULT_CAPACITY / 2;
            let chunk: Vec<u8> = (0..chunk_size)
                .map(|j| ((i * 100 + j) % 256) as u8)
                .collect();
            writer.write_bytes(&chunk).unwrap();
        }

        let buffer = writer.buffer.into_inner();
        let expected_size = DEFAULT_CAPACITY / 2 * 5; // 2.5 * DEFAULT_CAPACITY
        assert_eq!(buffer.len(), expected_size);
        assert!(buffer.len() > DEFAULT_CAPACITY);
    }

    #[test]
    fn test_buffer_management() {
        let mut writer = BinaryWriter::new();

        // Test position tracking
        assert_eq!(writer.pos(), 0);

        writer.write_u32(42).unwrap();
        assert_eq!(writer.pos(), 4);
        assert_eq!(writer.size(), 4);

        writer.write_u16(100).unwrap();
        assert_eq!(writer.pos(), 6);
        assert_eq!(writer.size(), 6);

        // Test clear
        writer.clear();
        assert_eq!(writer.pos(), 0);
        assert_eq!(writer.size(), 0);
        assert_eq!(writer.buffer.into_inner().len(), 0);
    }

    #[test]
    fn test_seek_operations() {
        let mut writer = BinaryWriter::new();

        // Write some initial data
        writer.write_u32(0x12345678).unwrap();
        writer.write_u32(0xABCDEF00).unwrap();
        assert_eq!(writer.pos(), 8);

        // Seek to beginning
        writer.seek_begin().unwrap();
        assert_eq!(writer.pos(), 0);

        // Seek forward
        writer.seek(4).unwrap();
        assert_eq!(writer.pos(), 4);

        // Overwrite second u32
        writer.write_u32(0x11111111).unwrap();

        let buffer = writer.buffer.into_inner();
        // First u32: 0x12345678 -> [0x78, 0x56, 0x34, 0x12]
        // Second u32: 0x11111111 -> [0x11, 0x11, 0x11, 0x11]
        assert_eq!(buffer, &[0x78, 0x56, 0x34, 0x12, 0x11, 0x11, 0x11, 0x11]);
    }

    #[test]
    fn test_into_buffer() {
        let mut writer = BinaryWriter::new();
        writer.write_u32(0x12345678).unwrap();
        writer.write_string_utf8("test").unwrap();

        let buffer = writer.buffer.into_inner();
        let expected = vec![0x78, 0x56, 0x34, 0x12, b't', b'e', b's', b't'];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_with_capacity() {
        let custom_capacity = 4096;
        let writer = BinaryWriter::with_capacity(custom_capacity);

        // Buffer should be empty but have the requested capacity
        assert_eq!(writer.size(), 0);
        assert_eq!(writer.pos(), 0);
        // We can't directly test capacity, but we can test that it works
        assert!(writer.buffer.into_inner().is_empty());
    }

    #[test]
    fn test_write_vec_primitives() {
        let mut writer = BinaryWriter::new();

        let i32_vec = vec![1, 2, 3, -1, -2];
        let u32_vec = vec![100, 200, 300];
        let f32_vec = vec![1.1, 2.2, 3.3];

        writer.write_vec_i32(&i32_vec).unwrap();
        writer.write_vec_u32(&u32_vec).unwrap();
        writer.write_vec_f32(&f32_vec).unwrap();

        let buffer = writer.buffer.into_inner();

        // Should have: 5*4 + 3*4 + 3*4 = 44 bytes
        assert_eq!(buffer.len(), 44);

        // Verify first few bytes (first i32: 1 -> [0x01, 0x00, 0x00, 0x00])
        assert_eq!(&buffer[0..4], &[0x01, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_write_vec_u32_big_endian() {
        let mut writer = BinaryWriter::new();
        let values = vec![0x12345678, 0xABCDEF00];

        writer.write_vec_u32_be(&values).unwrap();

        let buffer = writer.buffer.into_inner();
        // Big endian: 0x12345678 -> [0x12, 0x34, 0x56, 0x78]
        // Big endian: 0xABCDEF00 -> [0xAB, 0xCD, 0xEF, 0x00]
        assert_eq!(buffer, &[0x12, 0x34, 0x56, 0x78, 0xAB, 0xCD, 0xEF, 0x00]);
    }
}
