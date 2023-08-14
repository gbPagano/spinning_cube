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
    loop {
        let mut background: Vec<Vec<char>> = vec![vec!['.'; term_width.into()]; term_height.into()];
       
        let cube_size: i8 = 8;
        for x in -cube_size..=cube_size {
            for y in -cube_size..=cube_size {
                let x1 = x as f64 * a.cos() - y as f64 * a.sin();
                let y2 = x as f64 * a.sin() + y as f64 * a.cos();
                
                // println!("{x}, {y}");
                // println!("{x1}, {y2}");

                let idx_x = ((x1 / 2.0).round() + 9.0) as usize;
                let idx_y = (y2.round() + 19.0) as usize;
                background[idx_x][idx_y] = '#';
            }
        }
        a += 0.05;
        print_cube(&background);
        thread::sleep(Duration::from_millis(100));
    
        clean_terminal(term_height);
    }
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

