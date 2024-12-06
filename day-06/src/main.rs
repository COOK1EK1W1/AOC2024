use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let input2 = 
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    println!("{}", part2(input2));
    println!("{}", part2(input));
}


#[derive(PartialEq, PartialOrd)]
#[derive(Debug)]
enum LocType{
    Empty,
    Wall,
    Visited
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Dir{
    North,
    East,
    South,
    West
}


fn print_grid(grid: &Vec<Vec<LocType>>){
    for line in grid.iter(){
        for loc in line.iter(){
            match loc{
                LocType::Empty => print!("."),
                LocType::Wall => print!("#"),
                LocType::Visited => print!("X")
            }

        }
        print!("\n");

    }
    println!("\n\n")

}

fn part1(input: &str) -> String{
    let mut grid: Vec<Vec<LocType>> = Vec::new();
    let mut posx: i32 = 0;
    let mut posy: i32 = 0;
    let mut dir = Dir::North;


    //parse grid
    for (y, line) in input.lines().enumerate(){
        let mut grid_line: Vec<LocType> = Vec::new();
        for (x, char) in line.chars().enumerate(){
            match char{
                '.' => grid_line.push(LocType::Empty),
                '#' => grid_line.push(LocType::Wall),
                '^' => {
                    posx = x as i32;
                    posy = y as i32;
                    grid_line.push(LocType::Empty);
                },
                _ => ()

            }
        }
        grid.push(grid_line);

    }



    let width = grid.len() as i32;
    let height = grid[0].len() as i32;
    
    let mut total = 0;
    // run 
    loop{
        let mut move_dir = vec![0, 0];
        match dir{
            Dir::North =>{move_dir[1] = -1},
            Dir::East =>{move_dir[0] = 1},
            Dir::South =>{move_dir[1] = 1},
            Dir::West =>{move_dir[0] = -1},
        }
        let checkx = posx + move_dir[0];
        let checky = posy + move_dir[1];
        if !(checkx < width && checkx >= 0 && checky < height && checky >= 0){
            grid[posy as usize][posx as usize] = LocType::Visited;
            total += 1;
            break;
        }
        let next_sq = &grid[checky as usize][checkx as usize];
        match next_sq{
            LocType::Wall => {
                match dir{
                    Dir::North =>{dir = Dir::East},
                    Dir::East =>{dir = Dir::South},
                    Dir::South =>{dir = Dir::West},
                    Dir::West =>{dir = Dir::North},
                }

            },
            LocType::Empty | LocType::Visited => {
                match grid[posy as usize][posx as usize]{
                    LocType::Empty =>{
                        total += 1;
                        grid[posy as usize][posx as usize] = LocType::Visited;
                        },
                        _ => ()
                }
                posx += move_dir[0];
                posy += move_dir[1];
            }
        }

    }

    print_grid(&grid);


    return total.to_string();

}



fn is_loop(mut grid: Vec<Vec<LocType>>, startx: i32, starty: i32) -> bool{
    let mut posx: i32 = startx;
    let mut posy: i32 = starty;
    let mut dir = Dir::North;
    let mut visited: HashSet<(i32, i32, Dir)> = HashSet::new();


    //parse grid


    let width = grid.len() as i32;
    let height = grid[0].len() as i32;
    
    // run 
    loop{
        visited.insert((posx, posy, dir.clone()));
        let mut move_dir = vec![0, 0];
        match dir{
            Dir::North =>{move_dir[1] = -1},
            Dir::East =>{move_dir[0] = 1},
            Dir::South =>{move_dir[1] = 1},
            Dir::West =>{move_dir[0] = -1},
        }
        let checkx = posx + move_dir[0];
        let checky = posy + move_dir[1];
        if !(checkx < width && checkx >= 0 && checky < height && checky >= 0){
            grid[posy as usize][posx as usize] = LocType::Visited;
            break;
        }
        let next_sq = &grid[checky as usize][checkx as usize];
        match next_sq{
            LocType::Wall => {
                match dir{
                    Dir::North =>{dir = Dir::East},
                    Dir::East =>{dir = Dir::South},
                    Dir::South =>{dir = Dir::West},
                    Dir::West =>{dir = Dir::North},
                }

            },
            LocType::Empty => {
                match grid[posy as usize][posx as usize]{
                    LocType::Empty =>{
                        grid[posy as usize][posx as usize] = LocType::Visited;
                    },
                    _ => ()
                }
                posx += move_dir[0];
                posy += move_dir[1];
            },
            LocType::Visited => {


                match grid[posy as usize][posx as usize]{
                    LocType::Empty =>{
                        grid[posy as usize][posx as usize] = LocType::Visited;
                    },
                    LocType::Visited => {
                    },
                    _ => ()
                }
                posx += move_dir[0];
                posy += move_dir[1];
            }
        }

        if visited.contains(&(posx, posy, dir.clone())){return true}
    }

    return false

}



fn part2(input: &str) -> String{
    let mut grid: Vec<Vec<LocType>> = Vec::new();
    let mut posx: i32 = 0;
    let mut posy: i32 = 0;
    let mut dir = Dir::North;


    //parse grid
    for (y, line) in input.lines().enumerate(){
        let mut grid_line: Vec<LocType> = Vec::new();
        for (x, char) in line.chars().enumerate(){
            match char{
                '.' => grid_line.push(LocType::Empty),
                '#' => grid_line.push(LocType::Wall),
                '^' => {
                    posx = x as i32;
                    posy = y as i32;
                    grid_line.push(LocType::Empty);
                },
                _ => ()

            }
        }
        grid.push(grid_line);

    }



    let width = grid.len() as i32;
    let height = grid[0].len() as i32;
    
    // run 
    loop{
        let mut move_dir = vec![0, 0];
        match dir{
            Dir::North =>{move_dir[1] = -1},
            Dir::East =>{move_dir[0] = 1},
            Dir::South =>{move_dir[1] = 1},
            Dir::West =>{move_dir[0] = -1},
        }
        let checkx = posx + move_dir[0];
        let checky = posy + move_dir[1];
        if !(checkx < width && checkx >= 0 && checky < height && checky >= 0){
            grid[posy as usize][posx as usize] = LocType::Visited;
            break;
        }
        let next_sq = &grid[checky as usize][checkx as usize];
        match next_sq{
            LocType::Wall => {
                match dir{
                    Dir::North =>{dir = Dir::East},
                    Dir::East =>{dir = Dir::South},
                    Dir::South =>{dir = Dir::West},
                    Dir::West =>{dir = Dir::North},
                }

            },
            LocType::Empty | LocType::Visited => {
                match grid[posy as usize][posx as usize]{
                    LocType::Empty =>{
                        grid[posy as usize][posx as usize] = LocType::Visited;
                        },
                        _ => ()
                }
                posx += move_dir[0];
                posy += move_dir[1];
            }
        }

    }
                    print_grid(&grid);
    let mut count = 0;

    for (test_y, line) in grid.iter().enumerate(){
        for (test_x, item) in line.iter().enumerate(){
            match item{
                LocType::Visited => {
                    //parse grid
                    let mut startx = 0;
                    let mut starty = 0;
                    let mut new_grid: Vec<Vec<LocType>> = Vec::new();
                    for (y, line) in input.lines().enumerate(){
                        let mut grid_line: Vec<LocType> = Vec::new();
                        for (x, char) in line.chars().enumerate(){
                            match char{
                                '.' => {
                                    if test_x == x && test_y == y{
                                        grid_line.push(LocType::Wall);

                                    }else{
                                        grid_line.push(LocType::Empty)
                                    }
                                },
                                '#' => grid_line.push(LocType::Wall),
                                '^' => {
                                    startx = x as i32;
                                    starty = y as i32;
                                    grid_line.push(LocType::Empty);
                                },
                                _ => ()

                            }
                        }
                        new_grid.push(grid_line)
                    }
                    //print_grid(&new_grid);
                    if is_loop(new_grid, startx, starty) {count += 1}



                },
                _ => ()

            }

        }
    }

    return count.to_string();
}
