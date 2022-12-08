use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct Dir {
    folders:Vec<String>,
    size: u32
}

fn main() {
    let mut input = include_str!("../input.prod").lines();
    let mut map: HashMap<String, Dir> = HashMap::new();
    let mut path = "".to_string();
    let mut dir = Dir { folders: Vec::new(), size:0 }; 

    while let Some(inp) = input.next() {
        if inp.starts_with("$") {
            let commands: Vec<&str> = inp.split(" ").collect();//.unwrap();
            let command = commands[1];
            match command {
                "cd" => { add_dir(&mut path, commands[2].to_string()); },
                "ls" => {},
                _ => {},
            }
        } else {
            if inp.starts_with("dir") {
               let file = inp.split_once("dir").unwrap().1.trim();
               let pathf = add_dirr(&mut path, file);
               if let Some(Dir) = map.get_mut(&path) {
                    Dir.folders.push(pathf.clone());
               } else {
                    map.insert(path.clone(), Dir { folders: vec![pathf], size:0 });
               }
            } else {
                let ssize:u32 = inp.split_once(" ").unwrap().0.trim().parse().unwrap();
                if let Some(Dir) = map.get_mut(&path) {
                    Dir.size = Dir.size+ssize;
               } else {
                    map.insert(path.clone(), Dir { folders: Vec::new(), size: ssize });
               }               
            }
        }
    }

    //condensing it ig
    let mut map2 = map.clone();
    let rmap = RefCell::new(map);
    let mut sum = 0;
    rmap.borrow().keys().for_each(|f| {
        /* map[f].folders.iter().for_each(|dir| {
            mapp[f].size += map[dir].size;
        }); */
        count_add(&rmap, f.to_string(), &mut map2);

    });
    map2.keys().for_each(|f|{
        let val = map2[f].size;
        if val <= 100000 {
            sum += val;
        }
    });

    let used = map2.get("~").unwrap().size;
    let free = 70000000 - used;
    let tofree = 30000000 - free;
    //map2.values().sort(|f| if f>4125990 {f-4125990} else {0});
    let mut min = u32::MAX;
    map2.values().for_each(|v| {
        if v.size>tofree{//4125990  {
            if v.size-tofree < min {
                min=v.size;
            }
        }
    });
    println!("{:?} {sum} {min} ", map2);
}

fn count_add(map: &RefCell<HashMap<String, Dir>>, file: String, map2: &mut HashMap<String, Dir>) -> u32 {
        let mut sum = map.borrow()[&file].size;
        map.borrow()[&file].folders.iter().for_each(|f| {
            count_add(map, f.clone(), map2);
            sum += map2[&*f].size;
        });
        //map.borrow_mut()[&file].size = sum;
        if let Some(x) = map2.get_mut(&file) {
            x.size = sum;
        }
        sum
}

fn add_dir(path: &mut String, file: String) {
    if file == "/" {
        *path = "~".to_string();
    } else if file == ".." {
        *path = path.rsplit_once("/").unwrap().0.to_string();
    } else {
        *path = path.to_owned()  + "/" + &file;
    }
}


fn add_dirr(path: &mut String, file: &str) -> String {
    if file == "/" {
        "~".to_string()

    } else if file == ".." {
        path.rsplit_once("/").unwrap().0.to_string()
    } else {
        path.to_owned()  + "/" + file
    }
}
