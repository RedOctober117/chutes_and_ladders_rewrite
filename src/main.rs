use board_draw::board_draw::*;
use crate::player_data::player_data::*;

pub mod board_draw;
pub mod player_data;
pub mod coords;

fn main() {
    let player1 = Player::build_player('b', Coords::build_coords(10, 1));
    let mut players = vec![player1];
    draw_board(10, players);
    println!("x: {}", square_to_x_coord(12, 1));
    println!("x: {}", square_to_x_coord(99, 2));
    println!("x: {}", square_to_x_coord(11, 3));
    println!("x: {}", square_to_x_coord(26, 4));
    println!("y: {}", square_to_y_coord(12, 1));
    println!("y: {}", square_to_y_coord(99, 2));
    println!("y: {}", square_to_y_coord(11, 3));
    println!("y: {}", square_to_y_coord(26, 4));
}

fn square_to_x_coord(square: i32, player_number: i32) -> i32 {
    println!("Square {}", square);
    let mut effective_space = square;
    if square > 10 {
        if square / 10 as i32 % 10 == 0 {
            effective_space = 10;
        } else {
            if square / 10 as i32 % 2 == 0 {
                effective_space = effective_space - 10 * (effective_space / 10 as i32);
            } else {
                effective_space  = (((effective_space / 10 as i32) * 10) + 10) - (effective_space - 1);
            }
        }
    }
    let offset = match player_number {
        1 | 3 => 4,
        2 | 4 => 2,
        i32::MIN..=0_i32 | 5_i32..=i32::MAX => todo!(),
    };
    effective_space * 6 - offset
}


fn square_to_y_coord(square: i32, player_number: i32) -> i32 {
    println!("Square {}", square);
    let collection_of_y_1_2 = [28, 25, 22, 18, 15, 13, 10, 7, 4, 1];
    let collection_of_y_3_4 = [29, 26, 23, 20, 17, 14, 11, 8, 5, 2];
    let rounded_square = square / 10 as i32;
    match player_number {
        1 | 2 => collection_of_y_1_2[rounded_square as usize],
        3 | 4 => collection_of_y_3_4[rounded_square as usize],
        i32::MIN..=0_i32 | 5_i32..=i32::MAX => todo!(),
    }
}