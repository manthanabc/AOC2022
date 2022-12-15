use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.prod");

    let mut sensors: Vec<((i32, i32), (i32, i32), i32)> = Vec::new();
    input.lines().for_each(|lin| {
        let spl: Vec<&str> = lin.split([' ', '=', ',', ':']).collect();
        let x: Vec<i32> = [spl[3], spl[6], spl[13], spl[16]]
            .iter()
            .map(|f| f.parse::<i32>().unwrap())
            .collect();
        sensors.push((
            (x[0], x[1]),
            (x[2], x[3]),
            manhattan_dis((x[0], x[1]), (x[2], x[3])),
        ));
    });

    let mut counter = 0;
    for i in -300000..4000000 {
         for sensor in sensors.iter() {
            let dis1 = manhattan_dis((i, 2000000), sensor.0);
            let dis2 = sensor.2;
            if dis2 >= dis1 {
                counter += 1;
            }
        }
    }

    println!("Part 1 {}", counter);

    // EDGES OF SENSOR AREA
    let mut outaed: HashSet<(i32, i32)> = HashSet::new();

    for sensor in sensors.iter() {
        let (x, y) = sensor.0;

        // CREATE A POINT ABOVE AND BELOW THE SENSOR AREA
        let p1 = (sensor.0 .0, sensor.0 .1 + sensor.2 + 1);
        let p2 = (sensor.0 .0, sensor.0 .1 - sensor.2 - 1);

        // ------------------||----------- LEFT AND RIGHT
        let x1 = (sensor.0 .0 + sensor.2 + 1, sensor.0 .1);
        let x2 = (sensor.0 .0 - sensor.2 - 1, sensor.0 .1);

        // DIAGONAL BOTTOM TO RIGHT
        for i in 0..(p1.0 - x1.0).abs() + 1 {
            puush(&(p1.0 + i, p1.1 - i), &mut outaed);
        }

        // DIAGONAL TOP TO RIGHT
        for i in 0..(p2.0 - x1.0).abs() + 1 {
            puush(&(p2.0 + i, p2.1 + i), &mut outaed);
        }

        // DIAGONAL TOP TO LEFT
        for i in 0..(p2.0 - x2.0).abs() + 1 {
            puush(&(p2.0 - i, p2.1 + i), &mut outaed);
        }

        // DIAGONAL BOTTOM TO LEFT
        for i in 0..(p1.0 - x2.0).abs() + 1 {
            puush(&(p1.0 - i, p1.1 - i), &mut outaed);
        }
    }

    for i in &outaed.clone() {
        for sensor in sensors.iter() {
            let pos = i;
            let dis1 = manhattan_dis(*pos, sensor.0);
            let dis2 = sensor.2;
            if dis2 >= dis1 {
                outaed.remove(i);
            }
        }
    }

    let last_left = outaed.iter().next().unwrap();
    println!("Part 2 {:?}", (last_left.0 as u64 *4000000u64) + last_left.1 as u64);
}

fn set(pos: (i32, i32), sy: char, grid: &mut Vec<Vec<u64>>) {
    if pos.0 < 0 {
        return;
    }
    if pos.1 < 0 {
        return;
    }
    if pos.1 >= grid.len() as i32 {
        return;
    }
    if pos.0 >= grid[0].len() as i32 {
        return;
    }

    grid[pos.1 as usize][pos.0 as usize] = 1;
}

fn puush(pos: &(i32, i32), grid: &mut HashSet<(i32, i32)>) {
    if pos.0 < 0 {
        return;
    }
    if pos.1 < 0 {
        return;
    }
    if pos.1 >= 4000000 {
        return;
    }
    if pos.0 >= 4000000 {
        return;
    }

    if !grid.contains(&pos) {
        grid.insert(*pos);
    }
}

fn manhattan_dis(point1: (i32, i32), point2: (i32, i32)) -> i32 {
    (point1.0 - point2.0).abs() + (point1.1 - point2.1).abs()
}
