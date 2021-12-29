use std::vec::Vec;

type StateMatrix = Vec<Vec<bool>>;

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

fn main() {
    let row: usize = 5;
    let col: usize = 5;
    let state: StateMatrix = seed_state(row, col);
    println!("state: {:X?}", state);
}
