use std::iter::zip;
fn main() {
    let input = include_str!("input1.txt");
    let out = part2(input);
    println!("output: {}", out);
}

fn part1(input: &str) -> String{
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    let mut total = 0;
    for line in input.lines(){
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.get(0){
            Some(&x) => left.push(x.parse().expect("bruh")),
            None => ()
        }
        match parts.get(1){
            Some(&x) => right.push(x.parse().expect("bruh")),
            None => ()
        }
    }
    left.sort();
    right.sort();


    for (a, b) in zip(left, right){
        if a > b{
            println!("{}, {}, {}", a, b, a - b);
            total += a - b;
        }
        if a < b{
            println!("{}, {}, {}", a, b, b - a);
            total += b - a;
        }
    }


    total.to_string()
}




fn part2(input: &str) -> String{
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    let mut total = 0;
    for line in input.lines(){
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.get(0){
            Some(&x) => left.push(x.parse().expect("bruh")),
            None => ()
        }
        match parts.get(1){
            Some(&x) => right.push(x.parse().expect("bruh")),
            None => ()
        }
    }

    for a in left{
        for b in &right{
            if a == *b{
                total += a;
            }

        }
    }




    total.to_string()
}
