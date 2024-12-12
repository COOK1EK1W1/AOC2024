use std::{collections::HashMap, iter::zip};

use memoise::memoise;
fn main() {
    let input = include_str!("input.txt");
    let input2 = "125 17";
    //println!("{}", part1(input));
    //println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> String{
    let mut initial: Vec<u64> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    println!("{:?}", initial);
    let times = 25;

    for i in 0..times{
        blink(&mut initial);
    }

    return initial.len().to_string();
}

fn part2(input: &str) -> String{
    let mut initial: Vec<u64> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut total = 0;
    let mut pre = HashMap::new();
    let amount = 75;
    for stone in initial.iter(){
        how_many_stones(*stone, amount, &mut pre);
        total += pre.get(stone).unwrap()[amount as usize];

    }
    println!("{:?}", pre);

    return total.to_string();
}

fn blink(stones: &mut Vec<u64>){
    let mut i = 0;
    while i < stones.len(){
        if stones[i] == 0{
            stones[i] = 1;
        } else if stones[i].to_string().len() % 2 == 0{
            let num_str = stones[i].to_string();
            let len = num_str.len();
            let left_str = &num_str[..len / 2];
            let right_str = &num_str[len / 2..];
            stones[i] = left_str.parse::<u64>().unwrap_or(0);
            stones.insert(i, right_str.parse::<u64>().unwrap_or(0));
            i += 1;
        }else{
            stones[i] *= 2024;
        }
        i += 1;
    }

}

fn how_many_stones(stone_value: u64, steps: u64, pre: &mut HashMap<u64, Vec<u64>>){
    if pre.contains_key(&stone_value) {
        if pre.get(&stone_value).unwrap().len() as u64 > steps{
            //println!("{}, {:?}", stone_value, pre.get(&stone_value).unwrap());
            return
        }
    }

    if steps == 0{
        pre.insert(stone_value, vec![1]);
        return
    }
    if stone_value == 0{
        how_many_stones(1, steps - 1, pre);
        let mut a = pre.get(&1).unwrap().clone();
        a.truncate(steps as usize);
        a.insert(0, 1);
        pre.insert(stone_value, a);
    } else if stone_value.to_string().len() % 2 == 0{
        let num_str = stone_value.to_string();
        let len = num_str.len();
        let left_str = &num_str[..len / 2];
        let right_str = &num_str[len / 2..];
        let left_val = left_str.parse::<u64>().unwrap_or(0);
        let right_val = right_str.parse::<u64>().unwrap_or(0);
        how_many_stones(left_val, steps - 1, pre);
        how_many_stones(right_val, steps-1, pre);

        let mut a = pre.get(&right_val).unwrap().clone();
        let mut b = pre.get(&left_val).unwrap().clone();
        a.truncate(steps as usize);
        b.truncate(steps as usize);
        let mut zipped: Vec<u64> = zip(a, b).map(|(a, b)| a + b).collect();
        zipped.insert(0, 1);
        pre.insert(stone_value, zipped);

    }else{
        let new_val = stone_value * 2024;
        how_many_stones(new_val, steps-1, pre);
        let mut a = pre.get(&new_val).unwrap().clone();
        a.truncate(steps as usize);
        a.insert(0, 1);
        pre.insert(stone_value, a);
    }
}
