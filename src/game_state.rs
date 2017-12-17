use elements;
use config::BoardSize;

pub struct State {
    pub board: elements::Board,  // Game elements to be presented
    pub score: u32                      // Player current score
}


impl State {

    pub fn new( board_size : BoardSize ) -> State
    {
        let mut board = elements::Board::new(board_size);
        State{
            board,
            score: 0,
        }
    }
}
