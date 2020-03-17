#[cfg(test)]
mod blp_parse {
    use std::fs::File;
    use war_editor::map_data::binary_reader::BinaryReader;
    use war_editor::blp::BLP;
    use std::io::{Read, Write, Cursor, BufReader};
    use jpeg_decoder::Decoder;
    use std::ffi::CString;

    #[test]
    fn open_local_blp_palette() {
        let mut file = File::open("resources/blp/BTNDeathBomb.blp").unwrap();
        let mut buffer: Vec<u8> = Vec::with_capacity(2000);
        file.read_to_end(&mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer.to_owned());
        let blp = reader.read::<BLP>();
        println!("{:?}", s);

    }

//    #[test]
    fn open_local_blp_jpeg() {
//        let mut file = File::open("resources/blp/FrostmourneNew.blp").unwrap();
        let mut file = File::open("resources/sample_2/war3mapMap.blp").unwrap();
        let mut buffer: Vec<u8> = Vec::with_capacity(2000);
        file.read_to_end(&mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer.to_owned());
        let blp = reader.read::<BLP>();
        for i in 0..1{
            let name = format!("resources/war3mapMap_mmap{}.jpg", i);
            let mut file = File::create(name).unwrap();
            file.write(blp.get_jpeg_header());
            file.write(&blp.get_jpeg_mipmaps()[i]);
        }

    }

//    #[test]
    fn open_local_jpeg_mipmap() {
        let mut file = File::open("resources/FrostmourneNew_mmap2.jpg").unwrap();
        let buffer = BufReader::new(file);

        let mut decoder = Decoder::new(buffer);
        decoder.read_info();
        let info = decoder.info();
        println!("{:#?}", info);
//        let res = decoder.decode().expect("error while decoding");
    }
}