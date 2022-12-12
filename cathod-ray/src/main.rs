fn main() {
    let input = include_str!("../badapple.elfasm");
    let mut x: i32 = 1;
    let mut done = false;
    let mut sum = 0;
    let mut cycle = 0;
    let mut position = vec![" "; 40];
    let mut line_counter = 0;
    for (i, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut line = line.split(" ");
        let isnt = line.next().unwrap();
        let pra = line.next();

        match isnt {
            "addx" => {
                tick(&mut position, &mut cycle, &mut sum, x, &mut line_counter);
                tick(&mut position, &mut cycle, &mut sum, x, &mut line_counter);
                x += pra.unwrap().parse::<i32>().unwrap();
            }
            _ => tick(&mut position, &mut cycle, &mut sum, x, &mut line_counter),
        };
        // for i in 0..10000 {}
       /* if i% (39*3) == 0 {
            for t in 0..100000000 {
                //dbg!("f");
            }
        } */
    }

    println!("Sum: {sum}");
}

fn tick(position: &mut Vec<&str>, cycle: &mut i32, sum: &mut i32, x: i32, lc: &mut u64) {
    if (*cycle  %40 - x).abs() <= 1 {
        let index: usize = (*cycle % 40).try_into().unwrap();
        position[index] = "█";
    }
    *cycle += 1;
    if (*cycle)  %40 == 0 {
        print(position, lc);
        *position = vec![" "; 40];
    }
    if (*cycle -20) %40 == 0 {
        // *sum+=x* *cycle;
    }
}

fn print(veec: &mut Vec<&str>, lc: &mut u64) {
    let mut it = veec.iter();
    while let Some(vec) = it.next() {
        print!("{vec}");
    }
    println!();
    *lc += 1;
    if *lc % 32 == 0 {
        for i in 0..5000000 {}
    }
}
