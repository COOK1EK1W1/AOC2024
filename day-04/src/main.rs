fn main() {
    let input = include_str!("input.txt");

    let input2 = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";


    println!("{}", part2(input));
    println!("{}", part2(input2));
}

fn part1(input: &str) -> String{

    let mut count = 0;
    let find_str = "XMAS";
    let width = input.lines().collect::<Vec<&str>>().get(0).expect("b").chars().collect::<Vec<char>>().len();
    let height = input.lines().collect::<Vec<&str>>().len();
    let input_lines: Vec<&str> = input.lines().collect();

    // horizontal
    for y in 0..height{
        for x in 0..(width - (find_str.len() - 1)){
            let mut found = true;
            for (i, char) in find_str.chars().enumerate(){
                let line = input_lines.get(y).expect("no line");
                let &sq_char = line.chars().collect::<Vec<char>>().get(x+i).expect("bruh");
                if sq_char != char{
                    found = false;
                }
            }
            if found {count += 1}
        }
    }

    // vertical
    for y in 0..(height - (find_str.len() - 1)){
        for x in 0..(width){
            let mut found = true;
            for (i, char) in find_str.chars().enumerate(){
                let line = input_lines.get(y+i).expect("no line");
                let &sq_char = line.chars().collect::<Vec<char>>().get(x).expect("bruh");
                if sq_char != char{
                    found = false;
                }
            }
            if found {count += 1}
        }
    }

    // diag \
    for y in 0..(height - (find_str.len() - 1)){
        for x in 0..(width - (find_str.len() - 1)){
            let mut found = true;
            for (i, char) in find_str.chars().enumerate(){
                let line = input_lines.get(y+i).expect("no line");
                let &sq_char = line.chars().collect::<Vec<char>>().get(x+i).expect("bruh");
                if sq_char != char{
                    found = false;
                }
            }
            if found {count += 1}
        }
    }

    // diag /
    for y in find_str.len()-1..(height){
        for x in 0..(width - find_str.len() + 1){
            let mut found = true;
            for (i, char) in find_str.chars().enumerate(){
                let line = input_lines.get(y-i).expect("no line");
                let &sq_char = line.chars().collect::<Vec<char>>().get(x+i).expect("bruh");
                if sq_char != char{
                    found = false;
                }
            }
            if found {count += 1}
        }
    }






    // horizontal -backwards
    for y in 0..height{
        for x in find_str.len()-1..(width){
            let mut found = true;
            for (i, char) in find_str.chars().enumerate(){
                let line = input_lines.get(y).expect("no line");
                let &sq_char = line.chars().collect::<Vec<char>>().get(x-i).expect("bruh");
                if sq_char != char{
                    found = false;
                }
            }
            if found {count += 1}
        }
    }

    // vertical - backwards
    for y in find_str.len()-1..(height){
        for x in 0..(width){
            let mut found = true;
            for (i, char) in find_str.chars().enumerate(){
                let line = input_lines.get(y-i).expect("no line");
                let &sq_char = line.chars().collect::<Vec<char>>().get(x).expect("bruh");
                if sq_char != char{
                    found = false;
                }
            }
            if found {count += 1}
        }
    }

    // diag \
    for y in find_str.len() - 1..(height){
        for x in find_str.len() - 1..(width){
            let mut found = true;
            for (i, char) in find_str.chars().enumerate(){
                let line = input_lines.get(y-i).expect("no line");
                let &sq_char = line.chars().collect::<Vec<char>>().get(x-i).expect("bruh");
                if sq_char != char{
                    found = false;
                }
            }
            if found {count += 1}
        }
    }

    // diag /
    for y in 0..(height - find_str.len() + 1){
        for x in find_str.len() - 1..(width){
            let mut found = true;
            for (i, char) in find_str.chars().enumerate(){
                let line = input_lines.get(y+i).expect("no line");
                let &sq_char = line.chars().collect::<Vec<char>>().get(x-i).expect("bruh");
                if sq_char != char{
                    found = false;
                }
            }
            if found {count += 1}
        }
    }
    count.to_string()
}



fn part2(input: &str) -> String{
    let mut count = 0;
    let width = input.lines().collect::<Vec<&str>>().get(0).expect("b").chars().collect::<Vec<char>>().len();
    let height = input.lines().collect::<Vec<&str>>().len();
    let input_lines: Vec<&str> = input.lines().collect();

    // horizontal
    for y in 1..height-1{
        for x in 1..width-1{
            let line = input_lines.get(y).expect("no line");
            let &sq_char = line.chars().collect::<Vec<char>>().get(x).expect("bruh");

            if sq_char == 'A'{
                let tline = input_lines.get(y-1).expect("no line");
                let bline = input_lines.get(y+1).expect("no line");
                let &tl = tline.chars().collect::<Vec<char>>().get(x-1).expect("bruh");
                let &tr = tline.chars().collect::<Vec<char>>().get(x+1).expect("bruh");
                let &bl = bline.chars().collect::<Vec<char>>().get(x-1).expect("bruh");
                let &br = bline.chars().collect::<Vec<char>>().get(x+1).expect("bruh");

                if tl == 'S' && tr == 'S' && br == 'M' && bl == 'M'{count += 1};
                if tl == 'M' && tr == 'S' && br == 'S' && bl == 'M'{count += 1;};
                if tl == 'M' && tr == 'M' && br == 'S' && bl == 'S'{count += 1;};
                if tl == 'S' && tr == 'M' && br == 'M' && bl == 'S'{count += 1;};

            }
        }
    }
    count.to_string()


}
