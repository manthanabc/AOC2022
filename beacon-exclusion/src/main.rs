use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.prod");
    
    let mut sensors: Vec<((i32, i32), (i32, i32), i32)> = Vec::new();
    input.lines().for_each(|lin| {
        let spl: Vec<&str> = lin.split([' ', '=', ',', ':']).collect();
        //println!("px {:?} py {:?}  bx {:?} by {:?}", spl[3], spl[6], spl[13], spl[16]);
        let x: Vec<i32> = [spl[3], spl[6], spl[13], spl[16]].iter().map(|f| f.parse::<i32>().unwrap()).collect();
        sensors.push(((x[0], x[1]), (x[2], x[3]),  manhattan_dis((x[0], x[1]), (x[2], x[3]))));
        // println!("{:?}", sensors);
    });
    // let mut grido: Vec<Vec<u64>>= vec![vec![0; 62500];4000000];
    // let lines:Vec<&str> = input.lines().collect();
    let mut outaed: HashSet<(i32, i32)> = HashSet::new();

    let mut counter = 0 ;
    // for j in 0..4000000 {
    // for i in 0..4000000 {
        let mut is_covered = false;
        for sensor in sensors.iter() {
            //let pos = (i, j);//(i,  2000000);// (i, 10);
            //let dis1 = manhattan_dis(pos, sensor.0);
            // let dis1 = manhattan_dis(pos, sensor.1);
            //let dis2 = sensor.2;
            /* if sensors.iter().any(|s| s.0 == pos || s.1 == pos) {
                continue;
            } */

            // go round round poney
            
            let (x, y) = sensor.0;
            let p1 = (sensor.0.0, sensor.0.1 + sensor.2 + 1);
            let p2 = (sensor.0.0, sensor.0.1 - sensor.2 - 1);

            let x1 = (sensor.0.0 + sensor.2 + 1, sensor.0.1);
            let x2 = (sensor.0.0 - sensor.2 - 1, sensor.0.1);



             println!("for s {x} {y}  p1, x1 is {:?} {:?}", p1, x1);
            for i in 0..(p1.0- x1.0).abs()+1 {
                 puush(&(p1.0+i,p1.1-i) , &mut outaed);
            }
             println!("for s {x} {y}  p1, x1 is {:?} {:?}", p1, x1);
            for i in 0..(p2.0- x1.0).abs()+1 {
                 puush(&(p2.0+i,p2.1+i) , &mut outaed);
            }
             println!("for s {x} {y}  p1, x1 is {:?} {:?}", p1, x1);
            for i in 0..(p2.0- x2.0).abs()+1 {
                 puush(&(p2.0-i,p2.1+i) , &mut outaed);
            }
             println!("for s {x} {y}  p1, x1 is {:?} {:?}", p1, x1);
            for i in 0..(p1.0- x2.0).abs()+1 {
                 puush(&(p1.0-i,p1.1-i) , &mut outaed);
            }
             println!("for s {x} {y}  p1, x1 is {:?} {:?}", p1, x1);
        }

        for i in &outaed.clone() {
         for sensor in sensors.iter() {
            let pos = i;//(i,  2000000);// (i, 10);
            let dis1 = manhattan_dis(*pos, sensor.0);
            // let dis1 = manhattan_dis(pos, sensor.1);
            let dis2 = sensor.2;
            if dis2 >= dis1 {//if dis2 >= dis1 {
                //let hi = outaed.iter().position(|f| f==i);
                //if let Some(g) = hi {
                outaed.remove(i); //}
            }           
        }
         }
            /*
            for i in 1..sensor.2 {
                let grid = &mut grido;
                if i == 1 {
                    set((x+1, y), '#', grid);
                    set((x, y+1), '#', grid);
                    set((x-1, y), '#', grid);
                    set((x, y-1), '#', grid);
                } else {
                    set((x+i, y), '#', grid);
                    set((x, y+i), '#', grid);
                    set((x-i, y), '#', grid);
                    set((x, y-i), '#', grid);

                    let i = i-1;
                    set((x+i, y+i), '#', grid);
                    set((x+i, y-i), '#', grid);
                    set((x-i, y+i), '#', grid);
                    set((x-i, y-i), '#', grid);
                }
            } */
            /* if dis2 >= dis1 {//if dis2 >= dis1 {
                is_covered = true;
                //counter += 1;
                break;
            } */
        //    if !is_covered { println!("{i} {j}"); }
    // }
     //       if j%10000 == 0 { println!("at{j}"); }
    // }
    println!("counter {:?}", outaed);
}

fn set(pos: (i32, i32), sy: char, grid:&mut Vec<Vec<u64>>) {
        if pos.0 < 0 { return }
        if pos.1 < 0 { return }
        if pos.1 >= grid.len() as i32 { return }
        if pos.0 >= grid[0].len() as i32 { return }

        grid[pos.1 as usize][pos.0 as usize] = 1;
}

fn puush(pos: &(i32, i32), grid:&mut HashSet<(i32, i32)>) {
        if pos.0 < 0 { return }
        if pos.1 < 0 { return }
        if pos.1 >= 4000000 { return }
        if pos.0 >= 4000000 { return }

        //println!("{:?}", pos);
        if !grid.contains(&pos) {
        grid.insert(*pos); }
}

fn manhattan_dis(point1: (i32, i32), point2: (i32, i32)) -> i32{
    (point1.0-point2.0).abs() + (point1.1-point2.1).abs()
}
