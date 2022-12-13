#[derive(PartialEq, Debug)]
enum res {
    yes,
    no,
    idk
}

fn main() {
    let input = include_str!("../input.prod");
    
    let signals = input.split("\n\n");
    let mut allsig: Vec<&str> = Vec::new();
    let sum: usize =signals.enumerate().map(|(i, sig)| {
        let (first, second) = sig.split_once('\n').unwrap();
        allsig.push(first.clone());
        allsig.push(second.clone());
        if in_order(first.to_string(), second.to_string()) != res::no {    println!("made it"); i +1} else { 0 }

    }).sum();


    allsig.push("[[2]]");
    allsig.push("[[6]]");

    for i in 0..allsig.len() {
        for j in 1..allsig.len()-i {
            if in_order(allsig[j-1].to_string(), allsig[j].to_string()) == res::no{
                let tmpsig = allsig[j];
                allsig[j] = allsig[j-1];
                allsig[j-1] = tmpsig;
            }
        }
    }

    println!("{sum} {:?}", allsig.iter().position(|f| f==&"[[6]]" ));
    println!("{sum} {:?}", allsig.iter().position(|f| f==&"[[2]]" ));
    // println!("{sum} {:?}", allsig);
}

fn in_order(first: String, second: String) -> res {
    
    if first.is_empty() && second.is_empty() {
        return res::idk;
    } else if first.is_empty() {
        return res::yes;
    } else if second.is_empty() {
        return res::no;
    }
   // println!("{:?} {:?}", first, second);
    let firsst  = parse(first.clone());
    let seccond  = parse(second.clone());
    for (i, d) in firsst.iter().enumerate() {
        if i == seccond.len() {
                return res::no;
        }
        let f = match (firsst[i].clone().parse::<usize>(), seccond[i].clone().parse::<usize>()) {
            (Ok(x), Ok(y)) => { if x > y { res::no } else if y> x {  res::yes } else { res::idk } },
            (Err(_)  , Ok(y)) => in_order(firsst[i].clone(), format!("[{}]",seccond[i].clone())),
            (Ok(x), Err(_)) =>  in_order(format!("[{}]", firsst[i].clone()), seccond[i].clone()),
            (Err(_), Err(_))  => in_order(firsst[i].clone(), seccond[i].clone()) 
        };
        // println!("ppp {:?} {} {}",f, firsst[i], seccond[i]);
        if f == res::no {
            return f;
        } else if f == res::yes {
            return f;
        }
    }
    if seccond.len() > firsst.len() {
        return res::yes;
    }
    res::idk
}

fn parse(elem: String) -> Vec<String>{
    let mut counter = 0;
    let mut vec = Vec::new();
    let cha = elem.as_bytes().clone();
    let mut last = 1;
    let mut pusing = true;
    for i in 0..cha.len() {
        if cha[i] as char == '[' {
            counter += 1;
        } else if cha[i] as char == ']' {
            counter -= 1;
        } 
        if (cha[i] as char == ',' && counter == 1 ) || i == cha.len()-1{
            if pusing || i == cha.len()-1{
                vec.push(elem[last..i].to_string());
                last = i+1;
            } else {
                last = i+1;
                pusing = true;
            }
        }
    }
    vec
}
