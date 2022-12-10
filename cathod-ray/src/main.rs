
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
        let pra  = line.next();
        /* (0..if isnt=="addx" {2} else {0}).for_each(|_| {
            
            cycle += 1
        }); */
       
        match isnt {
            "addx" => {
                /* if (cycle -20) %40 == 0 { println!("----{cycle} {x} {i}");
                    sum+=x*cycle;
                } */
                if (cycle%40-x).abs() <= 1 {
                    let index: usize = (cycle%40).try_into().unwrap();
                    position[index] = "#";        
                }
                cycle+=1;
                if (cycle%40-x).abs() <= 1 {
                    let index: usize = (cycle%40).try_into().unwrap();
                    position[index] = "#";        
                }
                if (cycle )% 40 == 0 { // println!("----{cycle} {x} {i}");
                    print(&mut position);
                    // println!("{x}");
                    position = vec!["."; 40];
                    sum+=x*cycle;
                }
                cycle+=1;
                if (cycle )% 40 == 0 { // println!("----{cycle} {x} {i}");
                    print(&mut position);
                    // println!("{x}");
                    position = vec!["."; 40];

                    sum+=x*cycle;
                } 
                x+= pra.unwrap().parse::<i32>().unwrap();
                continue;
            },
            _   => ()
        };
                if (cycle%40-x).abs() <= 1 {
                    let index: usize = (cycle%40).try_into().unwrap();
                    position[index] = "#";        
                }
        cycle += 1;
        if (cycle )% 40 == 0 {
            //println!("{:?}", position);
                    print(&mut position);
            position = vec!["."; 40];
            // println!("--------------------{cycle} {x} {:?}", pra);
            sum += x*cycle;
        } //if (cycle -20)% 40 == 0 { sum+=x*cycle };
            /*            if (cycle -20)% 40 == 0 {
            println!("--------------------{cycle} {x} {:?}", pra);
            sum += x*cycle;
        } */
        // println!("{} {x}{:?}", isnt, pra);
    }
    println!("Hello, world! {sum}");
}

fn print(veec:&mut Vec<&str>) {
        let mut it =veec.iter();
        while let Some(vec) =  it.next() {
            print!("{vec}");
        }
        println!();
}
