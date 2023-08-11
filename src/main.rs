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
    term_height -= 8;
    
    let background: Vec<Vec<char>> = vec![vec!['.'; term_width.into()]; term_height.into()];
    print_cube(&background);
    thread::sleep(Duration::from_millis(360));
    
    clean_cube(term_height);
}


fn clean_cube(height: u16) {
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

