use std::{collections::HashSet, usize};

fn main() {
    let input = include_str!("input.txt");
    let input2 = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    println!("{}", part1(input));
    println!("{}", "");
    println!("{}", part1(input2));
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
    //println!("{:?}", grid);
    let mut total = 0;
    for (y, line) in grid.iter().enumerate(){
        for (x, &i) in line.iter().enumerate(){
            if i == 0{
                let a = find_paths(&grid, x as u32, y as u32);
                println!("{}", a);
                total += a; 
            }
        }
    }

    return total.to_string();
}

fn find_paths(grid: &Vec<Vec<u32>>, x: u32, y: u32) -> u32{
    let mut positions: Vec<(u32, u32)> = Vec::new();
    let width = grid[0].len() as u32;
    let height = grid.len() as u32;
    positions.push((x, y));
    let mut total = 0;
    let mut found: HashSet<(u32, u32)> = HashSet::new();
    while positions.len() > 0{
        let (cur_x, cur_y) = positions.pop().unwrap();
        let cur_item = grid[cur_y as usize][cur_x as usize];
        if grid[cur_y as usize][cur_x as usize] == 9{
            found.insert((cur_x, cur_y));
            total += 1;
        }
        // up
        if cur_y > 0 && cur_item + 1 == grid[(cur_y - 1) as usize][cur_x as usize]{
            positions.push((cur_x, cur_y - 1))
        }
        //down
        if cur_y + 1 < height && cur_item + 1 == grid[(cur_y + 1) as usize][cur_x as usize]{
            positions.push((cur_x, cur_y + 1))
        }
        //left
        if cur_x > 0 && cur_item + 1== grid[cur_y as usize][(cur_x - 1) as usize]{
            positions.push((cur_x - 1, cur_y))
        }
        //right
        if cur_x + 1 < width && cur_item + 1 == grid[cur_y as usize][(cur_x + 1) as usize]{
            positions.push((cur_x + 1, cur_y))
        }

    }

    return total;
}
