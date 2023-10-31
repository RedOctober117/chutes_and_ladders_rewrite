pub mod board_draw {

    use crate::Player;

    pub fn draw_board(dimension: usize, player_data: Vec<Player>) -> () {
        
        let horizontal_scalar = 6;
        let vertical_scalar = 3;

        let scaled_horizontal_dimension = (dimension * horizontal_scalar) + 1;
        let scaled_vertical_dimension = (dimension * vertical_scalar) + 1;

        let mut x = 0;
        let mut y = 0;

        while y < scaled_vertical_dimension {
            if y % 3 == 0 || y == 0 {
                loop {
                    if x == scaled_horizontal_dimension {
                        break;
                    }
                    print!("*");
                    x += 1;
                }
            } else {
                loop {
                    if x == scaled_horizontal_dimension {
                        break;
                    }
                    if x % horizontal_scalar == 0 || x == 0 {
                        print!("*");
                        x += 1;
                    } else {
                        for player in &player_data {
                            if player.get_coords() == (x, y){
                                print!("{}", player.get_token());
                                x += 1;
                            } else {
                                print!(" ");
                                x += 1;
                            }
                        }
                    }
                }
            }
            println!("");
            x = 0;
            y += 1;
        }
    }
}

// ******* 46
// * a b * 48
// * c d * 47
// ******* 46
// * a b * 48
// * c d * 47
// ******* 46
// * a b * 48
// * c d * 47
// ******* 46
// * a b * 48
// * c d * 47
// ******* 46
// * a b * 48
// * c d * 47
// ******* 46
// * a b * 48
// * c d * 47
// ******* 46
// * a b * 48
// * c d * 47
// ******* 46
// * a b * 48
// * c d * 47
// ******* 46
