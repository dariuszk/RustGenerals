extern crate opengl_graphics;


use piston_window::{self, Context, Transformed};
use piston_window::ellipse::Ellipse;
use piston_window::rectangle;
use piston_window::text;
use piston_window::math::Matrix2d;
use piston_window::types::Color;

use self::opengl_graphics::GlGraphics;
use resources::Resources;
use game_state;

use elements::{GridCell, CellCoordinate, Board, Player};

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
pub fn render(c: Context, g: &mut GlGraphics, state: &mut game_state::State, resources: &mut Resources )
{
    piston_window::clear(LIGHT_GRAY, g);
    render_board(c, g, resources,  &mut state.board, &state.players);
}

pub fn render_board(c: Context, g: &mut GlGraphics, resources: &mut Resources, board:  &mut Board, players : &Vec<Player>)
{

    for (col_id, col) in (*board).cells.iter_mut().enumerate()
    {
         for (row_id, cell) in col.iter_mut().enumerate()
         {

           render_cell(c, g, resources, cell, DARK_GRAY);

         }
    }

    for player in (players).iter()
    {
        let mut cell_coordinates = (*board).get_cell_coordinates( player.current_cell.position );
        mark_player(c, g, resources, player, &cell_coordinates);
    }

}

pub fn render_cell(c: Context, g: &mut GlGraphics, resources: &mut Resources, cell: &GridCell, color : Color)
{

    rectangle(color,
              [cell.coordinates.x as f64,
                    cell.coordinates.y as f64,
                    cell.coordinates.with as f64,
                    cell.coordinates.height as f64],
              c.transform,
              g);

     text(RED
          , 12
          , &cell.troops.to_string()
          , &mut resources.font
          , c.trans(cell.coordinates.x as f64 + 7.5   , cell.coordinates.y as f64 + 20.0).transform
          , g);

     text(YELLOW
          , 11
          , &cell.troops.to_string()
          , &mut resources.font
          , c.trans(cell.coordinates.x as f64 + 7.5   , cell.coordinates.y as f64 + 20.0).transform
          , g);

}

pub fn mark_player(c: Context, g: &mut GlGraphics, resources : &mut Resources, player: &Player, coordinates: &CellCoordinate)
{
    rectangle(CYAN,
              [(coordinates.x ) as f64,
                  ( coordinates.y  ) as f64,
                  ( coordinates.with  ) as f64,
                  ( coordinates.height ) as f64],
              c.transform,
              g);
    text(BLACK
          , 12
          , &player.army.to_string()
          , &mut resources.font
          , c.trans(coordinates.x  as f64 + 7.5, coordinates.y as f64 + 20.0).transform
          , g);
}


