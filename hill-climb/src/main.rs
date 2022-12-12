/* use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
  fn successors(&self) -> Vec<Pos> {
    let &Pos(x, y) = self;
    vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
         Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
  }
}
fn main() {
    static GOAL: Pos = Pos(4, 6);
    let result = bfs(&Pos(1, 1), |p| p.successors(), |p| *p == GOAL);
    assert_eq!(result.expect("no path found").len(), 5);
} */

/*
use pathfinding::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
  fn distance(&self, other: &Pos) -> usize {
    ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
  }

  fn neighbours(&self) -> Vec<(Pos, usize)> {
    let &Pos(x, y) = self;
    vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
         Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
         .into_iter().map(|p| (p, 1)).collect()
  }
} */

use std::cmp::max;

struct Position(usize, usize);

struct Node<'a> {
    position: Position,
    parent: &'a mut &'a Position,
    visited: bool,
    score: usize,
}

fn main() {
    let mut inputt: Vec<&str> = include_str!("../input.prod").lines().collect();
    let mut dist_map: Vec<Vec<i32>> = vec![vec![0; inputt[0].len()]; inputt.len()];
    let lines = inputt.iter().enumerate();

    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, inp) in lines.clone() {
        'm: for (j, ch) in inp.chars().enumerate() {
            match ch {
                'S' => start = (j, i),
                'E' => end = (j, i),
                _ => {}
            }
        }
    }
    let inpuut = &inputt[start.1].replace("S", "a");
    let inpuuut = &inputt[end.1].replace("E", "z");
    let mut input = inputt.clone();
    input[start.1] = &inpuut;
    input[end.1] = &inpuuut;
    println!("{:?} {:?}", start, end);
    for (i, inp) in lines.clone() {
        'm: for (j, ch) in inp.chars().enumerate() {
            dist_map[i][j] = (j as i32 - end.0 as i32) * (j as i32 - end.0 as i32)
                + (i as i32 - end.1 as i32) * (i as i32 - end.1 as i32);
        }
    }

    /* for (i, inp) in lines {
        'm: for (j, ch) in inp.chars().enumerate() {

            // let (mut start, mut end) = inp.split_at(j);
            // let end = &end[1..];
        }
    } */
    print(&dist_map);
    let mut stack: Vec<Node> = Vec::new();
    let mut found = false;
    let map = input;
    let start_node = Node {
        position: Position(start.0, start.1),
        parent: Position(start.0, start.1),
        visited: false,
        value: dist(start, end),
    };
    stack.push(Position(start.0, start.1));
    while !found() {
        stack.sort_by
        for node in stack {
            if !node.visited {
                let curr = node.position;
                for f in [
                    (curr.0 as i32, (curr.1 as i32 - 1) as i32),
                    ((curr.0 as i32 - 1) as i32, curr.1 as i32),
                    ((curr.0 + 1) as i32, curr.1 as i32),
                    (curr.0 as i32, (curr.1 + 1) as i32),
                ]
                .iter()
                {
                    if f.0 < 0 || f.0 >= dist_map[0].len() as i32 {
                        continue;
                    };
                    if f.1 < 0 || f.1 >= dist_map.len() as i32 {
                        continue;
                    };
                    let f: (usize, usize) = (f.0.try_into().unwrap(), f.1.try_into().unwrap());
                    if map[curr.1].as_bytes()[curr.0] as i8 - map[f.1].as_bytes()[f.0] as i8 <= 1 {
                       stack.push()
                    }
                }
            }
        }
    }
}

fn dist(curr: &(usize, usize), start: &(usize, usize), endd: &(usize, usize)) -> usize {
    
    let (i ,j) = curr;
    let end = start;
    let dstart = (j as i32 - end.0 as i32) * (j as i32 - end.0 as i32)
            + (i as i32 - end.1 as i32) * (i as i32 - end.1 as i32);

    let (i ,j) = curr;
    let end = endd;
    let dend   =(j as i32 - end.0 as i32) * (j as i32 - end.0 as i32)
            + (i as i32 - end.1 as i32) * (i as i32 - end.1 as i32);

    dstart + dend
}

