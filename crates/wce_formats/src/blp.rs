#![allow(dead_code)]

use std::io::Cursor;

use image::{GenericImageView, ImageReader};
use rgb::{RGB8, RGBA8};
use thiserror::Error;

use crate::binary_reader::BinaryReader;
use crate::binary_writer::{BinaryWriter, WriteResult};
use crate::{ReadError, WriteError};

type MipmapPixels = Vec<Vec<RGB8>>;
type MipmapIndexes = Vec<Vec<u8>>;
pub const PALETTE_SIZE: usize = 256;
pub const JPG_BLP: bool = false;
pub const PALETTED_BLP: bool = true;
pub const MAX_MIPMAP: usize = 16;

#[derive(Debug, Error)]
pub enum BLPError {
    #[error("Error while reading BLP buffer. {0}")]
    Read(ReadError),
    #[error("Error while writing BLP buffer. {0}")]
    Write(WriteError),
    #[error("Decoding JPEG failure. {0}")]
    Decoding(#[from] image::error::ImageError),
    #[error("Unknown BLP type '{0}'.")]
    UnknownType(u32),
    #[error("Unsupported BLP magic '{0}', expected 'BLP1'.")]
    InvalidMagic(String),
}
impl From<ReadError> for BLPError {
    fn from(value: ReadError) -> Self {
        Self::Read(value)
    }
}

/// Expected file magic for the BLP1 format (Warcraft III). BLP0 (beta) and
/// BLP2 (WoW) use different layouts and are not supported.
const BLP1_MAGIC: &str = "BLP1";
/// `Flags` bit that signals an alpha channel (`specs/blp.txt:49`).
const FLAG_ALPHA: u32 = 0x0000_0008;

#[derive(Debug)]
pub enum BlpData {
    //    JpgBlp(JpgBlpData),
    //    PalettedBlp(PalettedBlpData),
}

#[derive(PartialOrd, PartialEq, Clone, Debug, Copy)]
pub enum Compression {
    JPEG = 0,
    PALETTE = 1,
}

impl Compression {
    pub fn from(n: u32) -> Result<Self, BLPError> {
        match n {
            //TODO faire conversion slim (regarder jpeg_decoder marker)
            0 => Ok(Compression::JPEG),
            1 => Ok(Compression::PALETTE),
            _ => Err(BLPError::UnknownType(n)),
        }
    }
}

pub struct BLP {
    magic_num: String,
    compression: Compression,
    /// Raw `Flags` field, kept verbatim for fidelity. Bits combine; the alpha
    /// bit is `FLAG_ALPHA` (see `has_alpha`).
    flags: u32,
    width: u32,
    height: u32,
    /// Raw `PictureType` (3/4 = index + alpha list, 5 = index only), kept
    /// verbatim; layout decisions use `has_alpha_list`.
    picture_type: u32,
    /// Raw `PictureSubType`; semantics unknown per spec, kept opaque.
    picture_sub_type: u32,
    mipmap_offsets: Vec<u32>,
    mipmap_sizes: Vec<u32>,

