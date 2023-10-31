pub mod player_data {
    pub struct Coords {
        x: usize,
        y: usize,
    }

    pub struct Player {
        identifier: String,
        number: i32,
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
                identifier: String::from(""),
                number: 0,
                coords: Coords::new(),
            }
        }

        pub fn build_player(identifier: String, coords: Coords) -> Player {
            Player {
                identifier: identifier,
                coords: coords,
            }
        }
        
        pub fn define_token(&mut self, identifier: String) -> () {
            self.identifier = identifier;
        }

        pub fn get_token(&self) -> String {
            self.identifier
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

        pub fn get_square(&self) -> i32 {

        }
    }
}
