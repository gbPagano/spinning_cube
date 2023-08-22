mod cube;

use rand::Rng;
use std::time::Duration;
use std::{process, thread};
use terminal_size::{terminal_size, Height, Width};
use clap::Parser;

use cube::{Cube, Point};

/// Perspective projection of a rotating cube, using only ascii codes
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///  Camera distance, for a cube perspective
    #[arg(short, long, default_value_t = 11.0)]
    distance: f64,
}

fn main() {
    let args = Args::parse();
    let Some((Width(term_width), Height(term_height))) = terminal_size() else {
        println!("unable to get terminal size");
        process::abort()
    };
    assert!(term_height >= 20, "terminal is too small");
    assert!(term_width >= term_height, "terminal is too small");

    let distance: f64 = args.distance;
    let scale: f64 = distance / 3.0;

    let background_size = ((term_height - 9) as usize, term_width as usize);
    let cube_size: f64 = background_size.0 as f64 * 2.0 * scale;
    let vertical_offset = (background_size.0 / 2) as f64;
    let horizontal_offset = (background_size.1 / 2) as f64;

    let mut cube = Cube::new(cube_size as i16);
    loop {
        let angle_yaw: f64 = rand::thread_rng().gen_range(0.05..=0.15);
        let angle_pitch: f64 = rand::thread_rng().gen_range(0.05..=0.15);
        let angle_roll: f64 = rand::thread_rng().gen_range(0.05..=0.15);

        let mut background: Vec<Vec<char>> = vec![vec![' '; background_size.1]; background_size.0];
        let mut z_buffer: Vec<Vec<f64>> = vec![vec![-1.0; background_size.1]; background_size.0];

        for point in &mut cube.points {
            rotate_axis(point, angle_yaw, angle_pitch, angle_roll);

            let z_depth: f64 = 1.0 / (distance - (point.z / cube_size));

            let idx_x = ((z_depth * point.x).round() / 2.4 + vertical_offset) as usize;
            let idx_y = ((z_depth * point.y).round() + horizontal_offset) as usize;

            if idx_x < background_size.0
                && idx_y < background_size.1
                && z_buffer[idx_x][idx_y] <= z_depth
            {
                z_buffer[idx_x][idx_y] = z_depth;
                background[idx_x][idx_y] = point.mesh;
            }
        }
        print_cube(&background);
        thread::sleep(Duration::from_millis((1000.0 * (distance + scale) / cube_size) as u64));

        clean_background(background_size.0);
    }
}

fn rotate_axis(point: &mut Point, yaw: f64, pitch: f64, roll: f64) {
    rotate_yaw(point, yaw);
    rotate_pitch(point, pitch);
    rotate_roll(point, roll);
}

fn rotate_yaw(point: &mut Point, angle: f64) {
    let new_x = point.x * angle.cos() - point.y * angle.sin();
    let new_y = point.x * angle.sin() + point.y * angle.cos();

    point.x = new_x;
    point.y = new_y;
}

fn rotate_pitch(point: &mut Point, angle: f64) {
    let new_x = point.x * angle.cos() + point.z * angle.sin();
    let new_z = point.z * angle.cos() - point.x * angle.sin();

    point.x = new_x;
    point.z = new_z;
}

fn rotate_roll(point: &mut Point, angle: f64) {
    let new_y = point.y * angle.cos() - point.z * angle.sin();
    let new_z = point.y * angle.sin() + point.z * angle.cos();

    point.y = new_y;
    point.z = new_z;
}

fn clean_background(height: usize) {
    for _ in 0..height {
        // moves the cursor up and clears the line
        print!("\x1B[A\x1B[K");
    }
}

fn print_cube(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for val in row {
            print!("{}", val);
        }
        println!();
    }
}
