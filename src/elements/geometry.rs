use std::ops;

#[derive(Clone)]
pub enum MoveDirection
{
    Up,
    Down,
    Left,
    Right,
    None,
}




#[derive(Debug, Copy, Clone)]
pub struct Position
{
    pub row : usize,
    pub column: usize,
}

impl PartialEq for Position
{
    fn eq(&self, other: &Position) -> bool {
           return self.column == other.column && self.row == other.row;
        }
}

impl Position
{

    pub fn new(row : usize, column : usize ) -> Position
    {
        Position{ row, column }
    }

    pub fn set(&mut self, row : usize, column : usize)
    {
        self.row = row;
        self.column = column;
    }

    pub fn get_row(self) -> usize { self.row }
    pub fn get_column(self) -> usize { self.column }
}

impl Default for Position
{
    fn default() -> Position
    {
        Position { row: 0, column: 0 }
    }
}
