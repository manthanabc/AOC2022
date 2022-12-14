
fn main() {
    let input = include_str!("../input.test");
    let mut grid :Vec<Vec<char>> = vec![vec!['.'; 1610]; 1610];

    input.lines().for_each(|lin| {
        let mut last_elm: Option<(usize, usize)> = None;
        lin.split("->").for_each(|elm| {
                let f: (&str, &str) = elm.split_once(',').unwrap();
                // println!("{:?}", f.0.trim().parse::<u32>());
                let thiselm: (usize, usize) =(f.0.trim().parse().unwrap(), f.1.trim().parse().unwrap());
            if let Some(last) = last_elm {
                // println!("{:?} {:?}", thiselm, last);
                if thiselm.0 > last.0 {
                    for i in last.0..thiselm.0+1 {
                        grid[thiselm.1][i] = '#';
                    }
                } else if thiselm.0 < last.0 {
                    for i in thiselm.0..last.0+1 {
                        // println!("{i}");
                        grid[thiselm.1][i] = '#';
                    }
                } else if thiselm.1 > last.1 {
                     for i in last.1..thiselm.1+1 {
                        grid[i][thiselm.0] = '#';
                    }                    
                } else if thiselm.1 < last.1 {
                     for i in thiselm.1..last.1+1 {
                        grid[i][thiselm.0] = '#';
                    }                    
                }

            }
                last_elm = Some(thiselm);
        });
    });

    // PART 2 FINDING THE LOWEST POINT
    let mut  max = 0;
    for i in 0..610 {
        for j in 0..610 {
            if grid[j][i] == '#' {
                if j > max {
                    max = j;
                }
            }
        }
    }
    println!("{max}");
    
    for i in 0..1610 {
        grid[max+2][i] = '#';
    }
    for i in 0..200000 { println!("{i}"); propagate((500, 0), &mut grid);  };
    
    printn(&grid);
    println!("Hello, world!");
}

fn propagate(pos: (usize, usize), grid: &mut Vec<Vec<char>>) -> bool {
     if pos.1 == grid.len() - 1 {
        grid[pos.1][pos.0] = '.';
        return
        panic!("kefwop");
    } 
    // println!("{:?}", pos);
    if grid[pos.1+1][pos.0] == '#' ||  grid[pos.1+1][pos.0] == 'o'{
        if grid[pos.1+1][pos.0-1] == '#' ||  grid[pos.1+1][pos.0-1] == 'o'{
            if grid[pos.1+1][pos.0+1] == '#' ||  grid[pos.1+1][pos.0+1] == 'o'{
                grid[pos.1][pos.0] = 'o';
    if pos.1 == 0 {
        panic!("kefop");
        return true;
    } 
                return false;
            } else {
                propagate((pos.0+1, pos.1+1), grid);
            }
        } else {
                propagate((pos.0-1, pos.1+1), grid);
        }
    } else {
                propagate((pos.0, pos.1+1), grid);
    }
    return false;
}

fn printn(grid: &Vec<Vec<char>>) {
    for ii in 0..14 {// for (ii, i) in grid.iter().enumerate() {
      for j in 494..505 {  //for j in &grid[ii] {
            print!("{}", grid[ii][j]);
       }
        println!();
    }
}
