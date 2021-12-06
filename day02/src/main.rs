use std::fs::File;
use std::io::Read;

fn main() {
    let path = format!("{}/src/test_input.txt", env!("CARGO_MANIFEST_DIR"));
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let outcome = content
        .split("\n")
        .fold((0, 0), |mut acc, line| { 
            let pair = line.split(" ").collect::<Vec<&str>>();
            let value = pair[1].parse::<usize>().unwrap();
            match pair {
                _ if pair[0] == "forward" => { acc.0 += value }
                _ if pair[0] == "up" => { acc.1 -= value }
                _ if pair[0] == "down" => { acc.1 += value } 
                _ => {}
            };
            acc
        });
    println!("{:?}", outcome.0 * outcome.1);
}
