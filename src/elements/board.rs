

use config::BoardSize;
use elements::Player;
use elements::{GridCell, CellCoordinate};

#[derive(Clone, Default, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {

    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}



pub struct Board{
    pub fields :  Box<[GridCell]>,
    pub size : BoardSize,
    pub start_coordinates: Point,

}

impl Board {
    pub fn new(mut size: BoardSize ) -> Board {

        let mut fields : Box<[GridCell]> = vec![GridCell::default(); size.clone().get_all_fields() ].clone().into_boxed_slice();
        //let mut rng = rand::thread_rng();
        let mut tmp_field_in_col = 0;

        let mut start_point = Point::new(0.0,0.0);
        let mut row_pos = start_point.x;
        let mut col_pos = start_point.y;
        let grid = 2.0;
        for field in (*fields).iter_mut()
        {

            tmp_field_in_col += 1;




            field.set_coordinates( CellCoordinate::new(size.get_field_size().clone(),
                                                       size.get_field_size().clone() ,
                                                       col_pos.clone(),
                                                       row_pos.clone(),
                                                       col_pos.clone() + size.get_field_size().clone() ,
                                                       row_pos.clone() + size.get_field_size().clone() ));

            col_pos =  col_pos + size.get_field_size()  + grid;
            if tmp_field_in_col == size.columns_numbers
            {
                tmp_field_in_col = 0;
                col_pos = 0.0;
                row_pos += size.get_field_size() + grid;
            }

//            if rng.gen()
//            field.cell_type = set_random_type
        }



        Board{ fields,  size: size.clone(), start_coordinates: start_point }
    }
}