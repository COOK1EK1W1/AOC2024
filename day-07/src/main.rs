
fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
    let input2 = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    println!("{}", part1(input2));
}
fn part1(input: &str) -> String{
    let mut total = 0;
    for line in input.lines(){
        let parts: Vec<&str> = line.split(": ").collect();
        //println!("{:?}", parts);
        let expected: i64 = parts[0].parse().expect("b");
        let mut q = Vec::new();
        for num in parts[1].split_whitespace(){
            q.push(num.parse().expect(""));
        }

        if is_eq(q.pop().expect(""), q, expected){
            total += expected;
        }
    }

    return total.to_string();

}

fn is_eq(a: i64, mut b: Vec<i64>, c: i64) -> bool{
    //println!("{}, {:?}, {}", a, b, c);
    if b.len() == 0{
        return a == c;
    }

    let next = b.pop().expect("b");
    // addition
    if is_eq(next, b.clone(), c - a){
        return true;
    }
    // multiplication
    if c % a == 0 && is_eq(next, b.clone(), c / a){
        return true;
    }
    // Concatenation

    let len = a.to_string().len();
    let e = c.to_string();
    if a.to_string().len() >= c.to_string().len(){
        return false
    }
    //println!("{}", &e[..e.len()-len]);
    if c.to_string().ends_with(&a.to_string()) && is_eq(next, b.clone(), e[..e.len()-len].parse().expect("")) {
        return true;
    }

    return false;
}
