use std::time::Instant;

#[cfg(test)]
mod sample {
    use slkparser::SLKScanner;
    use slkparser::slk_type::{RecordType, Record};
    use slkparser::record::cell::{CellValue, Cell};
    use slkparser::document::Document;

    #[test]
    fn test_open(){
        let mut slk_reader = SLKScanner::open("resources/sample_1.slk");
    }

    #[test]
    fn parse_record_one_by_one() {
        let to_s = |s: &str| String::from(s);
        let s = String::default();
        let mut slk_reader = SLKScanner::open("resources/sample_1.slk");
        let fetch = slk_reader.parse_record();
        assert_eq!(fetch, Ok( Record::Header ));

        let fetch = slk_reader.parse_record();
        assert_eq!(fetch, Ok( Record::Info(3, 4) ));

        let fetch = slk_reader.parse_record();
        let cell = Cell::new(1u32, Some(1u32), Some(CellValue::Text(to_s("a"))) );
        assert_eq!(fetch, Ok( Record::CellContent(cell) ));

        for _ in 0..11 {
            slk_reader.parse_record();
        }
        let fetch = slk_reader.parse_record();
        assert_eq!(fetch, Ok( Record::EOF ));
        let fetch = slk_reader.parse_record();
        assert_eq!(fetch, Err(to_s("EOF")));
    }

    #[test]
    fn parse_iterator() {
        let mut slk_reader = SLKScanner::open("resources/sample_1.slk");
        let mut count = 0;
        for record in slk_reader {
            println!("{:?}", record);
            count+=1;
        }
        assert_eq!(count,14);
    }

    #[test]
    fn document_test() {
        let mut slk_reader = SLKScanner::open("resources/sample_1.slk");
        let mut document = Document::default();
        document.load(slk_reader);
        document.debug();
    }
}