
extern crate rand;
use config::BoardSize;
use rand::Rng;


use elements::{GridCell, CellCoordinate};
pub struct Board{
    pub fields :  Box<[GridCell]>,
    pub size : BoardSize,

}

impl Board {
    pub fn new(mut size: BoardSize ) -> Board {

        let mut fields : Box<[GridCell]> = vec![GridCell::default(); size.clone().get_all_fields() ].clone().into_boxed_slice();
        let mut rng = rand::thread_rng();
        let mut tmp_field_in_col = 0;
        let mut row_pos = 0.0;
        let mut col_pos = 0.0;
        let grid = 3.0;
        for field in (*fields).iter_mut()
        {

            tmp_field_in_col += 1;

            col_pos =  col_pos + size.get_field_size() as f32 + grid;


            field.set_coordinates( CellCoordinate::new(size.get_field_size().clone(),
                                                       size.get_field_size().clone() ,
                                                       col_pos.clone(),
                                                        row_pos.clone()));

            if tmp_field_in_col == size.columns_numbers
            {
                tmp_field_in_col = 0;
                col_pos = 0.0;
                row_pos += size.get_field_size() + grid;
            }

            if rng.gen()
            field.cell_type =
        }


        Board{ fields: fields, size: size.clone() }
    }
}