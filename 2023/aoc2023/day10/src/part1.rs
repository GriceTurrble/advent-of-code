/// Part 1 solution
pub fn solution(contents: &Vec<&str>) -> u64 {
    let points: Vec<Vec<char>> = contents
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let start = find_start(&points);
    let connections = connected_points(&points, start);
    let mut prev_step = start;
    let mut curr_step = connections[0];
    let mut total = 1;
    while curr_step != start {
        total += 1;
        let connections = connected_points(&points, curr_step);
        // assert_eq!(connections.len(), 2);
        for new_point in connections {
            if new_point != prev_step {
                (prev_step, curr_step) = (curr_step, new_point);
                break;
            }
        }
    }
    total / 2
}

fn find_start(inp: &Vec<Vec<char>>) -> (usize, usize) {
    let (mut i, mut j) = (0, 0);
    for (ii, line) in inp.iter().enumerate() {
        for (jj, chr) in line.iter().enumerate() {
            if chr == &'S' {
                i = ii;
                j = jj;
            }
        }
    }
    (i, j)
}

fn connected_points(all_points: &Vec<Vec<char>>, point: (usize, usize)) -> Vec<(usize, usize)> {
    let chr = all_points.get(point.0).unwrap().get(point.1).unwrap();
    let col_len = all_points.len();
    let row_len = all_points.get(0).unwrap().len();
    let mut connections: Vec<(usize, usize)> = Vec::new();
    let directions = match chr {
        // | is a vertical pipe connecting north and south.
        '|' => "NS",
        // - is a horizontal pipe connecting east and west.
        '-' => "EW",
        // L is a 90-degree bend connecting north and east.
        'L' => "NE",
        // J is a 90-degree bend connecting north and west.
        'J' => "NW",
        // 7 is a 90-degree bend connecting south and west.
        '7' => "SW",
        // F is a 90-degree bend connecting south and east.
        'F' => "SE",
        // S is the starting position of the animal;
        'S' => "start",
        // . is ground; there is no pipe in this tile.
        _ => "",
    };

    if directions == "start" {
        // The start can connect in all directions.
        // However, the rules of the problem state only two valid connections exist from the start.
        // So, recursively check the surrounding points for a valid connection back to *this* point,
        // then report the two points that do.
        let mut surrounding: Vec<(usize, usize)> = Vec::new();
        if point.0 > 0 {
            surrounding.push((point.0 - 1, point.1));
        }
        if point.0 < col_len - 1 {
            surrounding.push((point.0 + 1, point.1));
        }
        if point.1 > 0 {
            surrounding.push((point.0, point.1 - 1));
        }
        if point.1 < row_len - 1 {
            surrounding.push((point.0, point.1 + 1));
        }
        for p in surrounding {
            for r in connected_points(all_points, p) {
                if r == point {
                    connections.push(p);
                }
            }
        }
    } else if directions != "" {
        if directions.contains('N') && point.0 > 0 {
            // Nobody
            connections.push( (point.0 - 1, point.1) );
        }
        if directions.contains('E') && point.1 < row_len - 1 {
            // Enjoys
            connections.push( (point.0, point.1 + 1) );
        }
        if directions.contains('S') && point.0 < col_len - 1 {
            // Soviet
            connections.push( (point.0 + 1, point.1) );
        }
        if directions.contains('W') && point.1 > 0 {
            // Womble
            connections.push( (point.0, point.1 - 1) );
        }
    }

    connections
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data() -> Vec<&'static str> {
        let data = "7-F7-\n.FJ|7\nSJLL7\n|F--J\nLJ.LJ";
        data.trim().split("\n").collect()
    }

    #[test]
    fn test_solution() {
        let result = solution(&get_sample_data());
        let expected = 8;
        assert_eq!(result, expected);
    }
}
