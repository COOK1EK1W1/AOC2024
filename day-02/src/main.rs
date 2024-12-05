use std::str;

fn main() {
    println!("Hello, world!");

    let input = include_str!("input.txt");
    println!("{}", part2("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"));
    println!("{}", part2(input));
}


fn part1(input: &str) -> String{
    let mut safe_total = 0;
    for line in input.lines(){
        if check_safe(line){
            safe_total += 1;
        }
    }
    safe_total.to_string()

}

fn check_safe(input: &str) -> bool{
    let parts: Vec<&str> = input.split_whitespace().collect();
    let one: i32 = parts.get(0).expect("bruh").parse().expect("s");
    let two: i32 = parts.get(1).expect("bruh").parse().expect("s");
    let mut safe = true;
    if one < two{
        // ascending
        for (&a, &b) in std::iter::zip(parts.iter().take(parts.len() - 1), parts.iter().skip(1)){
            let a_num: i32 = a.parse().expect("s");
            let b_num: i32 = b.parse().expect("s");
            if !(a_num < b_num && a_num + 4 > b_num){
                safe = false;
            }
        }
    }else{
        // decending
        for (&a, &b) in std::iter::zip(parts.iter().take(parts.len() - 1), parts.iter().skip(1)){
            let a_num: i32 = a.parse().expect("s");
            let b_num: i32 = b.parse().expect("s");
            if !(a_num > b_num && a_num - 4 < b_num){
                safe = false;
            }
        }

    }
    return safe
}


fn part2(input: &str) -> String{
    let mut safe_total = 0;
    for line in input.lines(){
        if check_safe(line){
            safe_total += 1;
        }else{
            let parts: Vec<&str> = line.split_whitespace().collect();
            let mut safe = false;
            for i in 0..(parts.len()){
                let mut new_vec = parts.clone();
                new_vec.remove(i);
                if check_safe(&new_vec.join(" ")){
                    safe = true;

                }



            }
            if safe{
                safe_total += 1;

            }

        }

    }
    safe_total.to_string()

}

