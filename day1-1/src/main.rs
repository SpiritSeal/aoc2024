use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;
use std::path::Path;

fn main() {
    let path = Path::new("./input.txt");
    let display = path.display();
    

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read bc {}", why),
        Ok(_) => (),
    };

    // create vectors from each of the columns
    let mut v1:Vec<i32> = Vec::new();
    let mut v2:Vec<i32> = Vec::new();

    let l1: Vec<&str> = s.lines().collect();

    for line_val in l1 {
        let x:Vec<i32> = line_val.split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        
        v1.push(x[0]);
        v2.push(x[1]);
    }

    v1.sort();
    v2.sort();

    // both are 1000 elements btw
    // dbg!(&v1.len());
    // dbg!(&v2.len());


    // PART 1

    let pairs:Vec<(i32, i32)> = zip(v1.clone(), v2.clone()).collect();
    // dbg!(&z);

    let output1: u32 = pairs.into_iter()
        .filter_map(|x| Some((x.0 - x.1).abs()))
        .fold(0u32, |sum, i| sum + (i as u32));

    println!("Part 1 Answer: {}", output1);



    // PART 2
    let freq_val: HashMap<i32, u32> = v2.clone().into_iter().fold(
        HashMap::new(),
        |mut map, val| {
            map.entry(val)
                .and_modify(|f| *f+=1)
                .or_insert(1);
            map
        }
    );

    let output2: u32 = v1.clone().into_iter().fold(
        0,
        |accum, val| accum + val as u32 * (if freq_val.contains_key(&val) {*freq_val.get(&val).unwrap()} else {0})
    );

    println!("Part 2 Answer: {}", output2);



}
