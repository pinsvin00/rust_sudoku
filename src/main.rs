
const N: usize = 9;

use rand::random;

extern crate termion;
use termion::color;
use termion::style;

fn rand_string() -> char {
    return (0x20u8 + (random::<f32>() * 96.0) as u8) as char
}

pub fn is_ok(grid: &[[i16; 9]; 9], col : usize, row: usize, num: i16) -> bool {
    for x in 0..N {
        if grid[col][x] == num {
            return false;
        }
        if grid[x][row] == num {
            return false;
        }
    }

    let corner_x = col - col % 3;
    let corner_y = row - row % 3;

    for x in 0..3 {
        for y in 0..3 {
            if grid[corner_x + x][corner_y + y] == num {
                return false;
            }
        }
    }


    return true;
}


pub fn draw_seperator_x() {
    for _ in 0..3 {
        print!("+");
        print!("-------");
    }
    println!("+");
}

pub fn normal_cursor() {
    print!("{}{}", color::Bg(color::Black) , color::Fg(color::White));
}


pub fn draw_sudoku(grid: &[[i16; N]; N], sel_col: usize, sel_row: usize) {
    print!("{} {}", termion::clear::All, termion::cursor::Goto(1,1));
    for (x,v) in grid.iter().enumerate() {
        
        let draw_sep_x = x % 3 == 0;
        if draw_sep_x {
            draw_seperator_x();
        }

        for (y,i) in v.iter().enumerate() {


            let draw_sep_y = (y % 3) == 0;

            if draw_sep_y {
                print!("| ")
            }


            if x == sel_col && sel_row == y {
                print!("{}{} ", color::Fg(color::Red), i);
                normal_cursor();
            }
            else {
                if i == &0 {
                    print!("{}{}", color::Bg(color::Black) , color::Fg(color::Green));
                    print!("{} ", rand_string());
                    normal_cursor();
                }
                else {
                    print!("{}{} ", color::Fg(color::White), i);
                }
            }


        }
        println!("|");
    }
    draw_seperator_x();
}

pub fn solve_sudoku(grid: &mut [[i16; 9]; 9], mut col: usize, mut row: usize) -> bool {
    draw_sudoku(grid,col,row);
    std::thread::sleep(std::time::Duration::from_millis(50));


    if row == N-1 && col == N {
        return true;
    }

    if col == N {
        col = 0;
        row += 1;
    }

    if grid[col][row] > 0 {
        return solve_sudoku(grid, col+1, row);
    }

    for num in 1..=N {
        if is_ok(&grid, col, row, num as i16) {
            grid[col][row] = num as i16;
            if solve_sudoku(grid, col + 1, row) {
                return true;
            }
        }
        grid[col][row] = 0;
    }


    return false;
}

fn main() {
    let mut sudoku = [
     [3, 0, 6, 5, 0, 8, 4, 0, 0 as i16],
     [5, 2, 0, 0, 0, 0, 0, 0, 0],
     [0, 8, 7, 0, 0, 0, 0, 3, 1],
     [0, 0, 3, 0, 1, 0, 0, 8, 0],
     [9, 0, 0, 8, 6, 3, 0, 0, 5],
     [0, 5, 0, 0, 9, 0, 6, 0, 0],
     [1, 3, 0, 0, 0, 0, 2, 5, 0],
     [0, 0, 0, 0, 0, 0, 0, 7, 4],
     [0, 0, 5, 2, 0, 6, 3, 0, 0]
    ];

    if solve_sudoku(&mut sudoku, 0, 0) {
        println!("Solved!");
    }
    else {
        print!("{}", termion::clear::All);
        println!("No solution.");
    }

}
