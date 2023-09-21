use std::io;

fn turn_l(direction: &str) -> &str {
    if direction.eq("N") {
        return "W";
    } else if direction.eq("E") {
        return "N";
    } else if direction.eq("S") {
        return "E";
    }
    return "S";
}

fn turn_r(direction: &str) -> &str {
    if direction.eq("N") {
        return "E";
    } else if direction.eq("E") {
        return "S";
    } else if direction.eq("S") {
        return "W";
    }
    return "N";
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).expect("Failed to read line");
    let mut numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Invalid number"));
    numbers.next().expect("No input provided");
    numbers.next().expect("No input provided");
    input.clear();
    while stdin.read_line(&mut input).expect("Failed to read line") > 0 {
        let mut strings = input
            .trim()
            .split_whitespace().clone();
        let mut rover_x = strings.next().map(|x| x.parse::<i32>().expect("Invalid number")).expect("NaN");
        let mut rover_y = strings.next().map(|x| x.parse::<i32>().expect("Invalid number")).expect("Nan");
        let mut rover_d = strings.next().expect("Empty");
        drop(strings);
        println!("{} {} {}", rover_x, rover_y, rover_d);
        let mut new_input = String::new();
        stdin.read_line(&mut new_input).expect("Failed to read line");
        for character in new_input.chars() {
            if character == 'L' {
                rover_d = turn_l(rover_d);
            } else if character == 'R' {
                rover_d = turn_r(rover_d);
            } else if character == 'M' {
                if rover_d.eq("N") { rover_y += 1; } else if rover_d.eq("S") {
                    rover_y -= 1;
                } else if rover_d.eq("W") {
                    rover_x -= 1;
                } else {
                    rover_x += 1;
                }
            } else {
                break;
            }
        }
        println!("{} {} {}", rover_x, rover_y, rover_d);
        input = String::new();
    }
}
