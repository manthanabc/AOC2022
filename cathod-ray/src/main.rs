fn main() {
    let input = include_str!("../input.prod");
    let mut x: i32 = 1;
    let mut done = false;
    let mut sum = 0;
    let mut cycle = 0;
    let mut position = vec!["."; 40];
    for (i, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut line = line.split(" ");
        let isnt = line.next().unwrap();
        let pra = line.next();

        match isnt {
            "addx" => {
                tick(&mut position, &mut cycle, &mut sum, x);
                tick(&mut position, &mut cycle, &mut sum, x);
                x += pra.unwrap().parse::<i32>().unwrap();
            }
            _ => tick(&mut position, &mut cycle, &mut sum, x),
        };
    }
    println!("Sum: {sum}");
}

fn tick(position: &mut Vec<&str>, cycle: &mut i32, sum: &mut i32, x: i32) {
    if (*cycle % 40 - x).abs() <= 1 {
        let index: usize = (*cycle % 40).try_into().unwrap();
        position[index] = "#";
    }
    *cycle += 1;
    if (*cycle) % 40 == 0 {
        print(position);
        *position = vec!["."; 40];
    }
    if (*cycle -20) %40 == 0 {
        *sum+=x* *cycle;
    }
}

fn print(veec: &mut Vec<&str>) {
    let mut it = veec.iter();
    while let Some(vec) = it.next() {
        print!("{vec}");
    }
    println!();
}
