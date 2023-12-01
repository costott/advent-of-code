use std::fs;

fn get_value(line: &str) -> usize {
    let mut first: Option<usize> = Option::None;
    let mut last: Option<usize> = Option::None;

    let end = line.chars().count();

    for i in 0..end {
        if first.is_none() {
            if let Ok(n) = line[i..=i].parse::<usize>() {
                first = Some(n);
            }
        }
        if last.is_none() {
            if let Ok(n) = line[end - 1 - i..=end - 1 - i].parse::<usize>() {
                last = Option::Some(n);
            }
        }
    }

    return first.unwrap() * 10 + last.unwrap();
}

pub fn calibrate() {
    let input = fs::read_to_string("day1input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut sum = 0;

    for line in lines {
        sum += get_value(line);
    }

    println!("{}", sum);
}
