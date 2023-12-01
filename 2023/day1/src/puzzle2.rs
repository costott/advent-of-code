use std::fs;

fn get_value(line: String) -> usize {
    let mut first: Option<usize> = Option::None;
    let mut last: Option<usize> = Option::None;

    let end = line.chars().count();

    for i in 0..end {
        if first.is_none() {
            match line[i..=i].parse::<usize>() {
                Ok(n) => first = Option::Some(n),
                Err(_) => {}
            }
        }
        if last.is_none() {
            match line[end - 1 - i..=end - 1 - i].parse::<usize>() {
                Ok(n) => last = Option::Some(n),
                Err(_) => {}
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
        let mut line = line.replace("one", "o1e");
        line = line.replace("two", "t2o");
        line = line.replace("three", "t3e");
        line = line.replace("four", "f4r");
        line = line.replace("five", "f5e");
        line = line.replace("six", "s6x");
        line = line.replace("seven", "s7n");
        line = line.replace("eight", "e8t");
        line = line.replace("nine", "n9e");

        sum += get_value(line);
    }

    println!("{}", sum);
}
