use std::thread;
use std::time::Duration;

use minifb::{Key, Window, WindowOptions};

use crate::cube::{Color, Move, RubiksCube};

const CELL: usize = 48;
const GAP: usize = 2;
const MARGIN: usize = 24;
/// Net is 4 faces wide × 3 faces tall.
const NET_W: usize = 4;
const NET_H: usize = 3;
const WIDTH: usize = MARGIN * 2 + NET_W * (3 * CELL + 2 * GAP);
const HEIGHT: usize = MARGIN * 2 + NET_H * (3 * CELL + 2 * GAP) + 40;

fn color_rgb(color: Color) -> u32 {
    match color {
        Color::White => 0x00F5F5F5,
        Color::Blue => 0x002E6BE6,
        Color::Yellow => 0x00F5D000,
        Color::Green => 0x0028A745,
        Color::Red => 0x00E6392E,
        Color::Orange => 0x00F07718,
    }
}

fn fill_rect(buffer: &mut [u32], x: usize, y: usize, w: usize, h: usize, color: u32) {
    for py in y..(y + h).min(HEIGHT) {
        for px in x..(x + w).min(WIDTH) {
            buffer[py * WIDTH + px] = color;
        }
    }
}

fn face_origin(face: usize) -> Option<(usize, usize)> {
    // Net layout:
    //       [4 Red]
    // [0 W][1 B][2 Y][3 G]
    //       [5 Orange]
    match face {
        4 => Some((1, 0)),
        0 => Some((0, 1)),
        1 => Some((1, 1)),
        2 => Some((2, 1)),
        3 => Some((3, 1)),
        5 => Some((1, 2)),
        _ => None,
    }
}

fn draw_cube(buffer: &mut [u32], cube: &RubiksCube) {
    buffer.fill(0x00181820);

    let face_stride = 3 * CELL + 2 * GAP;

    for face in 0..6 {
        let Some((fx, fy)) = face_origin(face) else {
            continue;
        };
        let ox = MARGIN + fx * face_stride;
        let oy = MARGIN + fy * face_stride;

        for r in 0..3 {
            for c in 0..3 {
                let x = ox + c * (CELL + GAP);
                let y = oy + r * (CELL + GAP);
                fill_rect(
                    buffer,
                    x,
                    y,
                    CELL,
                    CELL,
                    color_rgb(cube.faces[face][r][c]),
                );
            }
        }
    }
}

/// Open a window, show the cube, then animate each move of the solution.
pub fn animate_solution(mut cube: RubiksCube, moves: &[Move], delay_ms: u64) {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Rubik's Cube — solution",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )
    .expect("unable to open window");

    window.set_target_fps(60);
    draw_cube(&mut buffer, &cube);
    window
        .update_with_buffer(&buffer, WIDTH, HEIGHT)
        .expect("buffer update");

    // Pause so the scrambled state is visible.
    let pause = InstantGuard::wait(&mut window, &buffer, 600);
    if !pause {
        return;
    }

    for mv in moves {
        if !window.is_open() || window.is_key_down(Key::Escape) {
            return;
        }
        cube.apply(*mv);
        draw_cube(&mut buffer, &cube);
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("buffer update");
        if !InstantGuard::wait(&mut window, &buffer, delay_ms) {
            return;
        }
    }

    // Keep the final solved cube on screen until the window is closed.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("buffer update");
        thread::sleep(Duration::from_millis(16));
    }
}

/// Show a cube without animating (blocks until Escape / close).
pub fn show_cube(cube: &RubiksCube, title: &str) {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut window = Window::new(
        title,
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )
    .expect("unable to open window");

    window.set_target_fps(60);
    draw_cube(&mut buffer, cube);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("buffer update");
        thread::sleep(Duration::from_millis(16));
    }
}

struct InstantGuard;

impl InstantGuard {
    /// Pump the window for about `ms` milliseconds. Returns false if closed.
    fn wait(window: &mut Window, buffer: &[u32], ms: u64) -> bool {
        let frames = (ms / 16).max(1);
        for _ in 0..frames {
            if !window.is_open() || window.is_key_down(Key::Escape) {
                return false;
            }
            window
                .update_with_buffer(buffer, WIDTH, HEIGHT)
                .expect("buffer update");
            thread::sleep(Duration::from_millis(16));
        }
        true
    }
}
