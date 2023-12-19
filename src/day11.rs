pub(crate) fn part_a() {
    let lines = include_str!("../input/day11.txt").lines();

    let mut entries: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    println!("{} x {}", entries.len(), entries[0].len());

    let mut empty_rows = vec![];
    for (row_num, row) in entries.clone().iter().enumerate() {
        if row.iter().all(|&e| e == '.') {
            empty_rows.push(row_num);
        }
    }
    empty_rows.reverse();
    for row in empty_rows {
        entries.insert(row, entries[row].clone())
    }

    println!("{} x {}", entries.len(), entries[0].len());

    entries = transpose(entries);

    let mut empty_cols = vec![];
    for (col_num, col) in entries.clone().iter().enumerate() {
        if col.iter().all(|&e| e == '.') {
            empty_cols.push(col_num)
        }
    }
    empty_cols.reverse();
    for col in empty_cols {
        entries.insert(col, entries[col].clone())
    }

    entries = transpose(entries);

    println!("{} x {}", entries.len(), entries[0].len());

    let mut galaxies = vec![];
    for (row_num, row) in entries.iter().enumerate() {
        for (col_num, &char) in row.iter().enumerate() {
            if char == '#' {
                galaxies.push((row_num, col_num));
            }
        }
    }

    let mut distance_sum = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let distance = galaxies[j].0.abs_diff(galaxies[i].0) + galaxies[j].1.abs_diff(galaxies[i].1);
            distance_sum += distance;
        }
    }
    println!("{}", distance_sum);
}

pub(crate) fn part_b() {
    let lines = include_str!("../input/day11.txt").lines();

    let mut entries: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    println!("{} x {}", entries.len(), entries[0].len());

    let mut empty_rows = vec![];
    for (row_num, row) in entries.clone().iter().enumerate() {
        if row.iter().all(|&e| e == '.') {
            empty_rows.push(row_num);
        }
    }
    println!("empty rows: {:?}", empty_rows);
    entries = transpose(entries);

    let mut empty_cols = vec![];
    for (col_num, col) in entries.clone().iter().enumerate() {
        if col.iter().all(|&e| e == '.') {
            empty_cols.push(col_num)
        }
    }
    println!("empty cols: {:?}", empty_cols);
    entries = transpose(entries);

    let mut galaxies = vec![];
    for (row_num, row) in entries.iter().enumerate() {
        for (col_num, &char) in row.iter().enumerate() {
            if char == '#' {
                let row_expansions = empty_rows.iter().filter(|&&r| r <= row_num).count();
                let col_expansions = empty_cols.iter().filter(|&&c| c <= col_num).count();
                galaxies.push((row_num + (row_expansions * 999_999), col_num + (col_expansions * 999_999)));
            }
        }
    }

    println!("{:?}", galaxies.clone().iter().take(5).collect::<Vec<&(usize, usize)>>());

    let mut distance_sum = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let distance = galaxies[j].0.abs_diff(galaxies[i].0) + galaxies[j].1.abs_diff(galaxies[i].1);
            distance_sum += distance;
        }
    }
    println!("{}", distance_sum);
}

fn transpose(entries: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = entries.len();
    let cols = entries[0].len();

    let mut transposed = vec![vec![' '; rows]; cols];

    for (row_num, row) in entries.iter().enumerate() {
        for (col_num, &value) in row.iter().enumerate() {
            transposed[col_num][row_num] = value;
        }
    }

    transposed
}
