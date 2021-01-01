use sdl2::{pixels, rect, render, video, mouse, event};
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::event::Event;
use crate::game_board::{GameBoard, MAX_BOARD_SIZE};

mod game_board;

#[test]
fn blinker_test(){
    let mut test_board = GameBoard{generation: 0, grid: [[false; MAX_BOARD_SIZE]; MAX_BOARD_SIZE]};
    test_board.add_blinker(1, 1);
    test_board.next_generation();
    assert_eq!(test_board.grid[1][2], true);
    assert_eq!(test_board.grid[2][2], true);
    assert_eq!(test_board.grid[3][2], true);
    assert_eq!(test_board.grid[2][1], false);
}

fn run(event_pump: &mut EventPump, canvas: &mut Canvas<Window>, board: &mut GameBoard){
    'main: loop {
        for event in &mut event_pump.poll_iter(){
            use sdl2::event;
            match event {
                event::Event::KeyDown {..} => continue 'main,
                _ => (),
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
        draw_graphical_board(canvas, &board);
        board.next_generation();
    }
}

fn draw_graphical_board(canvas: &mut WindowCanvas, board: &game_board::GameBoard){
    canvas.set_draw_color(pixels::Color::RGB(30, 30, 30));
    canvas.clear();
    canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
    for i in 0..board.grid.len(){
        for j in 0..board.grid.len(){
            if board.grid[i][j] == true {
                canvas.fill_rect(rect::Rect::new(i as i32 * 10, j as i32 * 10, 10, 10))
                    .expect("Failed to create rectangle");
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
    
    let mut event_pump = sdl_context.event_pump().expect("Failed to create event pump");
    // init board. 2d array<bool>
    let mut board = game_board::GameBoard::new();
    canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
    
    run(&mut event_pump, &mut canvas, &mut board);
}
