extern crate war_editor;

macro_rules! assert_cell{
    ($cell:expr, $x: expr, $value: expr) => {{
        let cell: &RecordCell = $cell;
        assert_eq!(*cell.column(), $x);
        assert_eq!(*cell.value(), $value);
    }};
}

#[cfg(test)]
mod slk_tests {
    use war_editor::slk::slk::SLKReader;
    use war_editor::slk::record::cell::{RecordCell,CellValue};
    use war_editor;
    use war_editor::slk::merge_slk;

    #[test]
    fn parse_sample_1_test(){
        let mut slk_reader = SLKReader::open_file("resources/slk/test.slk".to_string());
        let document = slk_reader.parse().unwrap();
        assert_eq!(2, *document.get_rows());
        assert_eq!(2, *document.get_columns());
        let cells: &Vec<RecordCell> = document.get_cells();
        assert_cell!(cells.get(0).unwrap(), 1, Some(CellValue::Text("a".to_string())));
        assert_cell!(cells.get(1).unwrap(), 2, Some(CellValue::Text("b".to_string())));
        assert_cell!(cells.get(2).unwrap(), 1, Some(CellValue::Integer(1)));
        assert_cell!(cells.get(3).unwrap(), 2, Some(CellValue::Integer(2)));
    }

    #[test]
    fn parse_merge_test(){
        let mut slk_reader = SLKReader::open_file("resources/slk/test.slk".to_string());
        let document = slk_reader.parse().unwrap();
//    document.debug();
        let mut lines = document.get_cells_value_sorted_by_line();
        let mut slk_reader = SLKReader::open_file("resources/slk/test_2.slk".to_string());
        let document = slk_reader.parse().unwrap();
        merge_slk(&mut lines, &document);
    }


}