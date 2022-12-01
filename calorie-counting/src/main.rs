use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("In file {}", "/input.txt");

    let contents: String = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file");

    let mut deers: Vec<Vec<&str>> = contents.rsplit("\n\n").map(|deer|  deer.split('\n').collect()).collect();
    let mut calories: Vec<i64> = vec![0; deers.len()];
    let mut max = [0; 3];
    for (i, deer) in deers.iter().enumerate() {
        let deeer= deer.iter().filter(|d| !d.is_empty());
        let val: Vec<i64>  = deeer.map(|x| x.parse::<i64>().unwrap()).collect();
        calories[i] = val.iter().sum();
        if calories[i] > max[0] {
            max[2] = max[1];
            max[1] = max[0];
            max[0] = calories[i];
        }
        else if calories[i] > max[1] {
            max[2] = max[1];
            max[1] = calories[i];
        }
        else if calories[i] > max[2] {
            max[2] = calories[i];
        }
        println!("{:?}, {i}", calories[i])
    }

    println!("With text:\n{:?}, {:?}", calories, max);
}
