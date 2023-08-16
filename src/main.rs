use terminal_size::{Width, Height, terminal_size};
use std::{process, thread};
use std::time::Duration;


fn main() {
    
    let Some((Width(term_width), Height(mut term_height))) = terminal_size() else {
        println!("unable to get terminal size");
        process::abort()
    };
    assert!(term_height > 10, "terminal height too low, needs to be at least 10 chars");
    assert!(term_width >= term_height, "terminal width must be at least equal to height");
    term_height -= 9;
    
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut c: f64 = 0.0;
    // let a = 45.0_f64.to_radians();
    loop {
        let mut background: Vec<Vec<char>> = vec![vec!['.'; term_width.into()]; term_height.into()];
       
        let cube_size: i8 = 8;
        for x in -cube_size..=cube_size {
            for y in -cube_size..=cube_size {
                
                let z = cube_size;

                let (x, y, z) = rotate_yaw(x.into(), y.into(), z.into(), a);
                let (x, y, z) = rotate_roll(x.into(), y.into(), z.into(), b);
                let (x, y, z) = rotate_pitch(x.into(), y.into(), z.into(), c);

                // println!("{x}, {y}");
                // println!("{x1}, {y2}");

                let idx_x = (((x + 1.0) / 2.2).round() + 9.0) as usize;
                let idx_x = ((x / 2.0).round() + 9.0) as usize;
                let idx_y = (y.round() + 19.0) as usize;
                background[idx_x][idx_y] = '#';
                

                let z = -cube_size;

                let (x, y, z) = rotate_yaw(x.into(), y.into(), z.into(), a);
                let (x, y, z) = rotate_roll(x.into(), y.into(), z.into(), b);
                let (x, y, z) = rotate_pitch(x.into(), y.into(), z.into(), c);

                // println!("{x}, {y}");
                // println!("{x1}, {y2}");

                let idx_x = (((x + 1.0) / 2.2).round() + 9.0) as usize;
                let idx_x = ((x / 2.0).round() + 9.0) as usize;
                let idx_y = (y.round() + 19.0) as usize;
                background[idx_x][idx_y] = '$';
            }
        }
        a += 0.05;
        b += 0.03;
        c += 0.02;
        print_cube(&background);
        thread::sleep(Duration::from_millis(100));
    
        clean_terminal(term_height);
    }
}

fn rotate_yaw(x: f64, y: f64, z: f64, angle: f64) -> (f64, f64, f64) {
    let new_x = x * angle.cos() - y * angle.sin(); 
    let new_y = x * angle.sin() + y * angle.cos();
    (new_x, new_y, z)
}

fn rotate_pitch(x: f64, y: f64, z: f64, angle: f64) -> (f64, f64, f64) {
    let new_x = x * angle.cos() + z * angle.sin(); 
    let new_z = z * angle.cos() - x * angle.sin();
    (new_x, y, new_z)
}

fn rotate_roll(x: f64, y: f64, z: f64, angle: f64) -> (f64, f64, f64) {    
    let new_y = y * angle.cos() - z * angle.sin(); 
    let new_z = y * angle.sin() + z * angle.cos();
    (x, new_y, new_z)
}

fn clean_terminal(height: u16) {
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

