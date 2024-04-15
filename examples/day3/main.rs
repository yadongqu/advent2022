use std::{collections::HashSet, vec};

fn calculate_score(ch: char) -> u32{
    let b = ch as u8;
    if b.is_ascii_lowercase() {
        return (b - 'a' as u8 + 1) as u32; 
    }
    else if b.is_ascii_uppercase() {
        return (b - 'A' as u8 + 27) as u32;
    }

    return 0;
}

fn part1(content: &str) -> u32 {
    content.lines().fold(0, |mut sum, line| {
        let mid = line.len() / 2;
        let first: HashSet<char> = line[0..mid].chars().collect();
        let second = line[mid..].to_string();

        sum += first.iter().fold(0, |mut score, ch|  {
            if second.contains(*ch) {
                score += calculate_score(*ch);
            }
            score
        });
        sum
    })
}

fn part2(content: &str) -> u32 {

    content.lines().collect::<Vec<&str>>().chunks(3).fold(0, |mut sum, group| {
        let first: HashSet<char> = group[0].chars().collect();
        let second = group[1];
        let third = group[2];
        sum += first.iter().fold(0, |mut score, ch| {
            if second.contains(*ch) && third.contains(*ch) {
                score += calculate_score(*ch);
            }
            score
        });
        sum
    })
}


fn main() {
    let content = include_str!("input.txt");
    let answer1 = part1(content);
    println!("{answer1}");
    let answer2 = part2(content);
    println!("{answer2}");
}