    jpeg_header_size: u32,
    jpeg_header: Vec<u8>,
    jpeg_mipmaps_dim: Vec<(u32, u32)>,
    jpeg_mipmaps: MipmapPixels,
    /// Raw compressed JPEG fragment bytes per mipmap, kept verbatim so the file
    /// can be re-serialized byte-for-byte without lossy re-encoding.
    jpeg_mipmaps_raw: Vec<Vec<u8>>,
    // jpeg_mipmaps: Vec<DynamicImage>,
    palette_colors: Vec<RGBA8>,
    palette_rgb_indexes: MipmapIndexes,
    palette_alpha_indexes: MipmapIndexes,
}

impl BLP {
    fn parse_jpeg_mipmaps(&mut self, reader: &mut BinaryReader) -> Result<(), BLPError> {
        self.jpeg_header_size = reader.read_u32()?;
        self.jpeg_header = reader.read_bytes(self.jpeg_header_size as usize)?;
        for i in 0..MAX_MIPMAP {
            let size = self.mipmap_sizes[i] as usize;
            let offset = self.mipmap_offsets[i] as i64;
            if size == 0 {
                break;
            }
            reader.seek_begin();
            reader.skip(offset);
            let raw = reader.read_bytes(size)?;
            // Keep the compressed fragment verbatim for byte-exact re-serialization,
            // then rebuild a full JPEG (shared header + fragment) for decoding.
            self.jpeg_mipmaps_raw.push(raw.clone());
            let mut jpeg_buffer = self.jpeg_header.clone();
            jpeg_buffer.reserve(size + 10);
            jpeg_buffer.extend_from_slice(&raw);

            let reader = Cursor::new(jpeg_buffer);
            let mut reader = ImageReader::new(reader);
            reader.set_format(image::ImageFormat::Jpeg);

            let image = reader.decode().map_err(BLPError::Decoding)?;
            self.jpeg_mipmaps_dim.push(image.dimensions());

            let pixels: Vec<RGB8> = image
                .to_rgb8()
                .pixels()
                .map(|rgb| RGB8::new(rgb.0[0], rgb.0[1], rgb.0[2]))
                .collect();
            self.jpeg_mipmaps.push(pixels);
        }
        Ok(())
    }

    fn parse_palette(&mut self, reader: &mut BinaryReader) -> Result<(), ReadError> {
        self.palette_colors = reader
            .read_bytes(PALETTE_SIZE * 4)?
            .chunks(4)
            .map(|bgra| RGBA8 {
                r: bgra[2],
                g: bgra[1],
                b: bgra[0],
                a: 255 - bgra[3],
            })
            .collect();
        for i in 0..MAX_MIPMAP {
            let size = self.mipmap_sizes[i] as usize;
            let offset = self.mipmap_offsets[i] as i64;
            if size == 0 {
                continue;
            }
            reader.seek_begin();
            reader.skip(offset);

            self.palette_rgb_indexes.push(reader.read_bytes(size)?);
            if self.has_alpha_list() {
                self.palette_alpha_indexes.push(reader.read_bytes(size)?);
            }
        }
        Ok(())
    }

    pub fn get_jpeg_header(&self) -> &Vec<u8> {
        &self.jpeg_header
    }

    /// Whether the `Flags` field marks an alpha channel. Bit test, not equality:
    /// the spec says flags combine (`specs/blp.txt:29,49`).
    pub fn has_alpha(&self) -> bool {
        self.flags & FLAG_ALPHA != 0
    }

    /// Whether each mipmap carries a separate per-pixel alpha index list, i.e.
    /// `PictureType` 3 or 4 (`specs/blp.txt:52-54`). Type 5 is index-only.
    fn has_alpha_list(&self) -> bool {
        matches!(self.picture_type, 3 | 4)
    }
    // pub fn get_jpeg_mipmaps(&self) -> &MipmapPixels {
    //     &self.jpeg_mipmaps
    // }

    pub fn from(reader: &mut BinaryReader) -> Result<Self, BLPError> {
        let magic_num = reader.read_string_utf8_safe(4)?;
        if magic_num != BLP1_MAGIC {
            return Err(BLPError::InvalidMagic(magic_num));
        }
        let compression = reader.read_u32()?;
        let compression = Compression::from(compression)?;
        let flags = reader.read_u32()?;
        let width = reader.read_u32()?;
        let height = reader.read_u32()?;
        let picture_type = reader.read_u32()?;
        let picture_sub_type = reader.read_u32()?;
        let mipmap_offsets = reader.read_vec_u32(MAX_MIPMAP)?;
        let mipmap_sizes = reader.read_vec_u32(MAX_MIPMAP)?;
        let mut blp = BLP {
            magic_num,
            compression,
            flags,
            width,
            height,
            picture_type,
            picture_sub_type,
            mipmap_offsets,
            mipmap_sizes,
            jpeg_header_size: 0,
            jpeg_header: Vec::with_capacity(MAX_MIPMAP),
            jpeg_mipmaps: Vec::with_capacity(MAX_MIPMAP),
            jpeg_mipmaps_raw: Vec::with_capacity(MAX_MIPMAP),
            palette_colors: vec![],
            palette_rgb_indexes: Vec::with_capacity(MAX_MIPMAP),
            palette_alpha_indexes: Vec::with_capacity(MAX_MIPMAP),
            jpeg_mipmaps_dim: Vec::with_capacity(MAX_MIPMAP),
        };
        match blp.compression {
            Compression::JPEG => blp.parse_jpeg_mipmaps(reader)?,
            Compression::PALETTE => blp.parse_palette(reader)?,
        };
        // No EOF assertion: the spec allows unused padding between the JPEG
        // header and the mipmap data, and after the last mipmap
        // (`specs/blp.txt:86-88`). A mipmap that overruns the file is already
        // caught by `read_bytes` (read_exact) during parsing.
        Ok(blp)
    }

