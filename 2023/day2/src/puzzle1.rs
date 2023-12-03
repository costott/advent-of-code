use std::fs;

struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}
impl Bag {
    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

pub fn possible() {
    let input = fs::read_to_string("day2input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut sum = 0;

    for line in lines {
        let info: Vec<&str> = line.split(": ").collect();

        let id: u32 = info[0].split(" ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        let mut all_valid = true;
        let sets: Vec<&str> = info[1].split("; ").collect();
        for set in sets {
            let mut bag = Bag {
                red: 0,
                green: 0,
                blue: 0,
            };

            let cubes: Vec<&str> = set.split(", ").collect();
            for cube in cubes {
                let colour: Vec<&str> = cube.split(" ").collect();
                let num = colour[0].parse::<usize>().unwrap();
                match colour[1].trim_end() {
                    "red" => bag.red += num,
                    "green" => bag.green += num,
                    "blue" => bag.blue += num,
                    _ => panic!(),
                };
            }

            if !bag.is_possible() {
                all_valid = false;
            }
        }

        if all_valid {
            sum += id;
        }
    }

    println!("{}", sum);
}
