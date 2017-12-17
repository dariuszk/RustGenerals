
#[derive(Copy, Clone)]
pub struct CellCoordinate{
    pub with: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32

}

impl Default for CellCoordinate {
    #[inline]
    fn default() -> CellCoordinate {
        CellCoordinate {
            with: 0.0,
            height: 0.0,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl CellCoordinate
{
    pub fn new( with : f32, height : f32, x : f32, y : f32 ) -> CellCoordinate
    {
        CellCoordinate{ with, height, x, y }
    }

    pub fn set(&mut self, with : f32, height : f32, x : f32, y : f32)
    {
        self.with = with;
        self.height = height;
        self.x = x;
        self.y = y;
    }
}



#[derive(Copy, Clone, PartialEq)]
pub enum GridCellType {
    Void,
    Acquired,
    Mountains,
    Town,
    Capital
}

#[derive(Copy, Clone)]
pub struct GridCell {
    pub cell_type: GridCellType,
    pub owner: i32,
    pub troops: i32,
    pub coordinates: CellCoordinate,
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
        }
    }
}


impl GridCell{

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