    //TODO very likely need to recalculate mipmap size/offset and palette for some use case (minimap, menu minimap, ...)
    pub fn write(&self, writer: &mut BinaryWriter) -> Result<(), BLPError> {
        self.write_blp(writer).map_err(BLPError::Write)
    }
    fn write_blp(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        writer.write_string_utf8(&self.magic_num)?;
        writer.write_u32(self.compression as u32)?;
        writer.write_u32(self.flags)?;
        writer.write_u32(self.width)?;
        writer.write_u32(self.height)?;
        writer.write_u32(self.picture_type)?;
        writer.write_u32(self.picture_sub_type)?;
        // Spec: MipMapOffset[16] then MipMapSize[16] as two contiguous blocks,
        // never interleaved (specs/blp.txt:56-57).
        for i in 0..MAX_MIPMAP {
            writer.write_u32(*self.mipmap_offsets.get(i).unwrap_or(&0))?;
        }
        for i in 0..MAX_MIPMAP {
            writer.write_u32(*self.mipmap_sizes.get(i).unwrap_or(&0))?;
        }
        match self.compression {
            Compression::JPEG => self.write_jpeg_mipmaps(writer)?,
            Compression::PALETTE => self.write_palette(writer)?,
        };
        Ok(())
    }

    /// Re-serialize the JPEG payload by preserving the original compressed
    /// bytes: shared header followed by each mipmap fragment placed at its
    /// original offset (spec `specs/blp.txt:56-96`). The gap the spec allows
    /// between header and data is reproduced as zero padding. This is lossless
    /// and byte-exact; it does not re-encode pixels (see todos/11 point 7,
    /// phase 2).
    fn write_jpeg_mipmaps(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        writer.write_u32(self.jpeg_header.len() as u32)?;
        writer.write_bytes(&self.jpeg_header)?;
        for (i, fragment) in self.jpeg_mipmaps_raw.iter().enumerate() {
            let target = *self.mipmap_offsets.get(i).unwrap_or(&0) as u64;
            while writer.pos() < target {
                writer.write_u8(0)?;
            }
            writer.write_bytes(fragment)?;
        }
        Ok(())
    }

    fn write_palette(&self, writer: &mut BinaryWriter) -> WriteResult<()> {
        for rgba in &self.palette_colors {
            let bgra = [rgba.b, rgba.g, rgba.r, 255 - rgba.a];
            writer.write_bytes(&bgra)?;
        }
        for i in 0..self.palette_rgb_indexes.len() {
            writer.write_bytes(&self.palette_rgb_indexes[i])?;
            if self.has_alpha_list() {
                writer.write_bytes(&self.palette_alpha_indexes[i])?;
            }
        }
        Ok(())
    }
}

fn cmyk_to_rgb(cmyk: &mut [u8]) -> RGB8 {
    let c = cmyk[0] as f32 / 255.0;
    let y = cmyk[1] as f32 / 255.0;
    let m = cmyk[2] as f32 / 255.0;
    let k = cmyk[3] as f32 / 255.0;
    let red = 255.0 * (1. - c) * (1. - k);
    //        let red = 255.0 - c;
    let green = 255.0 * (1. - y) * (1. - k);
    //        let green = 255.0 - y;
    let blue = 255.0 * (1. - m) * (1. - k);
    //        let blue = 255.0 - m;
    RGB8 {
        r: red as u8,
        b: blue as u8,
        g: green as u8,
    }
}

//     data.chunks(4).for_each(|cmyk| {

//         println!("[{:.0}, {:.0}, {:.0}] or [{:.0}, {:.0}, {:.0}, {:.0}]", red, green, blue, c*100., m*100. , y*100., k*100.);
//     });

#[cfg(test)]
mod blp_parse {
    use std::fs::File;
    use std::io::{BufReader, Read};

