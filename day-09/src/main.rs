fn main() {
    let input = include_str!("input.txt");
    let input2 = "2333133121414131402";
    println!("{}", part2(input));
    //println!("{}", part2(input2));
}

fn part1(input: &str) -> String{
    let mut disk: Vec<i64> = Vec::new();
    for (i, char) in input.chars().filter(|s|s.is_numeric()).enumerate(){
        // println!("{}", char);
        let len: i64 = char.to_digit(10).unwrap() as i64;
        for x in 0..len{
            if i % 2 == 0 {
                // file
                disk.push(i as i64 /2 + 1);
            }else{
                disk.push(0 as i64);
            }
        }

    }
    println!("{:?}", disk);

    let mut i = 0;
    while i < disk.len(){
        if *disk.get(i).expect("b") == 0{
            let mut val = disk.pop().expect(" ");
            while val == 0{
                val = disk.pop().expect("");
            }
            disk[i] = val;
        }
        i += 1;
    }

    let mut total: i64 = 0;
    println!("{:?}", disk);
    for (i, n) in disk.iter().enumerate(){
        total += i as i64 * (n-1);
    }
    return total.to_string();
}

fn part2(input: &str) -> String{
    let mut disk: Vec<i64> = Vec::new();
    for (i, char) in input.chars().filter(|s|s.is_numeric()).enumerate(){
        // println!("{}", char);
        let len: i64 = char.to_digit(10).unwrap() as i64;
        for x in 0..len{
            if i % 2 == 0 {
                // file
                disk.push(i as i64 /2 + 1);
            }else{
                disk.push(0 as i64);
            }
        }

    }
    println!("{:?}", disk);

    let mut pointer_a = disk.len() - 1;
    let mut pointer_b = disk.len() - 1;
    loop{

        let mut pointer_c = 0;
        let mut pointer_d = 0;

        // find next file
        while disk.get(pointer_a).expect("") == disk.get(pointer_b).expect(""){
            if pointer_a > 0{
                pointer_a -= 1;
            }else{
                break;
            }
        }
        if pointer_a < 2{
            break;
        }
        // fine slot
        while pointer_c < disk.len() && pointer_c - pointer_d < pointer_b - pointer_a{
            pointer_c += 1;
            if pointer_c < disk.len() && *disk.get(pointer_c).expect("") != 0{
                pointer_d = pointer_c;
            }
        }
        if pointer_c == disk.len(){
            pointer_b = pointer_a;
            continue;
        }

        // move
        if pointer_a < pointer_d{
            pointer_b = pointer_a;
            continue;
        }
        while pointer_a != pointer_b{
            disk.swap(pointer_b, pointer_d + 1);
            pointer_b -= 1;
            pointer_d += 1;

        }
        if pointer_a < 2{
            break;
        }
    }

    let mut total: i64 = 0;
    println!("{:?}", disk);
    for (i, n) in disk.iter().enumerate(){
        if *n >0{
        total += i as i64 * (n-1);
        }
    }
    return total.to_string();
}
