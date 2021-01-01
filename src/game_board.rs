pub const MAX_BOARD_SIZE: usize = 80;

pub struct GameBoard{
    pub generation: i32,
    pub grid:[[bool; MAX_BOARD_SIZE]; MAX_BOARD_SIZE],
}

impl GameBoard{
    pub fn new() -> GameBoard{
        let mut board = [[false; MAX_BOARD_SIZE];MAX_BOARD_SIZE];
        for i in &mut board{
            for j in i{
                *j = rand::random::<bool>();
            }
        };
        GameBoard{generation: 0, grid: board}
    }
    
    pub fn next_generation(self: &mut GameBoard){
        let previous_gen = self.grid.clone();
        for i in 0..MAX_BOARD_SIZE{
            for j in 0..MAX_BOARD_SIZE{
                let neighbours = GameBoard::count_neighbours(&previous_gen, i, j);
                self.grid[i][j] = match (self.grid[i][j], neighbours) {
                    (false, 3) => true,
                    (true, 2) => true,
                    (true, 3) => true,
                    _ => false,
                };
            }
        }
    }
    
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
        if grid[i.checked_sub(1).unwrap_or(0)][j]{
            neighbours += 1;
        }
        if grid[i][j.checked_sub(1).unwrap_or(0)]{
            neighbours += 1;
        }
        if grid[i.checked_sub(1).unwrap_or(0)][j.checked_sub(1).unwrap_or(0)]{
            neighbours += 1;
        }
        if grid.get(i + 1).unwrap_or(&[false; MAX_BOARD_SIZE])[j.checked_sub(1).unwrap_or(0)]{
            neighbours += 1;
        }
        if *grid[i.checked_sub(1).unwrap_or(0)].get(j + 1).unwrap_or(&false){
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
    
    /*pub fn add_glider(&mut self, x: usize, y: usize){ // places at bottom right
        self.grid.get(x)    .get(y + 1) = true;
        self.grid.get(x + 1).get(y + 2) = true;
        self.grid.get(x + 2).get(y + 2) = true;
        self.grid.get(x + 2).get(y + 1) = true;
        self.grid.get(x + 2).get(y) = true;
    }
    */
    pub fn add_blinker(&mut self, x: usize, y: usize){ // places at bottom right, vertically
        *self.grid.get_mut(x + 1)    .unwrap_or(&mut [false; MAX_BOARD_SIZE]).get_mut(y).unwrap_or(&mut false) = true;
        *self.grid.get_mut(x + 1).unwrap_or(&mut [false; MAX_BOARD_SIZE]).get_mut(y + 1).unwrap_or(&mut false) = true;
        *self.grid.get_mut(x + 1).unwrap_or(&mut [false; MAX_BOARD_SIZE]).get_mut(y + 2).unwrap_or(&mut false) = true;
    }
}