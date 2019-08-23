
pub mod colors {
    use crate::map_data::binary_reader::{BinaryConverter, BinaryReader};
    use crate::map_data::binary_writer::BinaryWriter;

    #[derive(Copy, Debug)]
    struct RGBAData {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }

    impl Clone for RGBAData {
        fn clone(&self) -> Self {
            RGBAData {
                r: self.r,
                g: self.g,
                b: self.b,
                a: self.a
            }
        }
    }

    #[derive(Copy, Clone)]
    pub union RGBA {
        value: u32,
        rgba: RGBAData,
    }

    impl RGBA {
        pub fn by_value(value: u32) -> Self{
            RGBA{value}
        }

        pub fn red(&self) -> u8 {
            unsafe {
                return self.rgba.r
            }
        }
        pub fn green(&self) -> u8 {
            unsafe {
                return self.rgba.g
            }
        }
        pub fn blue(&self) -> u8 {
            unsafe {
                return self.rgba.b
            }
        }
        pub fn alpha(&self) -> u8 {

            unsafe {
                return self.rgba.a
            }
        }
        pub fn value(&self) -> u32 {
            unsafe {
                return self.value
            }
        }

        pub fn debug(&self){
            let value = unsafe {
                match self {
                    RGBA { value } => value
                }
            };
            let rgba = unsafe {
                match self {
                    RGBA { rgba } => rgba
                }
            };
            println!("{:X?} <=> {:X?}", value, rgba );
        }
    }
    impl BinaryConverter for RGBA {
        fn read(reader: &mut BinaryReader) -> Self {
            let value = reader.read_u32();
            RGBA {value}
        }

        fn write(&self, writer: &mut BinaryWriter) {
            unimplemented!()
        }
    }

    #[derive(Copy, Debug)]
    struct RGBData {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl Clone for RGBData {
        fn clone(&self) -> Self {
            RGBData {
                r: self.r,
                g: self.g,
                b: self.b,
            }
        }
    }

    #[derive(Copy, Clone)]
    pub union RGB {
        value: u32,
        rgb: RGBData,
    }

    impl RGB {
        pub fn by_value(value: u32) -> Self{
            RGB{value: value&0x00FFFFFF}
        }

        pub fn red(&self) -> u8 {
            unsafe {
                return self.rgb.r
            }
        }
        pub fn green(&self) -> u8 {
            unsafe {
                return self.rgb.g
            }
        }
        pub fn blue(&self) -> u8 {
            unsafe {
                return self.rgb.b
            }
        }
        pub fn value(&self) -> u32 {
            unsafe {
                return self.value
            }
        }
        pub fn debug(&self){
            let value = unsafe {
                match self {
                    RGB { value } => value
                }
            };
            let rgb = unsafe {
                match self {
                    RGB { rgb } => rgb
                }
            };
            println!("{:X?} <=> {:X?}", value, rgb );
        }
    }

    impl BinaryConverter for RGB {
        fn read(reader: &mut BinaryReader) -> Self {
            let value = reader.read_u24();
            RGB {value}
        }

        fn write(&self, writer: &mut BinaryWriter) {
            unimplemented!()
        }
    }

}