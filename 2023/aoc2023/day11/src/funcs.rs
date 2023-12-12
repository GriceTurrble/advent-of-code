pub fn transpose_grid(contents: Vec<String>) -> Vec<String> {
    let thing: Vec<Vec<char>> = contents
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let transposed = transpose(thing);
    transposed
        .iter()
        .map(|inner| inner.into_iter().collect::<String>())
        .collect()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn find_galaxies(contents: Vec<String>) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = Vec::new();
    for (row, line) in contents.iter().enumerate() {
        for (col, char) in line.chars().into_iter().enumerate() {
            if char == '#' {
                output.push((row, col));
            }
        }
    }
    output
}

pub fn find_empty_rows(grid: &Vec<String>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    for (idx, row) in grid.iter().enumerate() {
        if !row.contains('#') {
            result.push(idx);
        }
    }
    result
}
