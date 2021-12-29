use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

type StateMatrix = Vec<Vec<bool>>;
type Cell = (usize, usize);
type Neighbours = Vec<Cell>;

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

fn get_neighbours(
    row_len: usize,
    col_len: usize,
    row_index: usize,
    col_index: usize,
) -> Neighbours {
    let (row_curr, mut row_next) = (row_index, row_index + 1);
    let (col_curr, mut col_next) = (col_index, col_index + 1);
    let (mut row_prev, mut col_prev): (usize, usize);
    if row_index > 0 {
        row_prev = row_index - 1
    } else {
        row_prev = row_len - 1;
    }
    if row_next == row_len {
        row_next = 0;
    }
    if col_index > 0 {
        col_prev = col_index - 1;
    } else {
        col_prev = col_len - 1;
    }
    if col_next == col_len {
        col_next = 0;
    }
    return vec![
        (row_prev, col_prev), // top left
        (row_prev, col_curr), // top center
        (row_prev, col_next), // top right
        (row_curr, col_prev), // left
        (row_curr, col_next), // right
        (row_next, col_prev), // down left
        (row_next, col_curr), // down center
        (row_next, col_next), // down right
    ];
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
            let neighbours = get_neighbours(row, col, row_index, col_index);
            for neighbour in neighbours {
                if state[neighbour.0][neighbour.1] {
                    living_neighbours_count += 1;
                }
            }
            if is_alive {
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
