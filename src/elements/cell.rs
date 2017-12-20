use super::Position;


#[derive(Copy, Clone, Debug)]
pub struct CellCoordinate{
    pub with: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub x_max: f64,
    pub y_max: f64,
}

impl Default for CellCoordinate {
    #[inline]
    fn default() -> CellCoordinate {
        CellCoordinate {
            with: 0.0,
            height: 0.0,
            x: 0.0,
            y: 0.0,
            x_max: 0.0,
            y_max: 0.0,
        }
    }
}

impl CellCoordinate
{
    pub fn new( mut with : f64, mut height : f64, mut x : f64, mut y : f64, mut x_max : f64, mut y_max: f64 ) -> CellCoordinate
    {
        CellCoordinate{ with, height, x, y, x_max, y_max }
    }

    pub fn set(&mut self, with : f64, height : f64, x : f64, y : f64, x_max : f64, y_max: f64)
    {
        self.with = with;
        self.height = height;
        self.x = x;
        self.y = y;
        self.x_max = x_max;
        self.y_max = y_max;
    }

    pub fn is_in(self, pos_x : f64, pos_y : f64) -> bool
    {
        return  self.x <= pos_x && pos_x <= self.x_max && self.y <= pos_y && pos_y <= self.y_max ;
    }
}



#[derive(Debug,Copy, Clone, PartialEq)]
pub enum GridCellType {
    Void,
    Acquired,
    Mountains,
    Town,
    Capital
}


#[derive(Copy, Clone, Debug)]
pub struct CellInfo {
    pub row_id: usize,
    pub column_id: usize,
    pub position: Position,
    pub row_min: usize,
    pub row_max: usize,
    pub col_min: usize,
    pub col_max: usize
}

impl Default for CellInfo
{
    fn default() -> CellInfo
    {
        CellInfo{
            row_id: 0,
            column_id: 0,
            position: Position::default(),
            row_min: 0,
            row_max: 0,
            col_min: 0,
            col_max: 0,
        }
    }
}

#[derive(Debug, Copy, Clone )]
pub struct GridCell {
    pub cell_type: GridCellType,
    pub owner: i32,
    pub troops: i32,
    pub coordinates: CellCoordinate,
    pub info: CellInfo
}

/// Default GridCell's shape_index to -1 instead of 0
impl Default for GridCell {
    #[inline]
    fn default() -> GridCell {
        GridCell {
            cell_type: GridCellType::Void,
            owner: -1,
            troops: 0,
            coordinates: CellCoordinate::default(),
            info: CellInfo::default(),
        }
    }
}


impl GridCell{

    pub fn new(cell_id : CellInfo) -> GridCell
    {
        let mut cell = GridCell::default();
        cell.info = cell_id;

        return  cell;
    }

    pub fn set_coordinates(&mut self, new_coordinates : CellCoordinate)
    {
        self.coordinates = new_coordinates;
    }

//    pub fn set_random_type(&mut self, &mut rng : ThreadRng)
//    {
//        let between = Range::new(0, 5); // TODO maybe macro for enum length
//
//        match between.ind_sample(&mut rng)
//            {
//                0 => { self.cell_type = GridCellType::Void },
//                1 => { self.cell_type = GridCellType::Mountains },
//                2 => { self.cell_type = GridCellType::Town },
//                _ =>  { self.cell_type = GridCellType::Void },
//            }
//    }
}
