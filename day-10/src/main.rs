fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String{
    let mut grid = Vec::new();

    for line in input.lines(){
        let mut grid_line = Vec::new();
        for char in line.chars(){
            grid_line.push(char.to_digit(10).unwrap());

        }
        grid.push(grid_line);

    }

    return "".to_string();
}
