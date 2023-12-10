pub(crate) fn part_a() {
    let mut sensor_readings: Vec<Vec<i64>> = include_str!("../input/day9.txt")
        .lines()
        .map(|l| l.split_whitespace().map(|c| c.parse().unwrap()).collect())
        .collect();

    let mut next_value_sum = 0;
    let mut previous_value_sum = 0;
    for reading in sensor_readings {
        println!("{:?}", &reading);

        let mut last_values = Vec::new();
        let mut first_values = Vec::new();
        let mut diffs = reading.clone();

        while !diffs.iter().all(|x| x == &0) {
            last_values.push(diffs.last().unwrap().clone());
            first_values.push(diffs.first().unwrap().clone());

            let mut _diffs = Vec::new();
            for win in diffs.windows(2) {
                _diffs.push(win[1] - win[0]);
            }
            diffs = _diffs;

            // println!("{:?}", diffs);
        }

        last_values.reverse();
        println!("last values: {:?}", last_values);
        let mut next_value = 0;
        for value in last_values {
            next_value += value;
        }
        next_value_sum += next_value;
        println!("next value: {}", next_value);

        first_values.reverse();
        println!("first values: {:?}", first_values);
        let mut previous_value = 0;
        for value in first_values {
            previous_value = value - previous_value;
        }
        previous_value_sum += previous_value;
        println!("prev value: {}", previous_value);

        println!("-----");
    }
    println!("Part A: {}", next_value_sum);
    println!("Part B: {}", previous_value_sum);
}