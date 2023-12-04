use advent_of_code_2023::utils::*;

fn main() {
    let input = lines_to_vector(read_in_data("03.0.1"));
    let mut grid: Vec<Vec<char>> = vec![];

    for line in input {
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    let result = scan_and_sum(&grid);

    println!("{:?}", grid);
}

struct ScanParameters {
    min_w: usize,
    max_w: usize,
    min_d: usize,
    max_d: usize,
}

fn scan_and_sum(grid: &Vec<Vec<char>>) -> i64 {
    let result: i64 = 0;
    let params = ScanParameters {
        min_w: 0 as usize,
        max_w: grid.len() - 1,
        min_d: 0 as usize,
        max_d: grid[0].len() - 1,
    };

    let mut value = String::new();
    let mut value_included = false;

    for line_no in params.min_w..params.max_w {
        let mut item_skip_to = 0;

        for item_no in params.min_d..params.max_d {
            if item_skip_to > item_no {
                continue;
            }

            let result = scan_from_index(grid, line_no, item_no, &params);
            item_skip_to = result.1.clone();
        }
    }

    result
}

fn scan_from_index(
    grid: &Vec<Vec<char>>,
    line_no: usize,
    item_no: usize,
    params: &ScanParameters,
) -> (i64, usize) {
    if grid[line_no][item_no] == '.' || SYMBOLS.contains(&grid[line_no][item_no]) {
        return (0, item_no);
    }

    let mut is_part = false;
    let mut item_full = String::new();
    let mut item_offset = 0;

    loop {
        if grid[line_no][item_no + item_offset] == '.' || SYMBOLS.contains(&grid[line_no][item_no])
        {
            break;
        }

        is_part = is_part || check_for_part(grid, line_no, item_no, params);

        item_full.push(grid[line_no][item_no + item_offset]);

        item_offset = item_offset + 1;
    }

    println!("{} is_part {}", item_full, is_part);

    (0, item_no + item_offset)
}

fn check_for_part(
    grid: &Vec<Vec<char>>,
    line_no: usize,
    item_no: usize,
    params: &ScanParameters,
) -> bool {
    let mut is_part = false;

    //...
    //.0.
    //..X
    if !is_part && line_no < params.max_d && item_no < params.max_w {
        is_part = is_part || SYMBOLS.contains(&grid[line_no + 1][item_no + 1]);
    }

    //...
    //.0.
    //.X.
    if !is_part && line_no < params.max_d {
        is_part = is_part || SYMBOLS.contains(&grid[line_no + 1][item_no + 0]);
    }

    //...
    //.0.
    //X..
    if !is_part && line_no < params.max_d && item_no > params.min_w {
        is_part = is_part || SYMBOLS.contains(&grid[line_no + 1][item_no - 1]);
    }

    //...
    //X0.
    //...
    if !is_part && item_no > params.min_w {
        is_part = is_part || SYMBOLS.contains(&grid[line_no + 0][item_no - 1]);
    }

    //X..
    //.0.
    //...
    if !is_part && line_no > params.min_d && item_no > params.min_w {
        is_part = is_part || SYMBOLS.contains(&grid[line_no - 1][item_no - 1]);
    }

    //.X.
    //.0.
    //...
    if !is_part && line_no > params.min_d {
        is_part = is_part || SYMBOLS.contains(&grid[line_no - 1][item_no + 0]);
    }

    //..X
    //.0.
    //...
    if !is_part && line_no > params.min_d && item_no < params.max_w {
        is_part = is_part || SYMBOLS.contains(&grid[line_no - 1][item_no + 1]);
    }

    //...
    //.0X
    //...
    if !is_part && item_no < params.max_w {
        is_part = is_part || SYMBOLS.contains(&grid[0][item_no + 1]);
    }

    is_part
}
