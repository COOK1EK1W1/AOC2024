use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    //println!("{}", part1(input));
    let input2 = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    println!("{}", part1(input));
}

fn part1(input: &str) -> String{
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines(){
        let mut grid_lines = Vec::new();
        for char in line.chars(){
            grid_lines.push(char)
        }
        grid.push(grid_lines);
    }
    let height = grid.len();
    let width = grid[0].len();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    let mut cost = 0;
    let mut amounts: HashMap<char, (u32, u32)> = HashMap::new();
    for y in 0..height{
        for x in 0..width{
            let item = grid[y][x];
            if seen.contains(&(x, y)){
                continue;
            }
            let mut open: Vec<(usize, usize)> = vec![(x, y)];
            let mut fence_len = 0;
            let mut area = 0;
            let mut current_region: HashSet<(i32, i32)> = HashSet::new();
            while open.len() > 0{
                let (cur_x, cur_y) = open.pop().unwrap();
                if seen.contains(&(cur_x, cur_y)){continue;}
                seen.insert((cur_x, cur_y));
                current_region.insert((cur_x as i32, cur_y as i32));
                fence_len += not_same_neighbors(&grid, item, cur_x, cur_y);
                // left
                if cur_x > 0 && grid[cur_y][cur_x - 1] == item{
                    if !seen.contains(&(cur_x - 1, cur_y)){
                        open.push((cur_x - 1, cur_y));
                    }
                }
                //right
                if cur_x < width - 1 && grid[cur_y][cur_x+1] == item{
                    if !seen.contains(&(cur_x + 1, cur_y)){
                        open.push((cur_x + 1, cur_y));
                    }
                }
                //top
                if cur_y > 0 && grid[cur_y - 1][cur_x] == item{
                    if !seen.contains(&(cur_x, cur_y - 1)){
                        open.push((cur_x, cur_y - 1));
                    }
                }
                //bottom
                if cur_y < height - 1 && grid[cur_y+1][cur_x] == item{
                    if !seen.contains(&(cur_x, cur_y + 1)){
                        open.push((cur_x, cur_y + 1));
                    }
                }
                area += 1;
            }
            let wall_cost = wall_directions(current_region);
            println!("{} {}", area, wall_cost);
            cost += area * wall_cost;
        }

    }



    return cost.to_string();
}

fn not_same_neighbors(grid: &Vec<Vec<char>>, cur: char, x: usize, y: usize) -> u32{

    let height = grid.len();
    let width = grid[0].len();
    let mut total = 0;
    //left
    if x == 0 || grid[y][x - 1] != cur{
        total += 1;
    }
    //right
    if x == width - 1 || grid[y][x+1] != cur{
        total += 1;
    }
    //top
    if y == 0 || grid[y - 1][x] != cur{
        total += 1;
    }
    //bottom
    if y == height - 1 || grid[y+1][x] != cur{
        total += 1;

    }

    //println!("{} {} {} {}", cur, x, y, total);
    return total;
}

fn wall_directions(locs: HashSet<(i32, i32)>) -> u32{
    let mut total = 0;
    for &(x, y) in locs.iter(){
        // NE
        if !(locs.contains(&(x + 1, y)) || locs.contains(&(x, y- 1))){
            total += 1
        }
        //SE
        if !(locs.contains(&(x + 1, y)) || locs.contains(&(x, y+ 1))){
            total += 1
        }
        //SW
        if !(locs.contains(&(x - 1, y)) || locs.contains(&(x, y+ 1))){
            total += 1
        }
        //NW
        if !(locs.contains(&(x - 1, y)) || locs.contains(&(x, y- 1))){
            total += 1
        }



        // NE
        if locs.contains(&(x + 1, y)) && locs.contains(&(x, y- 1)) && !locs.contains(&(x+1, y-1)){
            total += 1
        }
        //SE
        if locs.contains(&(x + 1, y)) && locs.contains(&(x, y+ 1)) && !locs.contains(&(x+1, y+1)){
            total += 1
        }
        //SW
        if locs.contains(&(x - 1, y)) && locs.contains(&(x, y+ 1)) && !locs.contains(&(x-1, y+1)){
            total += 1
        }
        //NW
        if locs.contains(&(x - 1, y)) && locs.contains(&(x, y- 1)) && !locs.contains(&(x-1, y-1)){
            total += 1
        }

    }

    return total;
}
