use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

type StateMatrix = Vec<Vec<bool>>;
type MatrixBound = (usize, usize, usize, usize);

fn seed_state(row: usize, col: usize) -> StateMatrix {
    let mut state = Vec::with_capacity(row);
    for _ in 0..row {
        let mut col_state = Vec::with_capacity(col);
        for _ in 0..col {
            col_state.push(rand::random())
        }
        state.push(col_state)
    }
    return state;
}

fn get_bounds(row_len: usize, col_len: usize, row_index: usize, col_index: usize) -> MatrixBound {
    let (mut row_start, mut row_end, mut col_start, mut col_end): MatrixBound =
        (row_index, row_index, col_index, col_index);
    if row_index > 0 {
        row_start -= 1;
    }
    if col_index > 0 {
        col_start -= 1;
    }
    if row_index < (row_len - 1) {
        row_end += 1;
    }
    if col_index < (col_len - 1) {
        col_end += 1;
    }
    return (row_start, row_end, col_start, col_end);
}

fn tick(state: StateMatrix) -> StateMatrix {
    let row: usize = state.len();
    let col: usize = state[0].len();
    let mut new_state: StateMatrix = Vec::with_capacity(row);
    for row_index in 0..row {
        let mut new_col_state = Vec::with_capacity(col);
        for col_index in 0..col {
            let mut is_alive = state[row_index][col_index];
            let mut living_neighbours_count = 0;
            let (row_start, row_end, col_start, col_end) =
                get_bounds(row, col, row_index, col_index);
            // Count the live neighbours + current cell
            for row_neighbour in row_start..row_end + 1 {
                for col_neignbour in col_start..col_end + 1 {
                    if state[row_neighbour][col_neignbour] {
                        living_neighbours_count += 1;
                    }
                }
            }
            if is_alive {
                // if current cell is alive, remove it from count
                living_neighbours_count -= 1;
                if (living_neighbours_count < 2) | (living_neighbours_count > 3) {
                    is_alive = false;
                }
            } else {
                if living_neighbours_count == 3 {
                    is_alive = true
                }
            }
            new_col_state.push(is_alive)
        }
        new_state.push(new_col_state)
    }
    return new_state;
}

fn print_vec(state: &StateMatrix) {
    let row: usize = state.len();
    let col: usize = state[0].len();
    for row_index in 0..row {
        for col_index in 0..col {
            if state[row_index][col_index] {
                print!("X");
            } else {
                print!(" ")
            }
        }
        println!("");
    }
    println!("=============================");
}

fn main() {
    let row: usize = 50;
    let col: usize = 50;
    let mut state: StateMatrix = seed_state(row, col);
    loop {
        print_vec(&state);
        state = tick(state);
        sleep(Duration::from_millis(500));
    }
}
