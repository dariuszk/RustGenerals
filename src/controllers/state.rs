use rand::{self, ThreadRng};
use super::Actions;
use game_state::State;
use elements::{PlayerMoveSteps,MoveDirection,Position};

pub struct StateInTimeController {
    rng: ThreadRng,
    current_time: f64,
    last_move: f64
}


impl StateInTimeController {

    pub fn new()-> StateInTimeController
    {
        StateInTimeController{
            rng: rand::thread_rng(),
            current_time: 0.0,
            last_move: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64, actions: &mut Actions, state: &mut State)
    {
        self.current_time += dt;
        self.last_move += dt;

        if self.last_move < 0.05
            {
               return
            }

        self.last_move = 0.0;



        println!("LEN : {}",actions.actions_to_consume.len() );
        if actions.actions_to_consume.len() == 0
        {
          return;
        }
        let mut moveType : MoveDirection = actions.actions_to_consume.dequeue().unwrap();
        let new_position = state.board.movement_on_board(state.players[0].current_cell.position, 1.0, moveType);
        state.players[0].current_cell.position.set(new_position.row, new_position.column);

//        if actions.go_up{
//            moveType = MoveDirection::Up;
//            //state.players[0].go(PlayerMoveSteps::Up);
//            let new_position = state.board.movement_on_board(state.players[0].current_cell.position, 1.0 , moveType);
//            state.players[0].current_cell.position.set(new_position.row, new_position.column);
//        }
//
//        if actions.go_down{
//            moveType = MoveDirection::Down;
//            //state.players[0].go(PlayerMoveSteps::Down);
//            let new_position = state.board.movement_on_board(state.players[0].current_cell.position, 1.0 , moveType);
//            state.players[0].current_cell.position.set(new_position.row, new_position.column);
//        }
//
//        if actions.go_left{
//            moveType = MoveDirection::Left;
//           // state.players[0].go(PlayerMoveSteps::Left);
//            let new_position = state.board.movement_on_board(state.players[0].current_cell.position, 1.0 , moveType);
//            state.players[0].current_cell.position.set(new_position.row, new_position.column);
//        }
//
//        if actions.go_right{
//            moveType = MoveDirection::Right;
//            //state.players[0].go(PlayerMoveSteps::Right);
//            let new_position = state.board.movement_on_board(state.players[0].current_cell.position, 1.0 , moveType);
//            state.players[0].current_cell.position.set(new_position.row, new_position.column);
//        }


    }
}


