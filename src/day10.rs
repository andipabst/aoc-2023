#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn left(&self) -> Option<Point> {
        if self.x == usize::MIN {
            None
        } else {
            Some(Point { x: self.x - 1, y: self.y })
        }
    }
    fn right(&self) -> Option<Point> {
        if self.x == usize::MAX {
            None
        } else {
            Some(Point { x: self.x + 1, y: self.y })
        }
    }
    fn top(&self) -> Option<Point> {
        if self.y == usize::MIN {
            None
        } else {
            Some(Point { x: self.x, y: self.y - 1 })
        }
    }
    fn bottom(&self) -> Option<Point> {
        if self.y == usize::MAX {
            None
        } else {
            Some(Point { x: self.x, y: self.y + 1 })
        }
    }

    fn next_point(&self, previous_point: &Point, shape: &char) -> Option<Point> {
        match shape {
            &'|' => {
                previous_point.not_matching(self.top(), self.bottom())
            }
            &'-' => {
                previous_point.not_matching(self.left(), self.right())
            }
            &'L' => {
                previous_point.not_matching(self.top(), self.right())
            }
            &'J' => {
                previous_point.not_matching(self.left(), self.top())
            }
            &'7' => {
                previous_point.not_matching(self.left(), self.bottom())
            }
            &'F' => {
                previous_point.not_matching(self.right(), self.bottom())
            }

            _ => { None }
        }
    }

    fn not_matching(&self, a: Option<Point>, b: Option<Point>) -> Option<Point> {
        if a.is_some_and(|p| p == *self) {
            b
        } else if b.is_some_and(|p| p == *self) {
            a
        } else {
            None
        }
    }
}

fn get_shape(entries: &Vec<Vec<char>>, point: &Point) -> char {
    entries[point.y][point.x]
}

pub(crate) fn part_a() {
    let lines = include_str!("../input/day10.txt").lines();
    //let lines = include_str!("../input/day10_test.txt").lines();

    let mut entries: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut starting_position = Point { x: 0, y: 0 };

    for (y, line) in entries.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == &'S' {
                starting_position = Point { x, y };
            }
        }
    }

    println!("starting position is {:?}", starting_position);

    let starts = vec![
        starting_position.top(),
        starting_position.right(),
        starting_position.bottom(),
        starting_position.left(),
    ];

    for p in starts {
        if let Some(p) = p {
            if get_shape(&entries, &p) == 'S' || get_shape(&entries, &p) == '.' { continue; }
            println!("start from {:?}: {}", p, get_shape(&entries, &p));

            let mut shape = ' ';
            let mut previous_position = starting_position;
            let mut position = p;
            let mut steps = 0;
            while shape != 'S' {
                shape = get_shape(&entries, &position);
                if shape == '.' {
                    break;
                }
                //println!("{:?}: {}", position, shape);

                let next = position.next_point(&previous_position, &shape);
                if next.is_some() {
                    previous_position = position;
                    position = next.unwrap();
                    steps += 1;
                } else {
                    break;
                }
            }
            println!("completed loop after {} steps (max distance {})", steps, ((steps as f32) / 2f32).ceil())
        }
    }
}

pub(crate) fn part_b() {
    let lines = include_str!("../input/day10.txt").lines();

    let entries: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    // from part A
    let mut starting_position = Point { x: 72, y: 30 };
    let p = starting_position.top().unwrap();
    println!("starting position is {:?}, continuing at {:?}", starting_position, p);

    let mut shape = ' ';
    let mut previous_position = starting_position;
    let mut position = p;
    let mut points = vec![];
    while shape != 'S' {
        shape = get_shape(&entries, &position);
        if shape == '.' {
            break;
        }

        let next = position.next_point(&previous_position, &shape);
        if next.is_some() {
            previous_position = position;
            position = next.unwrap();
            points.push(position);
        } else {
            break;
        }
    }
    println!("{:?} points in loop", points.len());

    let mut area = 0;
    for y in 0..entries.len() {
        let mut inside = false;
        let mut prev_corner = ' ';
        for x in 0..entries[y].len() {
            let p = Point { x, y };
            let shape = get_shape(&entries, &p);
            if points.contains(&p) {
                if shape == '|' {
                    inside = !inside;
                } else if vec!['J', '7', 'L', 'F'].contains(&shape) {
                    if (shape == 'F' && prev_corner == 'J') || (shape == 'L' && prev_corner == '7') {
                        inside = !inside;
                    }
                    prev_corner = shape;
                }
            } else if inside {
                area += 1;
            }
        }
    }

    println!("{}", area);
}