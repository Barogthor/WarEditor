//! BLP1 texture decoding and encoding (JPEG-content and paletted variants) for
//! Warcraft III minimap and imported map assets.

use rgb::RGBA8;
use thiserror::Error;

use crate::binary_reader::BinaryReader;
use crate::binary_writer::{BinaryWriter, WriteResult};
use crate::{ReadError, WriteError};
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
    /// Raw compressed JPEG fragment bytes per mipmap, kept verbatim so the file
    /// can be re-serialized byte-for-byte without lossy re-encoding.
    jpeg_mipmaps_raw: Vec<Vec<u8>>,
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
            // The compressed fragment is kept verbatim for byte-exact
            // re-serialization; decoding to pixels is deferred until an editor
            // actually needs them (todos/11 point 7, phase 2).
            self.jpeg_mipmaps_raw.push(reader.read_bytes(size)?);
        }
        Ok(())
    }

    fn parse_palette(&mut self, reader: &mut BinaryReader) -> Result<(), ReadError> {
        // BLP1 palette entries are stored BGRA, not RGBA as the Magos spec
        // literally states. Verified empirically (test `dump_palette_channel_order`)
        // and consistent with HiveWE; see the note in specs/blp.txt at COLOR.
        // Alpha is inverse-stored (real alpha = 255 - stored) for PictureType 5.
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

    /// Image width in pixels (top mipmap).
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Image height in pixels (top mipmap).
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Number of stored mipmap levels (raw JPEG fragments or paletted index lists).
    pub fn mipmap_count(&self) -> usize {
        match self.compression {
            Compression::JPEG => self.jpeg_mipmaps_raw.len(),
            Compression::PALETTE => self.palette_rgb_indexes.len(),
        }
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
            jpeg_mipmaps_raw: Vec::with_capacity(MAX_MIPMAP),
            palette_colors: vec![],
            palette_rgb_indexes: Vec::with_capacity(MAX_MIPMAP),
            palette_alpha_indexes: Vec::with_capacity(MAX_MIPMAP),
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

//     data.chunks(4).for_each(|cmyk| {

//         println!("[{:.0}, {:.0}, {:.0}] or [{:.0}, {:.0}, {:.0}, {:.0}]", red, green, blue, c*100., m*100. , y*100., k*100.);
//     });

#[cfg(test)]
mod blp_parse {
    use std::fs::File;
    use std::io::Read;

    use crate::binary_reader::BinaryReader;
    use crate::blp::BLP;
    use crate::get_resources_path;

    fn get_path(path: &str) -> String {
        let prefix = get_resources_path();
        format!("{prefix}/{path}")
    }

    /// Read a resource file fully, failing loudly if it is missing so a missing
    /// fixture can never make a test pass silently.
    fn read_resource(rel: &str) -> Vec<u8> {
        let path = get_path(rel);
        let mut file =
            File::open(&path).unwrap_or_else(|e| panic!("missing resource {}: {:?}", path, e));
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .unwrap_or_else(|e| panic!("failed to read {}: {:?}", path, e));
        buffer
    }

    fn parse_resource(rel: &str) -> BLP {
        let mut reader = BinaryReader::new(read_resource(rel));
        BLP::from(&mut reader).unwrap_or_else(|e| panic!("failed to parse {}: {:?}", rel, e))
    }

    #[test]
    fn open_local_blp_palette() {
        let blp = parse_resource("blp/BTNDeathBomb.blp");
        assert_eq!((blp.width(), blp.height()), (64, 64));
        assert_eq!(blp.mipmap_count(), 7);
    }

    /// Render the top paletted mipmap under both channel interpretations to
    /// settle whether BLP1 palettes are BGRA (as the code reads) or RGBA (as the
    /// Magos spec literally says). Ignored; run on demand and eyeball the PNGs:
    ///
    /// ```text
    /// cargo test -p wce_formats --lib dump_palette_channel_order -- --ignored
    /// ```
    #[test]
    #[ignore = "dumps channel-order comparison PNGs to resources/blp/out/"]
    fn dump_palette_channel_order() {
        let blp = parse_resource("blp/BTNDeathBomb.blp");
        let (w, h) = (blp.width(), blp.height());
        let indexes = &blp.palette_rgb_indexes[0];
        let out_dir = format!("{}blp/out", get_resources_path());
        std::fs::create_dir_all(&out_dir).unwrap_or_else(|e| panic!("{:?}", e));

        // `palette_colors` already holds the code's BGRA reading (r=byte2, b=byte0).
        let bgra = image::ImageBuffer::from_fn(w, h, |x, y| {
            let c = blp.palette_colors[indexes[(y * w + x) as usize] as usize];
            image::Rgb([c.r, c.g, c.b])
        });
        // The RGBA-literal reading is the same bytes with red/blue swapped back.
        let rgba = image::ImageBuffer::from_fn(w, h, |x, y| {
            let c = blp.palette_colors[indexes[(y * w + x) as usize] as usize];
            image::Rgb([c.b, c.g, c.r])
        });
        bgra.save(format!("{out_dir}/BTNDeathBomb_bgra.png"))
            .unwrap_or_else(|e| panic!("{:?}", e));
        rgba.save(format!("{out_dir}/BTNDeathBomb_rgba.png"))
            .unwrap_or_else(|e| panic!("{:?}", e));
        println!("wrote {out_dir}/BTNDeathBomb_{{bgra,rgba}}.png");
    }

    #[test]
    fn rejects_non_blp1_magic() {
        use crate::blp::BLPError;
        // BLP2 (WoW) has a different layout; it must fail fast with a clear
        // error instead of being misparsed as a BLP1 header.
        let mut reader = BinaryReader::new(b"BLP2".to_vec());
        match BLP::from(&mut reader) {
            Err(BLPError::InvalidMagic(m)) => assert_eq!(m, "BLP2"),
            Err(e) => panic!("expected InvalidMagic, got a different error: {:?}", e),
            Ok(_) => panic!("expected InvalidMagic, but parsing succeeded"),
        }
    }

    /// Read a resource, parse it, write it back, and return
    /// `(original_bytes, written_bytes)`.
    fn roundtrip(rel: &str) -> (Vec<u8>, Vec<u8>) {
        use crate::binary_writer::BinaryWriter;
        let original = read_resource(rel);
        let blp = parse_resource(rel);
        let mut writer = BinaryWriter::new();
        blp.write(&mut writer)
            .unwrap_or_else(|e| panic!("failed to write {}: {:?}", rel, e));
        (original, writer.into_buffer())
    }

    #[test]
    fn roundtrip_blp_palette() {
        let (original, written) = roundtrip("blp/BTNDeathBomb.blp");
        assert_eq!(
            written, original,
            "paletted BLP roundtrip must be byte-exact"
        );
    }

    /// Read a BLP, write it back, and persist the result to `resources/blp/out/`
    /// (mirroring the source's basename) so the output can be opened in a BLP
    /// viewer and compared visually with the source. Asserts byte-exactness.
    fn persist_roundtrip(src_rel: &str) {
        let (original, written) = roundtrip(src_rel);

        let basename = src_rel.rsplit('/').next().unwrap();
        let out_dir = format!("{}blp/out", get_resources_path());
        std::fs::create_dir_all(&out_dir).unwrap_or_else(|e| panic!("{:?}", e));
        let out_path = format!("{out_dir}/{basename}");
        std::fs::write(&out_path, &written).unwrap_or_else(|e| panic!("{:?}", e));

        println!("source:  {}", get_path(src_rel));
        println!("written: {out_path}");
        assert_eq!(
            written, original,
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
        let blp = parse_resource("Scenario/Sandbox_Roc/war3mapMap.blp");
        assert_eq!((blp.width(), blp.height()), (256, 256));
        assert_eq!(blp.mipmap_count(), 1);
        // Blizzard's shared JPEG header is the classic 624 bytes here.
        assert_eq!(blp.get_jpeg_header().len(), 624);
    }

    // Byte-exact roundtrip of a JPEG-compressed BLP: the writer preserves the
    // original compressed fragments (no lossy re-encode), including the zero gap
    // between the shared JPEG header and the mipmap data (todos/11 point 7, phase 1).
    #[test]
    fn roundtrip_blp_jpeg_map() {
        let (original, written) = roundtrip("Scenario/Sandbox_Roc/war3mapMap.blp");
        assert_eq!(written, original, "JPEG BLP roundtrip must be byte-exact");
    }

    #[test]
    fn open_local_blp_jpeg_texture() {
        // FrostmourneNew is a JPEG texture with the alpha flag set (Flags = 8).
        let blp = parse_resource("blp/FrostmourneNew.blp");
        assert_eq!((blp.width(), blp.height()), (256, 512));
        assert!(blp.has_alpha(), "Flags = 8 must report an alpha channel");
        assert!(blp.mipmap_count() >= 1);
        assert!(!blp.get_jpeg_header().is_empty());
    }
}