    use image::ImageReader;
    use log::warn;

    use crate::binary_reader::BinaryReader;
    use crate::blp::BLP;
    use crate::get_resources_path;

    fn get_path(path: &str) -> String {
        let prefix = get_resources_path();
        format!("{prefix}/{path}")
    }

    #[test]
    fn open_local_blp_palette() {
        let file_res = File::open(get_path("blp/BTNDeathBomb.blp"));
        match file_res {
            Ok(mut file) => {
                let mut buffer: Vec<u8> = Vec::with_capacity(2000);
                file.read_to_end(&mut buffer)
                    .unwrap_or_else(|e| panic!("{:?}", e));
                let mut reader = BinaryReader::new(buffer);
                let _blp = BLP::from(&mut reader);
                //        println!("{:?}", s);
            }
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn rejects_non_blp1_magic() {
        use crate::blp::BLPError;
        // BLP2 (WoW) has a different layout; it must fail fast with a clear
        // error instead of being misparsed as a BLP1 header.
        let mut reader = BinaryReader::new(b"BLP2".to_vec());
        match BLP::from(&mut reader) {
            Err(BLPError::InvalidMagic(m)) => assert_eq!(m, "BLP2"),
            other => panic!("expected InvalidMagic, got {other:?}"),
        }
    }

    #[test]
    fn roundtrip_blp_palette() {
        use crate::binary_writer::BinaryWriter;

        let mut file =
            File::open(get_path("blp/BTNDeathBomb.blp")).unwrap_or_else(|e| panic!("{:?}", e));
        let mut original: Vec<u8> = Vec::new();
        file.read_to_end(&mut original)
            .unwrap_or_else(|e| panic!("{:?}", e));

        let mut reader = BinaryReader::new(original.clone());
        let blp = BLP::from(&mut reader).unwrap();

        let mut writer = BinaryWriter::new();
        blp.write(&mut writer).unwrap();

        assert_eq!(
            writer.get_buffer(),
            original.as_slice(),
            "paletted BLP roundtrip must be byte-exact"
        );
    }

    /// Read a BLP, write it back, and persist the result to `resources/blp/out/`
    /// (mirroring the source's basename) so the output can be opened in a BLP
    /// viewer and compared visually with the source. Asserts byte-exactness.
    fn persist_roundtrip(src_rel: &str) {
        use crate::binary_writer::BinaryWriter;

        let src = get_path(src_rel);
        let mut file = File::open(&src).unwrap_or_else(|e| panic!("{:?}", e));
        let mut original: Vec<u8> = Vec::new();
        file.read_to_end(&mut original)
            .unwrap_or_else(|e| panic!("{:?}", e));

        let mut reader = BinaryReader::new(original.clone());
        let blp = BLP::from(&mut reader).unwrap();

        let mut writer = BinaryWriter::new();
        blp.write(&mut writer).unwrap();

        let basename = src_rel.rsplit('/').next().unwrap();
        let out_dir = format!("{}blp/out", get_resources_path());
        std::fs::create_dir_all(&out_dir).unwrap_or_else(|e| panic!("{:?}", e));
        let out_path = format!("{out_dir}/{basename}");
        std::fs::write(&out_path, writer.get_buffer()).unwrap_or_else(|e| panic!("{:?}", e));

        println!("source:  {src}");
        println!("written: {out_path}");
        assert_eq!(
            writer.get_buffer(),
            original.as_slice(),
            "written BLP differs from source (see {out_path})"
        );
    }

    /// Persist read→write output for both a paletted and a JPEG BLP to
    /// `resources/blp/out/` for manual visual inspection. Ignored by default
    /// (filesystem side effect); run on demand:
    ///
    /// ```text
    /// cargo test -p wce_formats --lib blp_write_persistent -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore = "writes to resources/blp/out/ for manual visual inspection"]
    fn blp_write_persistent() {
        persist_roundtrip("blp/BTNDeathBomb.blp"); // paletted
        persist_roundtrip("Scenario/Sandbox_Roc/war3mapMap.blp"); // JPEG
    }

    #[test]
    fn open_local_blp_jpeg_map() {
        let mut file = File::open(get_path("Scenario/Sandbox_Roc/war3mapMap.blp"))
            .unwrap_or_else(|e| panic!("{:?}", e));
        let mut buffer: Vec<u8> = Vec::with_capacity(2000);
        file.read_to_end(&mut buffer)
            .unwrap_or_else(|e| panic!("{:?}", e));
        let mut reader = BinaryReader::new(buffer);
        let _blp = BLP::from(&mut reader);
        // for i in 0..1{
        //     let name = format!("resources/war3mapMap_mmap{}.jpg", i);
        //     let mut file = File::create(name).unwrap();
        //     file.write(blp.get_jpeg_header()).unwrap();
        //     let mipmap = &blp.get_jpeg_mipmaps()[i];
        //     file.write().unwrap();
        // }
    }

    // Byte-exact roundtrip of a JPEG-compressed BLP: the writer preserves the
    // original compressed fragments (no lossy re-encode), including the zero gap
    // between the shared JPEG header and the mipmap data (todos/11 point 7, phase 1).
    #[test]
    fn roundtrip_blp_jpeg_map() {
        use crate::binary_writer::BinaryWriter;

        let mut file = File::open(get_path("Scenario/Sandbox_Roc/war3mapMap.blp"))
            .unwrap_or_else(|e| panic!("{:?}", e));
        let mut original: Vec<u8> = Vec::new();
        file.read_to_end(&mut original)
            .unwrap_or_else(|e| panic!("{:?}", e));

        let mut reader = BinaryReader::new(original.clone());
        let blp = BLP::from(&mut reader).unwrap();

        let mut writer = BinaryWriter::new();
        blp.write(&mut writer).unwrap();

        assert_eq!(
            writer.get_buffer(),
            original.as_slice(),
            "JPEG BLP roundtrip must be byte-exact"
        );
    }

    #[test]
    fn open_local_blp_jpeg_texture() {
        let file_res = File::open(get_path("blp/FrostmourneNew.blp"));
        match file_res {
            Ok(mut file) => {
                let mut buffer: Vec<u8> = Vec::with_capacity(2000);
                file.read_to_end(&mut buffer)
                    .unwrap_or_else(|e| panic!("{:?}", e));
                let mut reader = BinaryReader::new(buffer);
                let blp = BLP::from(&mut reader).unwrap();
                // let mmap1 = &blp.get_jpeg_mipmaps()[3];
                // println!("{mmap1:?}");
                // println!("{:#?}", mmap1[0..mmap1.len()/100]);
                // for i in 0..3{
                //     let name = format!("resources/FrostmourneNew_mmap{}.jpg", i);
                //     let mut file = File::create(name).unwrap();
                //     file.write(blp.get_jpeg_header()).unwrap();
                //     file.write(&blp.get_jpeg_mipmaps()[i]).unwrap();
                // }
            }
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn open_local_jpeg_mipmap() {
        let file_res = File::open(get_path("FrostmourneNew_mmap2.jpg"));
        match file_res {
            Ok(file) => {
                let buffer = BufReader::new(file);
                let mut reader = ImageReader::new(buffer);
                reader.set_format(image::ImageFormat::Jpeg);
                let image = reader.decode().unwrap_or_else(|e| panic!("{}", e));
                // image.read_info().unwrap_or_else(|e| panic!("{:?}", e));
                // let info = decoder.info();
                // println!("{info:#?}");
                // decoder.decode().unwrap_or_else(|e| panic!("{:?}", e));
            }
            Err(e) => println!("{e:?}"),
        }
    }
}
