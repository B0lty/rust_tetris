use minifb::{Key, KeyRepeat, MouseButton, Window, WindowOptions};
use rand::RngExt;

pub mod libs;

use crate::libs::buttons::{self, Button, new_lable};
use crate::libs::dd_drawing::{draw_line, draw_rect};

const WIDTH: usize = 550;
const HEIGHT: usize = 640;

const RED: u32 = 0x00ff0000;
// const GREEN: u32 = 0x0000ff00;
// const BLUE: u32 = 0x000000ff;
const BUTTON_BG_COL: u32 = 0x00222244;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Tetris - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    // Declaring buttons
    // Declared at compile time, can't add more during runtime
    let buttons: [Button; 1] = [Button {
        x_pos: 50.0,
        y_pos: 20.0,
        text: String::from("Restart Game"),
        border_width: 3.0,
        text_colour: RED,
        bg_colour: BUTTON_BG_COL,
        id: String::from("btn_restart"),
        action: buttons::Action::Restart,
        is_lable: false,
    }];

    // Declaring lables
    // Declared at compile time, can't add more during runtime
    let mut lables: [Button; 1] = [new_lable(
        200.0,
        20.0,
        String::from("Score: 0"),
        3.0,
        RED,
        BUTTON_BG_COL,
        String::from("lbl_score"),
    )];

    let mut button_pressed = false;

    // 0: blank
    // 1: cyan
    // 2: blue
    // 3: orange
    // 4: yellow
    // 5: green
    // 6: purple
    // 7: red
    let mut game_board: [[u8; 10]; 20] = [[0; 10]; 20];

    let mut placed_pieces: [[u8; 10]; 20] = [[0; 10]; 20];

    let mut next_piece_display: [[u8; 5]; 16] = [[0; 5]; 16];

    let tile_size = 28;
    let board_offset = [42, 50];
    let next_piece_offset = [42 + tile_size * 10 + 50, 105];

    #[derive(Clone, Copy)]
    struct Tetromino {
        piece: [[i32; 2]; 4],
        center: [i32; 2],
        col: u8,
    }

    impl Tetromino {
        fn rotate(&mut self) {
            // Exact 90° clockwise rotation about `center`. The pivot never moves.
            let [cx, cy] = self.center;

            for cell in &mut self.piece {
                let dx = cell[0] - cx;
                let dy = cell[1] - cy;
                cell[0] = cx - dy; // (dx, dy) -> (-dy, dx)
                cell[1] = cy + dx;
            }
        }

        fn translate(&mut self, x: i32, y: i32) {
            self.center[0] += x;
            self.center[1] += y;
            for pt in &mut self.piece {
                pt[0] += x;
                pt[1] += y;
            }
        }
    }

    let tetrominoes: [Tetromino; 7] = [
        Tetromino {
            piece: [[0, 0], [0, 1], [0, 2], [0, 3]],
            center: [0, 1],
            col: 1,
        },
        Tetromino {
            piece: [[1, 0], [1, 1], [1, 2], [0, 2]],
            center: [1, 1],
            col: 2,
        },
        Tetromino {
            piece: [[0, 0], [0, 1], [0, 2], [1, 2]],
            center: [0, 1],
            col: 3,
        },
        Tetromino {
            piece: [[0, 0], [0, 1], [1, 0], [1, 1]],
            center: [0, 0],
            col: 4,
        },
        Tetromino {
            piece: [[1, 0], [2, 0], [0, 1], [1, 1]],
            center: [1, 0],
            col: 5,
        },
        Tetromino {
            piece: [[0, 0], [1, 0], [2, 0], [1, 1]],
            center: [1, 0],
            col: 6,
        },
        Tetromino {
            piece: [[0, 0], [1, 0], [1, 1], [2, 1]],
            center: [1, 0],
            col: 7,
        },
    ];

    let mut current_piece: Tetromino;

    current_piece = tetrominoes[new_tetromino() as usize];

    let mut next_pieces: [u32; 3] = [0; 3];

    next_pieces[0] = new_tetromino() as u32;
    next_pieces[1] = new_tetromino() as u32;
    next_pieces[2] = new_tetromino() as u32;

    fn new_tetromino() -> i32 {
        let mut rng = rand::rng();
        return rng.random_range(0..=6);
    }

    fn inc_tetromino(next_pieces: &mut [u32; 3]) -> u32 {
        let tmp_index = next_pieces[0];

        next_pieces[0] = next_pieces[1];
        next_pieces[1] = next_pieces[2];
        next_pieces[2] = new_tetromino() as u32;

        return tmp_index;
    }

    let mut timer_1s = 0;

    let mut score: u128 = 0;

    fn update_score(lables: &mut [Button], score: &u128) {
        for lable in lables {
            if lable.id == "lbl_score" {
                lable.text = format!("Score: {}", score)
            }
        }
    }

    let mut alive = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // Resets all pixels to black
        }

        // the game only runs if the player is alive
        if alive == true {
            // timer
            {
                timer_1s += 1;

                if timer_1s > window.get_fps() {
                    timer_1s = 0;
                }
            }

            // Key press logic
            {
                let pressed_keys = window.get_keys_pressed(KeyRepeat::No);
                for key in pressed_keys {
                    match key {
                        Key::Up => {
                            current_piece.rotate();

                            let mut out_of_bounds = false;
                            for block in current_piece.piece {
                                if block[0] < 0
                                    || block[0] >= 10
                                    || block[1] < 0
                                    || block[1] >= 20
                                    || placed_pieces[block[1] as usize][block[0] as usize] != 0
                                {
                                    out_of_bounds = true;
                                }
                            }
                            if out_of_bounds == true {
                                current_piece.rotate();
                                current_piece.rotate();
                                current_piece.rotate();
                            }
                        }
                        Key::Down => {
                            let mut collided = false;
                            for block in current_piece.piece {
                                if (block[1] >= 19)
                                    || (placed_pieces[(block[1] + 1) as usize][block[0] as usize]
                                        != 0)
                                {
                                    collided = true;
                                }
                            }

                            if collided == true {
                                for block in current_piece.piece {
                                    placed_pieces[block[1] as usize][block[0] as usize] =
                                        current_piece.col;
                                }
                                current_piece =
                                    tetrominoes[inc_tetromino(&mut next_pieces) as usize];
                            } else {
                                current_piece.translate(0, 1);

                                let mut collided = false;
                                for block in current_piece.piece {
                                    if (block[1] >= 19)
                                        || (placed_pieces[(block[1] + 1) as usize]
                                            [block[0] as usize]
                                            != 0)
                                    {
                                        collided = true;
                                    }
                                }

                                if collided == true {
                                    for block in current_piece.piece {
                                        placed_pieces[block[1] as usize][block[0] as usize] =
                                            current_piece.col;
                                    }
                                    current_piece =
                                        tetrominoes[inc_tetromino(&mut next_pieces) as usize];
                                }
                            }
                        }
                        Key::Right => {
                            current_piece.translate(1, 0);

                            let mut out_of_bounds = false;
                            for block in current_piece.piece {
                                if block[0] < 0
                                    || block[0] >= 10
                                    || block[1] < 0
                                    || block[1] >= 20
                                    || placed_pieces[block[1] as usize][block[0] as usize] != 0
                                {
                                    out_of_bounds = true;
                                }
                            }
                            if out_of_bounds == true {
                                current_piece.translate(-1, 0);
                            }
                        }
                        Key::Left => {
                            current_piece.translate(-1, 0);

                            let mut out_of_bounds = false;
                            for block in current_piece.piece {
                                if block[0] < 0
                                    || block[0] >= 10
                                    || block[1] < 0
                                    || block[1] >= 20
                                    || placed_pieces[block[1] as usize][block[0] as usize] != 0
                                {
                                    out_of_bounds = true;
                                }
                            }
                            if out_of_bounds == true {
                                current_piece.translate(1, 0);
                            }
                        }
                        Key::Space => {
                            let mut collided = false;

                            while collided == false {
                                for block in current_piece.piece {
                                    if (block[1] >= 19)
                                        || (placed_pieces[(block[1] + 1) as usize]
                                            [block[0] as usize]
                                            != 0)
                                    {
                                        collided = true;
                                    }
                                }
                                if collided == false {
                                    current_piece.translate(0, 1)
                                }
                            }

                            if collided == true {
                                for block in current_piece.piece {
                                    placed_pieces[block[1] as usize][block[0] as usize] =
                                        current_piece.col;
                                }
                                current_piece =
                                    tetrominoes[inc_tetromino(&mut next_pieces) as usize];
                            }
                        }
                        _ => (),
                    }
                }
            }

            // next piece logic
            {
                let mut next_piece_buffer: [[u8; 5]; 16] = [[0; 5]; 16];
                let piece_offset = [1, 1];
                let mut i = 0;

                // rendering tetrominoes
                for piece in next_pieces {
                    for pt in tetrominoes[piece as usize].piece {
                        let x = pt[0] + piece_offset[0];
                        let y = pt[1] + piece_offset[1] + i;

                        if x >= 0 && x < 10 && y >= 0 && y < 20 {
                            next_piece_buffer[y as usize][x as usize] =
                                tetrominoes[piece as usize].col;
                        }
                    }
                    i += 5;
                }

                // drawing to main buffer
                for y in 0..next_piece_display.len() {
                    for x in 0..next_piece_display[y].len() {
                        // Drawing the buffer to the game board
                        if next_piece_display[y][x] != next_piece_buffer[y][x] {
                            next_piece_display[y][x] = next_piece_buffer[y][x];
                        }
                    }
                }
            }

            // game logic
            {
                let mut game_board_buffer: [[u8; 10]; 20] = [[0; 10]; 20];

                // rendering current tetromino
                {
                    for pt in current_piece.piece {
                        let x = pt[0];
                        let y = pt[1];

                        if placed_pieces[y as usize][x as usize] != 0 {
                            alive = false;
                        }

                        if x >= 0 && x < 10 && y >= 0 && y < 20 {
                            game_board_buffer[y as usize][x as usize] = current_piece.col;
                        }
                    }
                }

                // move down code
                if timer_1s % 60 == 0 {
                    // downward collision code
                    let mut collided = false;
                    for block in current_piece.piece {
                        if (block[1] >= 19)
                            || (placed_pieces[(block[1] + 1) as usize][block[0] as usize] != 0)
                        {
                            collided = true;
                        }
                    }

                    if collided == true {
                        for block in current_piece.piece {
                            placed_pieces[block[1] as usize][block[0] as usize] = current_piece.col;
                        }
                        current_piece = tetrominoes[inc_tetromino(&mut next_pieces) as usize];
                    } else {
                        current_piece.translate(0, 1);
                    }
                }

                // full row detection & handling & scoring
                {
                    let mut rows_cleared: u8 = 0;
                    for y in 0..placed_pieces.len() {
                        let mut row_filled = 0;
                        for x in 0..placed_pieces[y].len() {
                            if placed_pieces[y][x] != 0 {
                                row_filled += 1;
                            }
                        }

                        if row_filled >= 10 {
                            rows_cleared += 1;

                            for i in 0..(y as i32 - 1) {
                                placed_pieces[(y as i32 - i as i32) as usize] =
                                    placed_pieces[(y as i32 - i as i32 - 1) as usize];
                            }
                        }
                    }

                    match rows_cleared {
                        1 => score += 100,
                        2 => score += 300,
                        3 => score += 500,
                        4 => score += 800,
                        _ => (),
                    }
                    update_score(&mut lables, &score);
                }

                // drawing to main buffer
                for y in 0..game_board.len() {
                    for x in 0..game_board[y].len() {
                        // Drawing the buffer to the game board
                        if game_board[y][x] != game_board_buffer[y][x] {
                            game_board[y][x] = game_board_buffer[y][x];
                        }

                        // Drawing the placed pieces to the game board
                        if placed_pieces[y][x] != 0 {
                            game_board[y][x] = placed_pieces[y][x];
                        }
                    }
                }
            }
        }

        // Button logic
        {
            if window.get_mouse_down(MouseButton::Left) {
                if button_pressed == false {
                    let button_action: buttons::Action;

                    button_pressed = true;
                    button_action = buttons::button_pressed(&window, &buttons);

                    match button_action {
                        buttons::Action::Restart => {
                            alive = true;

                            game_board = [[0; 10]; 20];
                            placed_pieces = [[0; 10]; 20];
                            current_piece = tetrominoes[inc_tetromino(&mut next_pieces) as usize];
                        }
                        _ => (),
                    }
                }
            } else {
                button_pressed = false;
            }
        }

        // game rendering
        {
            // main board rendering
            for y in 0..game_board.len() {
                for x in 0..game_board[y].len() {
                    let x1 = (x * tile_size + board_offset[0]) as i32;
                    let y1 = (y * tile_size + board_offset[1]) as i32;
                    let x2 = (x * tile_size + tile_size - 1 + board_offset[0]) as i32;
                    let y2 = (y * tile_size + tile_size - 1 + board_offset[1]) as i32;

                    let col: u32;

                    match game_board[y][x] {
                        0 => col = 0x00101010,
                        1 => col = 0x002bdcfb,
                        2 => col = 0x000000ff,
                        3 => col = 0x00fba82b,
                        4 => col = 0x00e5ff00,
                        5 => col = 0x0000ff00,
                        6 => col = 0x007100ff,
                        7 => col = 0x00ff0000,
                        _ => col = 0x00000000,
                    };

                    // drawing blocks
                    draw_rect(&mut buffer, x1, y1, x2, y1, x2, y2, x1, y2, col);
                    // drawing accents
                    draw_line(&mut buffer, x1, y1, x1, y2, (col as f32 * 1.5) as u32);
                    draw_line(&mut buffer, x1, y2, x2, y2, (col as f32 * 1.5) as u32);
                    // drawing shadows
                    draw_line(&mut buffer, x1, y1, x2, y1, (col as f32 * 0.75) as u32);
                    draw_line(&mut buffer, x2, y1, x2, y2, (col as f32 * 0.75) as u32);
                }
            }

            // next piece display rendering
            for y in 0..next_piece_display.len() {
                for x in 0..next_piece_display[y].len() {
                    let x1 = (x * tile_size + next_piece_offset[0]) as i32;
                    let y1 = (y * tile_size + next_piece_offset[1]) as i32;
                    let x2 = (x * tile_size + tile_size - 1 + next_piece_offset[0]) as i32;
                    let y2 = (y * tile_size + tile_size - 1 + next_piece_offset[1]) as i32;

                    let col: u32;

                    match next_piece_display[y][x] {
                        0 => col = 0x00101010,
                        1 => col = 0x002bdcfb,
                        2 => col = 0x000000ff,
                        3 => col = 0x00fba82b,
                        4 => col = 0x00e5ff00,
                        5 => col = 0x0000ff00,
                        6 => col = 0x007100ff,
                        7 => col = 0x00ff0000,
                        _ => col = 0x00000000,
                    };

                    // drawing blocks
                    draw_rect(&mut buffer, x1, y1, x2, y1, x2, y2, x1, y2, col);
                    // drawing accents
                    draw_line(&mut buffer, x1, y1, x1, y2, (col as f32 * 1.5) as u32);
                    draw_line(&mut buffer, x1, y2, x2, y2, (col as f32 * 1.5) as u32);
                    // drawing shadows
                    draw_line(&mut buffer, x1, y1, x2, y1, (col as f32 * 0.75) as u32);
                    draw_line(&mut buffer, x2, y1, x2, y2, (col as f32 * 0.75) as u32);
                }
            }
        }

        // Rendering buttons & lables
        {
            buttons::render_buttons(&buttons, &mut buffer);
            buttons::render_buttons(&lables, &mut buffer);
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
