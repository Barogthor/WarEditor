extern crate war_editor;

macro_rules! assert_cell {
    ($cell:expr, $x: expr, $value: expr) => {{
        let cell: &RecordCell = $cell;
        assert_eq!(*cell.column(), $x);
        assert_eq!(*cell.value(), $value);
    }};
}

macro_rules! assert_line_cell{
    ($cell:expr, $x: expr, $value: expr) => {{
        let tuple = $cell.get(*$x-1).unwrap();
        assert_eq!(*$x, tuple.0);
        assert_eq!($value, tuple.1);
    }}
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
        let line1 = lines.get(&1).unwrap();
        assert_line_cell!(line1, &1, CellValue::Text( "a".to_string() ));
        assert_line_cell!(line1, &2, CellValue::Text( "b".to_string() ));
        assert_line_cell!(line1, &3, CellValue::Text( "c".to_string() ));
        assert_line_cell!(line1, &4, CellValue::Text( "d".to_string() ));
        let line2 = lines.get(&2).unwrap();
        assert_line_cell!(line2, &1, CellValue::Integer( 1 ));
        assert_line_cell!(line2, &2, CellValue::Integer( 2 ));
        assert_line_cell!(line2, &3, CellValue::Integer( 3 ));
        assert_line_cell!(line2, &4, CellValue::Integer( 4 ));

    }


}