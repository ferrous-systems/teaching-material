//wasm-game-of-life/src/lib.rs

impl Universe {

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // ...
}
