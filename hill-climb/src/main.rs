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

use std::cell::RefCell;
use std::cmp::max;

#[derive(PartialEq, Clone, Debug)]
struct Position(usize, usize);

#[derive(PartialEq, Clone, Debug)]
struct Node {
    position: Position,
    parent: Position,
    visited: RefCell<bool>,
    value: usize,
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
    // println!("{}, {}", start.0, end.0);
    let mut input = inputt.clone();
    let inpuut = &inputt[start.1].replace("S", "a");
    input[start.1] = &inpuut;
    let inpuuut = &input[end.1].replace("E", "z");
    input[end.1] = &inpuuut;

    println!("{:?}", input);

    for (i, inp) in lines.clone() {
        for (j, ch) in inp.chars().enumerate() {
            dist_map[i][j] = (j as i32 - end.0 as i32) * (j as i32 - end.0 as i32)
                + (i as i32 - end.1 as i32) * (i as i32 - end.1 as i32);
        }
    }


    let mut min = 100000;
    for (i, inp) in lines {
        'm: for (j, ch) in inp.chars().enumerate() {
            if ch == 'a' {
                println!("finding {ch}");
                if let Some(pn) = find_path(&(j, i), &end, &input, &dist_map, 400) {
                if pn.len() < min {
                    min = pn.len();
                    println!("current best {min}");
                }
                }
            }
        }
    } 
    // print(&dist_map);
    // let n = find_path(&start, &end, &input, &dist_map, 10000);
    println!("{min}");
}

fn find_path(
    start: &(usize, usize),
    end: &(usize, usize),
    input: &Vec<&str>,
    dist_map: &Vec<Vec<i32>>,
    max_steps: usize
) -> Option<Vec<Position>> {
    let mut path = Vec::new();
    let mut stack: Vec<Node> = Vec::new();
    let mut found = false;
    let map = input;
    let mut steps = 0;
    let start_node = Node {
        position: Position(start.0, start.1),
        parent: Position(start.0, start.1),
        visited: RefCell::new(false),
        value: dist(&start, &start, &end),
    };
    stack.push(start_node);
    while !found || steps < max_steps {
        // println!("{steps}");
        steps += 1;
        if steps > 1000 {
            return None;
        }
        stack.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap());
        for (i, node) in stack.clone().iter().enumerate() {
            let nodee = &mut stack[i];
            if !(*nodee.visited.borrow()) {
                *nodee.visited.borrow_mut() = true;
                let curr = &node.position;
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
                    let dis = dist(&f, &start, &end);
                    // println!("checking {:?} {:?} {dis} {:?}", f, node.position, end);
                    let xk = map[curr.1].as_bytes()[curr.0];
                    let yk = map[f.1].as_bytes()[f.0];
                    // println!("{} {}", xk as char, yk as char);
                    if (map[curr.1].as_bytes()[curr.0] as i8 - map[f.1].as_bytes()[f.0] as i8) >= -1
                    {
                        // println!("{:?}", dis);
                        let nodee = Node {
                            position: Position(f.0, f.1),
                            value: node.value + 1 + dis,
                            parent: node.position.clone(),
                            visited: RefCell::new(false),
                        };
                        if dis == 0 {
                            /* println!(
                                "YAYY {:?}",
                                stack
                                    .iter()
                                    .map(|t| t.position.clone())
                                    .collect::<Vec<Position>>()
                            ); */
                            found = true;
                            unstack(&stack, &nodee, &start, &mut path);
                            return Some(path);
                        }
                        check_insert(&mut stack, nodee);
                    }
                }
            }
        }
    }
    return None;
}

fn check_insert(stack: &mut Vec<Node>, nod: Node) {
    match stack.iter().position(|H| H.position == nod.position) {
        Some(x) => {
            // println!("Already existed just updatin'");
            if stack[x].value > nod.value {
                stack[x].parent = nod.parent.clone();
                stack[x].value = nod.value;
            }
        }
        None => {
            // println!("pushed a new copy");
            stack.push(nod)
        }
    }
}

fn unstack(stack: &Vec<Node>, node: &Node, start: &(usize, usize), path: &mut Vec<Position>) {
    // println!("{:?}", node.position);
    path.push(node.position.clone());
    if node.position.0 == start.0 && node.position.1 == start.1 {
        return;
    }
    if let Some(nod) = stack.iter().position(|H| H.position == node.parent) {
        unstack(stack, &stack[nod].clone(), start, path);
    }
}

fn dist(curr: &(usize, usize), start: &(usize, usize), endd: &(usize, usize)) -> usize {
    let (i, j) = (curr.0, curr.1);
    let end = start;
    let dstart = (j as i32 - end.0 as i32) * (j as i32 - end.0 as i32)
        + (i as i32 - end.1 as i32) * (i as i32 - end.1 as i32);

    // let (i ,j) = curr;
    let end = endd;
    // println!("{:?} {i} {j}", end);
    let dend = (i as i32 - end.0 as i32) * (i as i32 - end.0 as i32)
        + (j as i32 - end.1 as i32) * (j as i32 - end.1 as i32);

    (dend).try_into().unwrap()
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
