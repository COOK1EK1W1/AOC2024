use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String{
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|do\(\)|don\'t\(\)").unwrap();
    let re2 = Regex::new(r"\b\d{1,3}\b").unwrap();
    let mut total = 0;

    let matches: Vec<&str> = re
        .find_iter(input) // Iterate over all matches
        .map(|mat| mat.as_str())
        .collect(); // Extract the matched text as a &str
    println!("{}", matches.len());

    let mut active = true;

    for op in matches.iter(){
        match op.chars().nth(2){
            Some('l') =>{

                if active{
                    let matches: Vec<&str> = re2
                        .find_iter(op) // Iterate over all matches
                        .map(|mat| mat.as_str())
                        .collect(); // Extract the matched text as a &str
                    let a: i32 = matches.get(0).expect("a").parse().expect("d");
                    let b: i32 = matches.get(1).expect("b").parse().expect("d");
                    //println!("{} * {} - {}", a, b, total);
                    total += a * b;

                }

            },
            Some('(')=>{active=true}
            Some('n')=>{active = false},
            None => (),
            _ => ()
        }

    }

    total.to_string()
}

fn part2(input: &str) -> String{
    for line in input.lines(){

    }
    "".to_string()
}
