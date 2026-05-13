pub struct Board {
    const GRID_SIZE: i32 = 3;
    let matrix: [[i32;GRID_SIZE]; GRID_SIZE] = [[0;GRID_SIZE] ; GRID_SIZE];
}

impl Board {

    fn get_position(&self, x: i32, y: i32){
        assert!(x >= self.GRID_SIZE || x < 0 || y >= self.GRID_SIZE || y < 0, "Position out of bounds");
        return self.matrix[x][y]
    }

}