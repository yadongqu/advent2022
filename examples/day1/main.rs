fn part1(content: &str) -> u32 {
    content.split("\n\n").fold(std::u32::MIN, |current_max, calaries| {
        let sum_calaries = |s: &str| {
            s.split("\n").fold(0u32, |mut sum, number| {
                sum += number.parse::<u32>().unwrap();
                sum
            })
        };
    sum_calaries(calaries).max(current_max)
    })
} 


fn part2(content: &str) -> u32 {
    let mut calories: Vec<u32> = content.split("\n\n").map(|numbers| {
        numbers.split("\n").fold(0, |mut sum, number| {
            sum += number.parse::<u32>().unwrap();
            sum
        })
    }).collect();
    calories.sort_by(|a, b| a.cmp(b).reverse());
    calories.truncate(3);
    calories.iter().sum()
}


fn main() {
    let content = include_str!("input.txt");
    let answer1 = part1(content);
    println!("{answer1}");
    let answer2 = part2(content);
    println!("{answer2}");
    
}