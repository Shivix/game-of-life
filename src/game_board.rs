pub const MAX_BOARD_SIZE: usize = 80;

pub struct GameBoard{
    pub generation: i32,
    pub grid: [[bool; MAX_BOARD_SIZE]; MAX_BOARD_SIZE],
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
                if !previous_gen[i][j] {
                    if neighbours == 3 {
                        self.grid[i][j] = true;
                    }
                }
                else{
                    if neighbours >= 2 && neighbours <= 3 {
                        self.grid[i][j] = true;
                    }
                    else {
                        self.grid[i][j] = false;
                    }
                }
            }
        }
    }
    
    fn count_neighbours(grid: &[[bool; MAX_BOARD_SIZE]; MAX_BOARD_SIZE], i: usize, j: usize) -> i32{
        let mut neighbours = 0;
        if *grid.get(i + 1).unwrap_or(&[false; MAX_BOARD_SIZE]).get(j).unwrap_or(&false){
            neighbours += 1;
        }
        if *grid.get(i).unwrap_or(&[false; MAX_BOARD_SIZE]).get(j + 1).unwrap_or(&false){
            neighbours += 1;
        }
        if *grid.get(i + 1).unwrap_or(&[false; MAX_BOARD_SIZE]).get(j + 1).unwrap_or(&false){
            neighbours += 1;
        }
        if *grid.get(i - 1).unwrap_or(&[false; MAX_BOARD_SIZE]).get(j).unwrap_or(&false){
            neighbours += 1;
        }
        if *grid.get(i).unwrap_or(&[false; MAX_BOARD_SIZE]).get(j - 1).unwrap_or(&false){
            neighbours += 1;
        }
        if *grid.get(i - 1).unwrap_or(&[false; MAX_BOARD_SIZE]).get(j - 1).unwrap_or(&false){
            neighbours += 1;
        }
        if *grid.get(i + 1).unwrap_or(&[false; MAX_BOARD_SIZE]).get(j - 1).unwrap_or(&false){
            neighbours += 1;
        }
        if *grid.get(i - 1).unwrap_or(&[false; MAX_BOARD_SIZE]).get(j + 1).unwrap_or(&false){
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
    
    pub fn add_blinker(&mut self, x: usize, y: usize){ // places at bottom right
        self.grid.get(x)    .get(y + 1) = true;
        self.grid.get(x + 1).get(y + 1) = true;
        self.grid.get(x + 2).get(y + 1) = true;
    }*/
}