fn overlap_part1(first: Vec<u32>, second: Vec<u32>) -> bool {
    if (first[0] <= second[0] && first[1] >= second[1]) || (second[0] <= first[0] && second[1] >= first[1]) {
        return true;
    }
    return false;
}

fn overlap_part2(first: Vec<u32>, second: Vec<u32>) -> bool {
    if first[1] < second[0] || first[0] > second[1] {
        return false;
    }
    return true;
}

type OverlapFunc = fn(Vec<u32>, Vec<u32>) -> bool;

fn solve(content: &str, overlap_func: OverlapFunc) -> u32 {
    content.split("\n").fold(0, |mut count, line| {
        let pair: Vec<&str> = line.split(",").collect();
        let first: Vec<u32> = pair[0].split("-").map(|s| {
            s.parse::<u32>().unwrap()
        }).collect();
        let second : Vec<u32>=  pair[1].split("-").map(|s| {
            s.parse::<u32>().unwrap()
        }).collect();
        if overlap_func(first, second) {
            count += 1;
        }
        count
    })
}


fn main() {
    let content = include_str!("input.txt");
    println!("{}", solve(content, overlap_part1));
    println!("{}", solve(content, overlap_part2));
}