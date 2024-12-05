use std::collections::HashMap;


fn main() {
    let input = include_str!("input.txt");
    let input2 = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    println!("{}", part1(input));
    println!("{}", part1(input2));
}

fn part1(input: &str) -> String{
    let mut ans = 0;
    let splits: Vec<&str> = input.split("\n\n").collect();
    let rules = splits.get(0).expect("no rules");
    let orders = splits.get(1).expect("no orders");
    println!("rules: {}", rules);
    println!("orders: {}", orders);

    let allows: HashMap<u32, Vec<u32>> = parse_rules(rules);

    for order in orders.lines(){
        let mut valid = true;
        let mut page_nums: Vec<u32> = order.split(",").map(|p| p.trim().parse::<u32>().expect("a")).collect();
        println!("old: {:?}",page_nums);

        for (i, page) in page_nums.clone().iter().enumerate(){
            // pre rules
            // allows not includes pre
            for x in 0..i{
                match allows.get(page){
                    Some(entry) => {
                        if entry.contains(page_nums.get(x).expect("")){
                            page_nums.swap(x, i);
                            valid = false;
                        }
                    },
                    None => ()
                }
            }

            // post rules
            // allows includes post

        }
        println!("new: {:?}",page_nums);
        if !valid {
            let middle = page_nums.len() / 2;
            ans += page_nums[middle];
        };
    }

    ans.to_string()
}

fn parse_rules(rules_inp: &str) -> HashMap<u32, Vec<u32>>{
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in rules_inp.lines(){
        let pages: Vec<u32> = line.split("|").map(|p| p.trim().parse::<u32>().expect("bruh")).collect();
        let i1 = pages[0];
        let i2 = pages[1];

        rules.entry(i1).or_insert_with(Vec::new).push(i2);
    }
    return rules;
}
