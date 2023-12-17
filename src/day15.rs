pub(crate) fn part_a() {
    let mut init_sequence = include_str!("../input/day15.txt").split(",");
    //let mut init_sequence = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".split(",");

    let mut result = 0;
    for step in init_sequence {
        let current_value = hash(step);
        println!("{step}: {current_value}");
        result += current_value;
    }
    println!("---\n{result}");
}

pub(crate) fn part_b() {
    let mut init_sequence = include_str!("../input/day15.txt").split(",");
    let mut init_sequence = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".split(",");

    let mut result_a = 0;
    for step in init_sequence {
        let current_value = hash(step);
        println!("{step}: {current_value}");
        result_a += current_value;
    }
    println!("---\n{result_a}");
    println!("---\n{}", hash("rn"));
}

fn hash(label: &str) -> u32 {
    let mut current_value = 0;
    for c in label.chars() {
        current_value += c as u32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}