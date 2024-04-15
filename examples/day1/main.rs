use advent2022::TWO_LINE_ENDING;

fn part1(content: &str) -> u32 {
    content.split(TWO_LINE_ENDING).fold(std::u32::MIN, |current_max, calaries| {
        let sum_calaries = |s: &str| {
            s.lines().fold(0u32, |mut sum, number| {
                let number = number.trim();
                sum += number.parse::<u32>().unwrap();
                sum
            })
        };
    sum_calaries(calaries).max(current_max)
    })
} 


fn part2(content: &str) -> u32 {
    let mut calories: Vec<u32> = content.split("\r\n\r\n").map(|numbers| {
        numbers.lines().fold(0, |mut sum, number| {
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