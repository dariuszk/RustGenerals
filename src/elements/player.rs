

use elements::CellCoordinate;
use elements::CellInfo;


pub enum PlayerMoveSteps
{
    Up,
    Down,
    Left,
    Right,
}

pub struct Player {
    pub number: usize,
    pub army: usize,
    pub position : CellCoordinate,
    pub current_cell : CellInfo,
    pub owned_cells : Box<[CellInfo]>,
}

impl Player
{
     pub fn new(position : CellCoordinate, number : usize) -> Player {
        Player{
            number: 0,
            army: 0,
            position,
            current_cell: CellInfo::default(),
            owned_cells: vec![ CellInfo::default() ].clone().into_boxed_slice()
        }
     }

    pub fn go(&mut self, direction : PlayerMoveSteps )
    {
        match direction {
            PlayerMoveSteps::Up => { self.position.y -= 1.0; },
            PlayerMoveSteps::Down => { self.position.y += 1.0; },
            PlayerMoveSteps::Left => { self.position.x -= 1.0; },
            PlayerMoveSteps::Right => { self.position.x += 1.0; },
        }
    }
}