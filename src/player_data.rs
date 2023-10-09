pub mod player_data {
    pub struct Coords {
        x: usize,
        y: usize,
    }

    pub struct Player {
        token: char,
        coords: Coords
    }

    impl Coords {
        pub fn new() -> Coords {
            Coords {
                x: 0,
                y: 0,
            }
        }

        pub fn build_coords(x: usize, y: usize) -> Coords {
            Coords {
                x: x,
                y: y,
            }
        }

        pub fn define_x_y(&mut self, x: usize, y: usize) -> () {
            self.x = x;
            self.y = y;
        }

        pub fn define_x(&mut self, x: usize) {
            self.x = x;
        }

        pub fn define_y(&mut self, y: usize) {
            self.y = y;
        }

        pub fn get_coords(&self) -> (usize, usize) {
            return (self.x, self.y)
        }
    }

    impl Player {
        pub fn new() -> Player {
            Player {
                token: ' ',
                coords: Coords::new(),
            }
        }

        pub fn build_player(token: char, coords: Coords) -> Player {
            Player {
                token: token,
                coords: coords,
            }
        }
        
        pub fn define_token(&mut self, token: char) -> () {
            self.token = token;
        }

        pub fn get_token(&self) -> char {
            self.token
        }

        pub fn define_x_y(&mut self, x: usize, y: usize) -> () {
            self.coords.define_x_y(x, y);
            // self.x = x;
            // self.y = y;
        }

        pub fn define_x(&mut self, x: usize) -> () {
            self.coords.define_x(x);
        }

        pub fn define_y(&mut self, y: usize) -> () {
            self.coords.define_y(y);
        }

        pub fn get_coords(&self) -> (usize, usize) {
            self.coords.get_coords()
        }
    }
}
