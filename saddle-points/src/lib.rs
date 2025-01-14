pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();
    let row_count = input.len();
    for (row_num, row) in input.iter().enumerate() {
        for (col_num, val) in row.iter().enumerate() {
            if row.iter().all(|x| x <= val) && (0..row_count).all(|x| input[x][col_num] >= *val) {
                saddle_points.push((row_num, col_num));
            }
        }
    }
    saddle_points
}
