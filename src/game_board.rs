pub const MAX_BOARD_SIZE: usize = 80;

pub struct GameBoard{
    pub generation: i32,
    pub grid:[[bool; MAX_BOARD_SIZE]; MAX_BOARD_SIZE],
}

impl GameBoard{
    /// creates a randomly generated board
    pub fn new() -> GameBoard{
        let mut board = [[false; MAX_BOARD_SIZE];MAX_BOARD_SIZE];
        for i in &mut board{
            for j in i{
                *j = rand::random::<bool>();
            }
        };
        GameBoard{generation: 0, grid: board}
    }
    
    /// sets board state to the next generation
    pub fn next_generation(self: &mut GameBoard){
        let previous_gen = self.grid;
        for i in 0..MAX_BOARD_SIZE{
            for j in 0..MAX_BOARD_SIZE{
                let neighbours = GameBoard::count_neighbours(&previous_gen, i, j);
                self.grid[i][j] = match (self.grid[i][j], neighbours) {
                    (_, 3) => true,
                    (true, 2) => true,
                    _ => false,
                };
            }
        }
    }
    
    /// returns the number of neighbours that are true in all 8 surrounding cells
    fn count_neighbours(grid: &[[bool; MAX_BOARD_SIZE]; MAX_BOARD_SIZE], i: usize, j: usize) -> i32{
        let mut neighbours = 0;
        if grid.get(i + 1).unwrap_or(&[false; MAX_BOARD_SIZE])[j]{
            neighbours += 1;
        }
        if *grid[i].get(j + 1).unwrap_or(&false){
            neighbours += 1;
        }
        if *grid.get(i + 1).unwrap_or(&[false; MAX_BOARD_SIZE]).get(j + 1).unwrap_or(&false){
            neighbours += 1;
        }
        if grid[i.saturating_sub(1)][j]{
            neighbours += 1;
        }
        if grid[i][j.saturating_sub(1)]{
            neighbours += 1;
        }
        if grid[i.saturating_sub(1)][j.saturating_sub(1)]{
            neighbours += 1;
        }
        if grid.get(i + 1).unwrap_or(&[false; MAX_BOARD_SIZE])[j.saturating_sub(1)]{
            neighbours += 1;
        }
        if *grid[i.saturating_sub(1)].get(j + 1).unwrap_or(&false){
            neighbours += 1;
        }
        neighbours
    }
    
    #[allow(dead_code)]
    pub fn draw_board(&self){
        for i in self.grid.iter(){
            for j in i{
                match *j{
                    true => print!("1"),
                    false => print!("0"),
                }
            }
            println!("\n");
        }
    }
    
    /// places a "blinker" pattern in it's first state on the board to the bottom right of the x, y coordinates given
    #[allow(dead_code)]
    pub fn add_blinker(&mut self, x: usize, y: usize){
        *self.grid.get_mut(x + 1)    .unwrap_or(&mut [false; MAX_BOARD_SIZE]).get_mut(y).unwrap_or(&mut false) = true;
        *self.grid.get_mut(x + 1).unwrap_or(&mut [false; MAX_BOARD_SIZE]).get_mut(y + 1).unwrap_or(&mut false) = true;
        *self.grid.get_mut(x + 1).unwrap_or(&mut [false; MAX_BOARD_SIZE]).get_mut(y + 2).unwrap_or(&mut false) = true;
    }
}