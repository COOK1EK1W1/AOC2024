use regex::Regex;
use std::io;

fn main() {
    let input = include_str!("input.txt");
    let input2 = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    part2(input, 101, 103);
    //println!("{}", part1(input2, 11, 7, 100));
}

#[derive(Debug)]
struct Robot{
    pos: (i32, i32),
    vel: (i32, i32)
}

fn part1(input: &str, width: i32, height: i32, seconds: i32) -> String{
    let mut robots = Vec::new();
    let re = Regex::new(r"-?\d+").unwrap();
    for line in input.lines(){

        let b: Vec<i32> = re.find_iter(line)
            .filter_map(|m| m.as_str().parse::<i32>().ok())
            .collect();

        robots.push(Robot{pos: (b[0], b[1]), vel: (b[2], b[3])});
    }
    for _ in 0..seconds{
        for robot in robots.iter_mut(){
            robot.pos.0 = (robot.pos.0 + robot.vel.0 + width) % width;
            robot.pos.1 = (robot.pos.1 + robot.vel.1 + height) % height;
        }
        //print_robots(&robots, width, height);
    }

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    println!("");

    for y in 0..height{
        if y == (height) / 2{
            print!("\n");
            continue;
        }
        for x in 0..width{
            if x == (width) / 2{
                print!(" ");
                continue;
            }
            let mut total = 0;
            for robot in robots.iter(){
                if robot.pos.0 == x && robot.pos.1 == y{
                    total += 1;
                }
            }
            if total > 0{
                print!("{}", total);
                if x < width / 2 && y < height / 2{a += total}
                if x > width / 2 && y < height / 2{b += total}
                if x < width / 2 && y > height / 2{c += total}
                if x > width / 2 && y > height / 2{d += total}
            }else{
                print!(".");
            }
        }
        print!("\n");
    }

    println!("{} {} {} {}", a, b, c, d);

    (a * b * c * d).to_string()
}

fn part2(input: &str, width: i32, height: i32){
    let mut robots = Vec::new();
    let re = Regex::new(r"-?\d+").unwrap();
    for line in input.lines(){

        let b: Vec<i32> = re.find_iter(line)
            .filter_map(|m| m.as_str().parse::<i32>().ok())
            .collect();

        robots.push(Robot{pos: (b[0], b[1]), vel: (b[2], b[3])});
    }
    let mut i = 0;
    for _ in 0..7320{
        for robot in robots.iter_mut(){
            robot.pos.0 = (robot.pos.0 + robot.vel.0 + width) % width;
            robot.pos.1 = (robot.pos.1 + robot.vel.1 + height) % height;
        }
        i += 1;
    }
    loop{
            for robot in robots.iter_mut(){
                robot.pos.0 = (robot.pos.0 + robot.vel.0 + width) % width;
                robot.pos.1 = (robot.pos.1 + robot.vel.1 + height) % height;
            }
            i += 1;
        print_robots(&robots, width, height);
        println!("{}", i);
        let mut b= String::new();
        io::stdin().read_line(&mut b).expect("");
    }
}


fn print_robots(robots: &Vec<Robot>, width: i32, height: i32){
    for y in 0..height{
        for x in 0..width{
            let mut total = 0;
            for robot in robots.iter(){
                if robot.pos.0 == x && robot.pos.1 == y{
                    total += 1;
                }
            }
            if total == 0{
                print!(".");

            }else{

                print!("{}", total);
            }
        }
        println!("");
    }
}

