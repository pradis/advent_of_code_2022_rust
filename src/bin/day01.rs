use std::fs;
use std::io::{BufRead,BufReader,Error};

struct Elf {
    calories: u32,
}

impl Elf {
    fn init() -> Elf{
        Elf{calories: 0}
    }


    fn gets_food(&self, calories: u32) -> Self {
        Elf {
            calories: self.calories + calories
        }
    }

}

fn ex1(path: &str) -> u32{
    let file = fs::File::open(path).expect("Should have been able to read the file");
    let reader = BufReader::new(file);
    let (_,richest_elf) = reader.lines()
        .fold((Elf::init(),Elf::init()),|(elf,richest_elf),line| {
            let maybe_line= line.expect("line with some issues");
            if maybe_line.is_empty() {
                let new_richest_elf = if richest_elf.calories >= elf.calories {
                    richest_elf
                } else {
                    elf
                };
                (Elf::init(),new_richest_elf)

            } else {
                (elf.gets_food(maybe_line.parse().unwrap()),richest_elf)

            }
        });
    richest_elf.calories
}


fn ex2(path: &str) -> u32{
    let file = fs::File::open(path).expect("Should have been able to read the file");
    let reader = BufReader::new(file);
    let (_,first_elf,second_elf,third_elf) = reader.lines()
        .fold((Elf::init(),Elf::init(),Elf::init(),Elf::init()),|(actual_elf,first_elf,second_elf,third_elf),line| {
            let maybe_line= line.expect("line with some issues");
            if maybe_line.is_empty() {
                if actual_elf.calories >= first_elf.calories {
                    (Elf::init(), actual_elf, first_elf, second_elf)
                } else if actual_elf.calories >= second_elf.calories{
                    (Elf::init(),first_elf,actual_elf,second_elf)
                } else if actual_elf.calories >= third_elf.calories {
                    (Elf::init(), first_elf, second_elf, actual_elf)
                } else {
                    (Elf::init(), first_elf, second_elf, third_elf)
                }
            } else {
                (actual_elf.gets_food(maybe_line.parse().unwrap()),first_elf,second_elf,third_elf)

            }
        });
    println!("{}",first_elf.calories);
    println!("{}",second_elf.calories);
    println!("{}",third_elf.calories);
    first_elf.calories + second_elf.calories + third_elf.calories
}

fn main() -> Result<(), Error>{

    println!("example es1: {}", ex1("resources/examples/day_01.txt"));
    println!("result es1: {}", ex1("resources/inputs/day_01.txt"));
    println!("example es1: {}", ex2("resources/examples/day_01.txt"));
    println!("result es1: {}", ex2("resources/inputs/day_01.txt"));
    Ok(())
}
