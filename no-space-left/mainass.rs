use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.prod");
    let mut sizes = 0;
    let mut map:HashMap<String, u32> = HashMap::new();
    let size = find_consume("/", input, &mut map);
    dbg!(size);
    dbg!(map.values().fold(0, |acc, t| if *t < 100000 {acc+t} else {acc}));
}

fn find_consume(dirname: &str,input: &str, map: &mut HashMap<String, u32>) -> u32 {
    let mut sum = 0;
    /* if map.contains_key(dirname) {
        return sum;
    } else {
            map.insert(dirname.to_string(), 0);
    } */
    println!("finding {dirname}");
    let start = input.split_once(&format!("cd {dirname}")).unwrap().1;
    // println!("{:?}", input);
    let part = start.split_once("cd").unwrap().0;
    part.lines().for_each(|line| {
        if line.starts_with("dir") {
            sum += find_consume(line.split_once("dir").unwrap().1.trim(), input, map);
        } else if line.starts_with("$") {
        } else if !line.is_empty(){
            sum += line.split_once(" ").unwrap().0.parse::<u32>().unwrap();
            // println!("{}");
        }
    });
    // println!("{part}");
        map.insert(dirname.to_string(), sum);
    sum
}
