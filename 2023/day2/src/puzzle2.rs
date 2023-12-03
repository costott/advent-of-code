use std::fs;

struct Bag {
    min_red: usize,
    red: usize,
    min_green: usize,
    green: usize,
    min_blue: usize,
    blue: usize,
}
impl Bag {
    fn new() -> Bag {
        Bag {
            min_red: 0,
            red: 0,
            min_green: 0,
            green: 0,
            min_blue: 0,
            blue: 0,
        }
    }

    fn clear(&mut self) {
        self.red = 0;
        self.green = 0;
        self.blue = 0;
    }

    fn check_mins(&mut self) {
        self.min_red = self.min_red.max(self.red);
        self.min_green = self.min_green.max(self.green);
        self.min_blue = self.min_blue.max(self.blue);
    }

    fn power(&self) -> usize {
        self.min_red * self.min_green * self.min_blue
    }
}

pub fn sum_powers() {
    let input = fs::read_to_string("day2input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut sum = 0;

    for line in lines {
        let info: Vec<&str> = line.split(": ").collect();
        let mut bag = Bag::new();

        let sets: Vec<&str> = info[1].split("; ").collect();
        for set in sets {
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

            bag.check_mins();
            bag.clear();
        }

        sum += bag.power();
    }

    println!("{}", sum);
}
