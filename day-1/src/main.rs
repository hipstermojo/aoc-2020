use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
fn main() {
    let mut file = File::open("day_1.txt").expect("Unable to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file contents");
    let expenses = contents
        .lines()
        .map(|line| line.parse::<i32>().expect("Unable to parse value to i32"))
        .collect();
    let expense = find_expenses(&expenses, 2020);
    if let Some(val) = expense {
        println!("{}", val);
    } else {
        println!("No 2 expenses found summing to 2020")
    }

    if let Some(val) = find_expenses_3(&expenses, 2020) {
        println!("{}", val)
    } else {
        println!("No 3 expenses found summing to 2020")
    }
}

fn find_expenses(expenses: &Vec<i32>, limit: i32) -> Option<i32> {
    let mut part_exps: HashSet<i32> = HashSet::new();
    let mut res: Option<i32> = None;
    for expense in expenses {
        let part_exp = limit - expense;
        if part_exps.contains(&part_exp) {
            res = Some(part_exp * expense);
            break;
        } else {
            part_exps.insert(*expense);
        }
    }
    res
}

fn find_expenses_3(expenses: &Vec<i32>, limit: i32) -> Option<i32> {
    let expenses_set: HashSet<&i32> = HashSet::from_iter(expenses.iter());
    let slash_expenses: Vec<i32> = expenses.iter().map(|exp| limit - exp).collect();
    let mut res: Option<i32> = None;
    for expense in expenses {
        for slash_expense in &slash_expenses {
            let diff = slash_expense - expense;
            if expenses_set.contains(&diff) {
                res = Some(diff * expense * (limit - slash_expense));
                return res;
            }
        }
    }
    res
}
