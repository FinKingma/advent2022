use std::fs;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    name: i64,
    inspections: i64,
    operator: String,
    items: VecDeque<i64>,
    check: i64,
    true_monkey: i64,
    false_monkey: i64
}

impl Monkey {
    fn pull(&mut self) -> (usize, i64) {
        self.inspections += 1;

        let operator_parts: Vec<&str> = self.operator.split(" ").collect();
        let operator = operator_parts[1];
        let operator_change = operator_parts[2];

        let mut item: i64 = self.items.pop_front().unwrap();
        let change_value: i64;
        if operator_change == "old" {
            change_value = item;
        } else {
            change_value = operator_change.parse().unwrap();
        }
        if operator == "*" {
            item *= change_value;
        } else if operator == "+" {
            item += change_value;
        }
        if item % self.check == 0 {
            (self.true_monkey as usize, item)
        } else {
            (self.false_monkey as usize, item)
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let binding = contents.replace("\r", "");
    let monkeys_input = binding.split("\n\n").collect::<Vec<&str>>();
    let mut monkeys = Vec::new();

    for monkey_input in monkeys_input {
        let re = Regex::new(r"Monkey (\d+):\n  Starting items: ([\d, ]+)\n  Operation: new = (.+)\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey (\d+)").unwrap();
        let caps = re.captures(&monkey_input).unwrap();
        let name: i64 = caps[1].parse().unwrap();
        let starting_items: Vec<i64> = caps[2].to_string().split(",").map(|e| e.replace(" ", "").parse().unwrap()).collect::<Vec<i64>>();
        let operator: String = caps[3].to_string();
        let check: i64 = caps[4].parse().unwrap();
        let true_monkey: i64 = caps[5].parse().unwrap();
        let false_monkey: i64 = caps[6].parse().unwrap();
        let mut items: VecDeque<i64> = VecDeque::new();
        for item in starting_items {
            items.push_back(item);
        }
        monkeys.push(Monkey {
            name: name,
            inspections: 0,
            operator: operator,
            items: items,
            check: check,
            true_monkey: true_monkey,
            false_monkey: false_monkey
        });
    }

    for i in 0..4 {
        if monkeys[i].name as usize != i {
            panic!("error! monkey {} is not in correct position", i);
        }
    }

    let mut distress = 1;
    for monkey in &monkeys {
        distress *= monkey.check;
    }
    println!("distress {}", distress);

    // start rounds
    let rounds: usize = 10000;
    for r in 0..rounds {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let next = monkeys[i].pull();
                // println!("test {:?} {}",next, next.1 % distress);
                monkeys[next.0].items.push_back(next.1 % distress);
            }
        }
    }

    monkeys.sort_by(|a,b| b.inspections.partial_cmp(&a.inspections).unwrap());

    // for monkey in monkeys {
    //     println!("monkey {:?}", monkey);
    // }
    let score = monkeys[0].inspections * monkeys[1].inspections;
    println!("score {}", score);

}