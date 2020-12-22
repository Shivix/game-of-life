use sdl2::pixels;
use sdl2::rect;
use sdl2::render;
use sdl2::render::Canvas;
use sdl2::video::Window;

mod game_board;

fn draw_graphical_board(canvas: &mut render::WindowCanvas, board: &game_board::GameBoard){
    canvas.set_draw_color(pixels::Color::RGB(30, 30, 30));
    canvas.clear();
    canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
    for i in 0..board.grid.len(){
        for j in 0..board.grid.len(){
            if board.grid[i][j] == true {
                canvas.fill_rect(rect::Rect::new(i as i32 * 10, j as i32 * 10, 10, 10));
            }
        }
    }
    canvas.present();
}

fn main() {
    // set up window
    let sdl_context = sdl2::init().expect("Failed to create sdl context");
    let video_subsystem = sdl_context.video().expect("Failed to create video subsystem");

    let window = video_subsystem.window("Game of life", 800, 800)
        .position_centered()
        .vulkan()
        .build()
        .expect("Failed to create window");

    let mut canvas: Canvas<Window> = window.into_canvas()
        .build()
        .expect("Failed to create canvas");
    
    canvas.set_draw_color(pixels::Color::RGB(30, 30, 30));
    canvas.clear();
    canvas.present();
    
    // init board. 2d array<bool>
    let mut board = game_board::GameBoard::new();
    canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
    loop {
        std::thread::sleep(std::time::Duration::from_millis(300));
        draw_graphical_board(&mut canvas, &board);
        board.next_generation();
    }
    std::io::stdin().read_line(&mut String::new());
}
