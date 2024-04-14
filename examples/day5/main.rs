

fn main() {
    let content: Vec<&str> = include_str!("input.txt").split("\n\n").collect();
    let configuration: Vec<&str> = content[0].lines().collect();
    let length = configuration[configuration.len() - 1].trim().split(" ").filter(|group| !group.trim().is_empty()).collect::<Vec<&str>>().len();
    let mut groups: Vec<Vec<char>> = vec![vec![]; length];
    for line in configuration[0..configuration.len() - 1].iter().rev() {
        for(index, ch) in line.chars().enumerate().filter(|x| x.1.is_ascii_uppercase()) {
            groups[(index as f32 / 4.).ceil() as usize - 1].push(ch)
        }
    }

    let instructions : Vec<(usize, usize, usize)> = content[1].lines().map(|ins| {
        let instructions: Vec<&str> = ins.trim().split(" ").collect();
        let number: usize = instructions[1].parse().unwrap();
        let from_group: usize = instructions[3].parse().unwrap();
        let to_group: usize = instructions[5].parse().unwrap();
        (number, from_group, to_group)
    }).collect();

    for (number, from_group, to_group) in instructions.iter() {
        let len = groups[from_group - 1].len();
        let removed: Vec<char> = groups[from_group-1].drain(len - number..).collect();
        groups[to_group-1].extend(removed.iter().rev());
    }

    let result: String = groups.iter().fold(vec![], |mut s, v| {
        s.push(v.last().unwrap());
        s
    }).into_iter().collect();

    println!("{result}" );
}