/*
    let path = vec![start];
    let paht = continuee(&path, &input, &dist_map, &mut paths);

    let mut min = 100000000;
    let mut mini = 0;
    for i in 0..paths.len() {
        if paths[i].len() < min {
            min = paths[i].len();
            mini = i;
        }
    }
    println!("{min} {mini} {:?}", paths[mini]);
    for i in 0..dist_map.len() {
        for j in 0..dist_map[i].len() {
            print!("{:?}{:?}\t", dist_map[i][j], if paths.iter().any(|f| f.contains(&(j, i))) {"."} else {""});
        }
        println!();
    }
}

fn continuee(
    path: &Vec<(usize, usize)>,
    map: &Vec<&str>,
    dist_map: &Vec<Vec<i32>>,
    paths: &mut Vec<Vec<(usize, usize)>>
) -> Option<Vec<(usize, usize)>> {
    let curr = path[path.len() - 1];

    // println!("{:?}", dist_map[curr.1][curr.0]);

    if dist_map[curr.1][curr.0] == 0 {
        paths.push(path.clone());
        println!("{:?}", path);
        // return Some(path.clone());
    }
    /* if curr.1 == map.len() - 1 {
        for f in [
            (curr.0, curr.1 - 1),
            (curr.0 - 1, curr.1), (curr.0 + 1, curr.1)
        ]
        .iter()
        {
            if f.0 == 0 || f.0 == dist_map[0].len()  {continue };
            if (map[curr.1].as_bytes()[curr.0] as i8 - map[f.1].as_bytes()[f.0] as i8).abs() < 1 {
                if path.contains(f) {
                    return None;
                }
                let mut p2 = path.clone();
                p2.push(*f);
                if let Some(path) = continuee(&p2, map, dist_map) {
                    return Some(path);
                }
            }
        }
    } else if curr.1 == 0 {
        for f in [
            (curr.0 - 1, curr.1), (curr.0 + 1, curr.1),
            (curr.0 , curr.1 + 1)
        ]
        .iter()
        {
            if f.0 == 0 || f.0 == dist_map[0].len()  {continue };
            if (map[curr.1].as_bytes()[curr.0] as i8 - map[f.1].as_bytes()[f.0] as i8).abs() < 1 {
                if path.contains(f) {
                    return None;
                }
                let mut p2 = path.clone();
                p2.push(*f);
                if let Some(path) = continuee(&p2, map, dist_map) {
                    return Some(path);
                }
            }
        }
        } else {*/
        for f in [
            (curr.0 as i32, (curr.1 as i32 - 1) as i32),
            ((curr.0 as i32 - 1) as i32, curr.1 as i32), ((curr.0 + 1) as i32, curr.1 as i32),
            (curr.0 as i32, (curr.1 + 1) as i32)
        ]
        .iter()
        {
            // println!("{:?}", f);
            if f.0 < 0 || f.0 >= dist_map[0].len() as i32  {continue };
            if f.1 < 0 || f.1 >= dist_map.len() as i32  {continue };
            let f: (usize, usize) = (f.0.try_into().unwrap(), f.1.try_into().unwrap());
            let xk = map[curr.1].as_bytes()[curr.0];
                //.as_bytes()[curr.0] as i8 -
            let yk = map[f.1].as_bytes()[f.0];
                //.as_bytes()[f.0] as i8;
            // dbg!(xk);
            // dbg!(yk);
            if map[f.1].as_bytes()[f.0] == 'E' as u8{
                    paths.push(path.clone());
                    // println!("{:?}", path);
            };
            if map[curr.1].as_bytes()[curr.0] as i8 - map[f.1].as_bytes()[f.0] as i8 <= 1 {
                // println!("f {:?} {:?}", path, f);
                if path.contains(&f) {
                    continue;
                }
                let mut p2 = path.clone();
                p2.push(f);
                if let Some(path) = continuee(&p2, map, dist_map, paths) {
                    return Some(path);
                }
            }
        }
    return None;
    //continue(path.clone().push(next[0]));

 } */
/*else if curr.0 == map[0].len()-1 || curr.0 == 0 {

} else {
    for f in
    [(curr.0, curr.1),
     (curr.0, curr.1), (curr.0, curr.1),
     (curr.0, curr.1), (curr.0, curr.1)
    ].iter()  {
        let mut p2 = path.clone();
        p2.push(*f);
        if let Some(path) = continuee(&p2, map, dist_map){
            return Some(path);
        }
    }
} */

use std::fmt::Debug;
fn print<T: Debug>(tp: &Vec<Vec<T>>) {
    for i in 0..tp.len() {
        for j in 0..tp[i].len() {
            print!("{:?} \t", tp[i][j]);
        }
        println!();
    }
}

/*
            if start.len() > 0 {
            for i in 0..start.len() {
                if start.as_bytes()[start.len()-1-i] as char >= ch {
                    right+=1;
                    break;
                }
                right += 1;
            }}
            if end.len() > 1 {
            for i in 0..end.len() {
                if end.as_bytes()[i] as char >= ch {
                    left+=1;
                    break;
                }
                left+=1;
            }}
            let (mut start, mut end) = (start.chars(), end.chars());
            if !start.any(|cfh| cfh >= ch) || !end.any(|cfh| cfh >= ch) {
                // total += 1;
                // println!("visible");
                side = true;
            }

            //visible from top
            let mut up = true;
            for x in 0..i {
               if input[i-1-x].as_bytes()[j] as char >= ch {
                    // println!(" t {:?} blocks", input[i-1-x].as_bytes()[j] as char);
                    top +=1;
                    up = false;
                    break;
                }
               top+=1;
            }
            //visible from top
            let mut down = true;
            for x in i+1..input.len() {
               if input[x].as_bytes()[j] as char >= ch {
                    // println!("b {:?} blocks", input[x].as_bytes()[j] as char);
                    downn +=1;
                    down = false;
                    break;
                }
               downn+=1;
            }
            // println!("{top}*{downn}*{right}*{left}");
            if top*downn*right*left > best_score {
                best_score = top*downn*right*left;
            }
            if up || down || side{
                total += 1;
            }

        }
    }
    println!("{total} {best_score}");
}
*/
