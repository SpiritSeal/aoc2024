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


    let pairs:Vec<(i32, i32)> = zip(v1, v2).collect();
    // dbg!(&z);

    let output: u32 = pairs.into_iter()
        .filter_map(|x| Some((x.0 - x.1).abs()))
        .fold(0u32, |sum, i| sum + (i as u32));

    println!("
    Answer: {}", output);


}
