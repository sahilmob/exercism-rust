fn find_left(minefield: &[&str], i: usize, j: usize) -> i32 {
    if j == 0 {
        0
    } else if minefield[i].as_bytes()[j - 1] == 42 {
        1
    } else {
        0
    }
}

fn find_right(minefield: &[&str], i: usize, j: usize) -> i32 {
    if j == minefield[i].as_bytes().len() - 1 {
        0
    } else if minefield[i].as_bytes()[j + 1] == 42 {
        1
    } else {
        0
    }
}

fn find_top(minefield: &[&str], i: usize, j: usize) -> i32 {
    if i == 0 {
        0
    } else if minefield[i - 1].as_bytes()[j] == 42 {
        1
    } else {
        0
    }
}

fn find_bottom(minefield: &[&str], i: usize, j: usize) -> i32 {
    if i == minefield.len() - 1 {
        0
    } else if minefield[i + 1].as_bytes()[j] == 42 {
        1
    } else {
        0
    }
}

fn find_top_left(minefield: &[&str], i: usize, j: usize) -> i32 {
    if i == 0 || j == 0 {
        0
    } else if minefield[i - 1].as_bytes()[j - 1] == 42 {
        1
    } else {
        0
    }
}

fn find_top_right(minefield: &[&str], i: usize, j: usize) -> i32 {
    if i == 0 || j == minefield[i - 1].as_bytes().len() - 1 {
        0
    } else if minefield[i - 1].as_bytes()[j + 1] == 42 {
        1
    } else {
        0
    }
}

fn find_bottom_left(minefield: &[&str], i: usize, j: usize) -> i32 {
    if i == minefield.len() - 1 || j == 0 {
        0
    } else if minefield[i + 1].as_bytes()[j - 1] == 42 {
        1
    } else {
        0
    }
}

fn find_bottom_right(minefield: &[&str], i: usize, j: usize) -> i32 {
    if i == minefield.len() - 1 || j == minefield[i + 1].as_bytes().len() - 1 {
        0
    } else if minefield[i + 1].as_bytes()[j + 1] == 42 {
        1
    } else {
        0
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for (i, row) in minefield.iter().enumerate() {
        let mut row_result = String::from("");
        for (j, col) in row.as_bytes().iter().enumerate() {
            if *col == 42 {
                row_result.push('*');
            } else {
                let count = find_left(minefield, i, j)
                    + find_right(minefield, i, j)
                    + find_top(minefield, i, j)
                    + find_bottom(minefield, i, j)
                    + find_top_left(minefield, i, j)
                    + find_top_right(minefield, i, j)
                    + find_bottom_left(minefield, i, j)
                    + find_bottom_right(minefield, i, j);
                if count > 0 {
                    row_result.push_str(count.to_string().as_str());
                } else {
                    row_result.push(' ');
                }
            }
        }
        result.push(row_result);
    }

    result
}
