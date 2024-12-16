use std::cmp::min;

use regex::Regex;

fn main() {
    println!("Hello, world!");
    let input = include_str!("input.txt");
    let input2 = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    println!("{}", part1(input));
}

struct Machine{
    a_move: (i64, i64),
    b_move: (i64, i64),
    prize: (i64, i64)

}

fn part1(input: &str) -> String{
    let b: Vec<&str> = input.split("\n\n").collect();
    let machines = parse_machines(b);
    let mut total_tokens = 0;
    for (i, machine) in machines.iter().enumerate(){

        let b2_num = machine.prize.1 * machine.b_move.0 - machine.b_move.1 * machine.prize.0;
        let slope_diff_num = machine.a_move.1 * machine.b_move.0 - machine.b_move.1 * machine.a_move.0;
        let intersect_x = (b2_num * machine.a_move.0) / slope_diff_num;

        let a_times = intersect_x / machine.a_move.0;
        let b_times = (machine.prize.0 - intersect_x) / machine.b_move.0;

        //println!("{} {}", a_times, b_times);

        if a_times * machine.a_move.0 + b_times * machine.b_move.0 == machine.prize.0 || a_times * machine.a_move.0 + b_times * machine.b_move.0 == machine.prize.0{
            if a_times % 100 == 0 && b_times % 100 == 0{
                total_tokens += (a_times/100) * 3 + (b_times/100);
            }
        }

    }
    return total_tokens.to_string();

}

fn parse_machines(l: Vec<&str>) -> Vec<Machine>{
    let mut ret = Vec::new();
    let re = Regex::new(r"\d+").unwrap();
    for a in l.iter(){
        let b: Vec<i64> = re.find_iter(a)
            .filter_map(|m| m.as_str().parse::<i64>().ok())
            .collect();

        //ret.push(Machine { a_move: (b[0], b[1]), b_move: (b[2], b[3]), prize: (b[4], b[5]) })
        ret.push(Machine { a_move: (b[0], b[1]), b_move: (b[2], b[3]), prize: ((10000000000000 + b[4]) * 100, (10000000000000 + b[5]) * 100) })
    }

    return ret
}
