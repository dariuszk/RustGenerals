

use elements::CellCoordinate;


pub struct Player {
    pub number: usize,
    pub army: usize,
    pub position : CellCoordinate,

}

impl Player
{
     pub fn new(position : CellCoordinate, number : usize) -> Player {
        Player{
            number: 0,
            army: 0,
            position,
        }
     }
}