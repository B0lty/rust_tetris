use minifb::{Key, KeyRepeat, MouseButton, Window, WindowOptions};

pub mod libs;

use crate::libs::buttons::Button;
use crate::libs::buttons::{self, new_lable};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

const RED: u32 = 0x00ff0000;
const GREEN: u32 = 0x0000ff00;
const BLUE: u32 = 0x000000ff;
const BUTTON_BG_COL: u32 = 0x00222244;

fn main() {
    let mut buffer: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "GUI Template - ESC to exit",
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
        x_pos: 10.0,
        y_pos: 10.0,
        text: String::from("Example Button"),
        border_width: 3.0,
        text_colour: RED,
        bg_colour: BUTTON_BG_COL,
        id: String::from("btn_example"),
        action: buttons::Action::None,
        is_lable: false,
    }];

    // Declaring lables
    // Declared at compile time, can't add more during runtime
    let lables: [Button; 1] = [new_lable(
        200.0,
        10.0,
        String::from("Example lable"),
        3.0,
        RED,
        BUTTON_BG_COL,
        String::from("lbl_example"),
    )];

    let mut button_pressed = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // Resets all pixels to black
        }

        // Key press logic
        {
            let pressed_keys = window.get_keys_pressed(KeyRepeat::No);
            for key in pressed_keys {
                match key {
                    _ => (),
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
                        _ => (),
                    }
                }
            } else {
                button_pressed = false;
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
