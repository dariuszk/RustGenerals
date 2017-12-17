extern crate opengl_graphics;


use piston_window::{self, Context, Transformed};
use piston_window::ellipse::Ellipse;
use piston_window::rectangle;
use piston_window::text;
use piston_window::math::Matrix2d;
use piston_window::types::Color;
use self::opengl_graphics::GlGraphics;

use game_state;
use elements::Board;
use elements::GridCell;
use elements::Player;

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const LIGHT_GRAY: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
pub const DARK_GRAY: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const CYAN: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
pub const PURPLE: [f32; 4] = [0.5, 0.0, 1.0, 1.0];

//
pub fn render(c: Context, g: &mut GlGraphics, state: &game_state::State){


    piston_window::clear(LIGHT_GRAY, g);

    render_board(c, g, &state.board, &state.players);
}

pub fn render_board(c: Context, g: &mut GlGraphics, board:  &Board, players : &Vec<Player>)
{
    for field in (*board.fields).iter()
    {
        render_cell(c, g, field);

        for player in (players).iter()
        {
            if field.coordinates.is_in(player.position.x, player.position.y)
                {
                    mark_player(c, g, player, field);
                }


        }
    }
}

pub fn render_cell(c: Context, g: &mut GlGraphics, cell: &GridCell)
{
    rectangle(DARK_GRAY,
              [cell.coordinates.x as f64, cell.coordinates.y as f64, cell.coordinates.with as f64, cell.coordinates.height as f64],
              c.transform,
              g);

}

pub fn mark_player(c: Context, g: &mut GlGraphics, player: &Player, cell: &GridCell)
{
    rectangle(CYAN,
              [(cell.coordinates.x + 2.0) as f64,
                  ( cell.coordinates.y + 2.0 ) as f64,
                  ( cell.coordinates.with - 3.0 ) as f64,
                  ( cell.coordinates.height  - 3.0 )as f64],
              c.transform,
              g);

}


