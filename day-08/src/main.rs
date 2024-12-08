use std::{char, collections::{HashMap, HashSet}};

fn main() {
    let input = include_str!("input.txt");
    let input2 = 
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    let input3 = 
"T...........
...T........
.T..........
............
............
............
............
............
............
............
............
............";

// ......X....X
// ...X....0...
// ....X0....X.
// ..X....0....
// ....0....X..
// .X....A.....
// ...X........
// .......X....
// ........A...
// .........A..
// ..........X.
// ..........X.

    println!("{}", part1(input));
    println!("sol: {}", part1(input2));
}

fn part1(input: &str) -> String{
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let height: i32 = input.lines().collect::<Vec<&str>>().len() as i32;
    let width: i32 = input.lines().collect::<Vec<&str>>()[0].chars().collect::<Vec<char>>().len() as i32;

    for (y, line) in input.lines().enumerate(){
        for (x, char) in line.chars().enumerate(){
            if char != '.'{
                antennas.entry(char).or_insert_with(Vec::new).push((x as i32, y as i32));
                println!("{}", char);
            }
        }
    }

    let mut nodes = HashSet::new();
    for freq in antennas.keys(){
        let ants = antennas.get(freq).expect("");
        for (i, &ant_a) in ants.iter().enumerate(){
            for antB in i+1..ants.len(){
                let &ant_b = ants.get(antB).expect("");
                let offset_x = ant_b.0 - ant_a.0;
                let offset_y = ant_b.1 - ant_a.1;

                let mut anti_node1 = (ant_b.0, ant_b.1);
                while anti_node1.0 >= 0 && anti_node1.0 < width && anti_node1.1 >= 0 && anti_node1.1 < height{
                    nodes.insert(anti_node1);
                    anti_node1 = (anti_node1.0 + offset_x, anti_node1.1 + offset_y);
                }

                let mut anti_node2 = (ant_a.0, ant_a.1);
                while anti_node2.0 >= 0 && anti_node2.0 < width && anti_node2.1 >= 0 && anti_node2.1 < height {
                    nodes.insert(anti_node2);
                    anti_node2 = (anti_node2.0 - offset_x, anti_node2.1 - offset_y)
                }
            }


        }
        println!("{:?}", nodes);

    }


    for y in 0..width{
        for x in 0..height{
            if nodes.contains(&(x, y)){
                print!("x");
            }else{
                print!(".");
            }
        }
        print!("\n");

    }
    return nodes.len().to_string();

}
