pub mod utility {
    pub enum Movement {
        Left,
        Right,
        Up,
        Down,
    }

    pub struct Player {
        pub body: Vec<(Movement, u32, u32)>, //mov,row,col
        pub score: u32,
        pub name: String,
        pub alive: bool,
    }
}
