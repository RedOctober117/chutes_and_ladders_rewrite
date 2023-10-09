use board_draw::board_draw::*;
use crate::player_data::player_data::*;

pub mod board_draw;
pub mod player_data;

fn main() {
    let player1 = Player::build_player('b', Coords::build_coords(2, 1));
    let mut players = vec![player1];
    draw_board(8, players)
}
