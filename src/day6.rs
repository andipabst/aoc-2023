use roots::find_roots_quadratic;


/* Input:
Time:        47     98     66     98
Distance:   400   1213   1011   1540
*/

pub(crate) fn part_a() {
    let input = vec![
        /*(7, 9),
        (15, 40),
        (30, 200),*/
        (47, 400),
        (98, 1213),
        (66, 1011),
        (98, 1540),
    ];

    let mut sum_a = 1.0;
    for (time, distance) in input {
        let result = find_roots_quadratic(-1.0, time as f32, -distance as f32).as_ref().to_vec();
        let diff = ((result[1] - 1.0).ceil() - (result[0] + 1.0).floor()) + 1.0;
        println!("{:?} {}", result, diff);
        sum_a *= diff;
    }

    println!("---\n{}", sum_a)
}

pub(crate) fn part_b() {
    let result = find_roots_quadratic(-1.0, 47986698f64, -400121310111540f64).as_ref().to_vec();
    let diff = ((result[1] - 1.0).ceil() - (result[0] + 1.0).floor()) + 1.0;
    println!("{:?} {}", result, diff);
}

