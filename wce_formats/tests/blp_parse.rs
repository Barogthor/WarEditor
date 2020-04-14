#![allow(dead_code)]
#[cfg(test)]
mod blp_parse {
    use std::fs::File;
    use std::io::{BufReader, Read, Write};
    use std::io;

    use jpeg_decoder::Decoder;

    use war_editor::blp::BLP;
    use wce_map::binary_reader::BinaryReader;

    #[test]
    fn open_local_blp_palette() {
        let mut file = File::open("resources/blp/BTNDeathBomb.blp").unwrap();
        let mut buffer: Vec<u8> = Vec::with_capacity(2000);
        file.read_to_end(&mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer.to_owned());
        let _blp = reader.read::<BLP>();
//        println!("{:?}", s);

    }

    #[test]
    fn open_local_blp_jpeg_map() -> Result<(), io::Error>{
        let mut file = File::open("resources/sample_2/war3mapMap.blp")?;
        let mut buffer: Vec<u8> = Vec::with_capacity(2000);
        file.read_to_end(&mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer.to_owned());
        let blp: BLP = reader.read();
        // for i in 0..1{
        //     let name = format!("resources/war3mapMap_mmap{}.jpg", i);
        //     let mut file = File::create(name).unwrap();
        //     file.write(blp.get_jpeg_header()).unwrap();
        //     let mipmap = &blp.get_jpeg_mipmaps()[i];
        //     file.write().unwrap();
        // }
        Ok(())
    }

    #[test]
    fn open_local_blp_jpeg_texture() -> Result<(), io::Error>{
        let mut file = File::open("resources/blp/FrostmourneNew.blp")?;
        let mut buffer: Vec<u8> = Vec::with_capacity(2000);
        file.read_to_end(&mut buffer).unwrap();
        let mut reader = BinaryReader::new(buffer.to_owned());
        let blp: BLP = reader.read();
        let mmap1 = &blp.get_jpeg_mipmaps()[3];
        println!("{:?}", mmap1);
        // println!("{:#?}", mmap1[0..mmap1.len()/100]);
        // for i in 0..3{
        //     let name = format!("resources/FrostmourneNew_mmap{}.jpg", i);
        //     let mut file = File::create(name).unwrap();
        //     file.write(blp.get_jpeg_header()).unwrap();
        //     file.write(&blp.get_jpeg_mipmaps()[i]).unwrap();
        // }
        Ok(())
    }



   // #[test]
    fn open_local_jpeg_mipmap() -> Result<(), ()> {
        let file = File::open("resources/FrostmourneNew_mmap2.jpg").unwrap();
        let mut buffer = BufReader::new(file);

        let mut decoder = Decoder::new(buffer);
        decoder.read_info().unwrap();
        let info = decoder.info();
        println!("{:#?}", info);
        let res = decoder.decode().expect("error while decoding");
        Ok(())
    }
}