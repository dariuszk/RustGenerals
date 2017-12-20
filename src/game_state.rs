use elements;
use config::BoardSize;
use elements::Player;
use elements::CellCoordinate;


pub struct State {
    pub board: elements::Board,  // Game elements to be presented
    pub score: u32,                      // Player current score
    pub players : Vec<Player>,
}


impl State {

    pub fn new(mut board_size : BoardSize ) -> State
    {
        let mut board = elements::Board::new(board_size.clone() );

        let number_player =  1;
        let cord_player = CellCoordinate::new( board_size.get_height().clone(),
                                               board_size.get_width().clone(),
                                               60.0,
                                               60.0,
                                               0.0,
                                               0.0
        );

        let mut player =  Player::new( cord_player, number_player );


        State{
            board,
            score: 0,
            players: vec![player],
        }
    }
}
