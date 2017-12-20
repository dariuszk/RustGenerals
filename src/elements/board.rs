use config::BoardSize;
use elements::Player;
use elements::{GridCell, CellCoordinate};
use super::{MoveDirection,Position};


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
    pub fields : Box<[GridCell]>,
    pub cells: Box<[ Box<[ GridCell]>]>,
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



        let mut data = vec![ vec![GridCell::default(); size.get_rows_number().clone()].into_boxed_slice(); size.get_columns_number() ].into_boxed_slice();

        let mut board_start_point = Point::new(0.0, 0.0);
        let mut row_position = board_start_point.x;
        let mut column_position =  board_start_point.y;
        let grid = 2.0;

        for (col_id, col) in data.iter_mut().enumerate()
        {
            for (row_id, cell) in col.iter_mut().enumerate()
            {
                let cell_data = CellCoordinate::new(size.get_field_size().clone(),
                                                                size.get_field_size().clone() ,
                                                                    column_position.clone() ,
                                                                    row_position.clone(),
                                                                 column_position.clone() + size.get_field_size().clone() ,
                                                       row_position.clone()  + size.get_field_size().clone() );
                cell.info.position.set( col_id, row_id );
                cell.set_coordinates(cell_data);

                row_position += size.get_field_size() + grid;
            }
            row_position = 0.0;
            column_position += size.get_field_size() + grid;
        }

        Board{ fields, cells: data, size: size.clone(), start_coordinates: start_point }
    }

    pub fn get_cell_coordinates(&mut self, position : Position) -> CellCoordinate
    {
        for (col_id, col) in self.cells.iter_mut().enumerate()
        {
            for (row_id, cell) in col.iter_mut().enumerate()
                {
                    if row_id == position.row && col_id == position.column
                        {
                            return cell.coordinates;
                        }
                }
        }

        return CellCoordinate::default();
    }


    pub fn movement_on_board(& self, current_position : Position, steps : f32, direction : MoveDirection  ) -> Position
    {

        let mut new_position = Position::new(current_position.get_row(), current_position.get_column());

        match direction {
            MoveDirection::Up => {

                if (current_position.get_row() as f32 - steps) >= 0.0
                    {
                        new_position.set( ( current_position.get_row() as f32 - steps ) as usize , current_position.get_column()  as usize  );
                    }
            }
            MoveDirection::Down => {
                if (current_position.get_row() as f32 + steps) <= (self.size.get_rows_number()-1) as  f32
                    {
                        new_position.set( (current_position.get_row()  as f32 + steps ) as usize, current_position.get_column()  as usize );
                    }
            },
            MoveDirection::Left => {

                if (current_position.get_column() as f32 - steps ) >= 0.0
                    {
                        new_position.set(current_position.get_row() as usize, ( current_position.get_column() as f32 - steps) as usize )  ;
                    }
            },
            MoveDirection::Right => {

               if (current_position.get_column() as f32 + steps) <= (self.size.get_columns_number()-1) as f32
                   {
                      new_position.set( current_position.get_row()  as usize, (current_position.get_column() as f32 + steps)  as usize );
                   }
            },
            _ => {}

        }


        return new_position;

    }
}