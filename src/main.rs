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
    // term_height -= 9;
    println!("{term_height}, {term_width}");

    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut c: f64 = 0.0;
    // let a = 45.0_f64.to_radians();
    loop {
        let mut background: Vec<Vec<char>> = vec![vec![' '; term_width.into()]; term_height.into()];
        let mut z_buffer: Vec<Vec<f64>> = vec![vec![-1000.0; term_width.into()]; term_height.into()];
       
        let cube_size: i8 = 40;
        for x in -cube_size..=cube_size {
            for y in -cube_size..=cube_size {
                let z = cube_size;
                
                let (idx_x, idx_y, new_z) = rotate_axis(x, y, z, a, b, c);
                if idx_x < 36 && idx_y < 170 && z_buffer[idx_x][idx_y] <= new_z {
                    z_buffer[idx_x][idx_y] = new_z;
                    background[idx_x][idx_y] = '#';
                }
                let (idx_x, idx_y, new_z) = rotate_axis(-z, y, x, a, b, c);
                if idx_x < 36 && idx_y < 170 && z_buffer[idx_x][idx_y] <= new_z {
                    z_buffer[idx_x][idx_y] = new_z;
                    background[idx_x][idx_y] = '$';
                } 
                let (idx_x, idx_y, new_z) = rotate_axis(z, y, -x, a, b, c);
                if idx_x < 36 && idx_y < 170 && z_buffer[idx_x][idx_y] <= new_z {
                    z_buffer[idx_x][idx_y] = new_z;
                    background[idx_x][idx_y] = '~';
                } 
                let (idx_x, idx_y, new_z) = rotate_axis(-x, y, -z, a, b, c);
                if idx_x < 36 && idx_y < 170 && z_buffer[idx_x][idx_y] <= new_z {
                    z_buffer[idx_x][idx_y] = new_z;
                    background[idx_x][idx_y] = ';';
                } 
                let (idx_x, idx_y, new_z) = rotate_axis(x, z, -y, a, b, c);
                if idx_x < 36 && idx_y < 170 && z_buffer[idx_x][idx_y] <= new_z {
                    z_buffer[idx_x][idx_y] = new_z;
                    background[idx_x][idx_y] = '@';
                }
                let (idx_x, idx_y, new_z) = rotate_axis(x, -z, y, a, b, c);
                if idx_x < 36 && idx_y < 170 && z_buffer[idx_x][idx_y] <= new_z {
                    z_buffer[idx_x][idx_y] = new_z;
                    background[idx_x][idx_y] = '+';
                }
            }
        }
        a += 0.05;
        b += 0.05;
        c += 0.01;
        print_cube(&background);
        thread::sleep(Duration::from_millis(80));
    
        clean_terminal(term_height);
    }
}

fn rotate_axis(x: i8, y: i8, z: i8, yaw: f64, pitch: f64, roll: f64) -> (usize, usize, f64) {

    let (x, y, z) = rotate_yaw(x.into(), y.into(), z.into(), yaw);
    let (x, y, z) = rotate_pitch(x.into(), y.into(), z.into(), pitch);
    let (x, y, z) = rotate_roll(x.into(), y.into(), z.into(), roll);

    // let idx_x = (((x + 1.0) / 2.2).round() + 9.0) as usize;
    // println!("coords: {x},{y},{z}");
    let distance: f64 = 100.0;
    let cube_size: f64 = 40.0;
    let ooz: f64 = (1.0 / (distance - (z / cube_size))) * cube_size;
    // let distance: f64 = 110.0;
    // let ooz: f64 = (cube_size / (distance - z)); 

    // let ooz: f64 = 1.0;
    // println!("ooz: {ooz}");
    // println!("ooz*x: {}", ooz * x);
    
    


    let idx_x = ((ooz * x).round() / 2.0 + 18.0) as usize;
    let idx_y = ((ooz * y).round() + 86.0) as usize;
    (idx_x, idx_y, z)
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

