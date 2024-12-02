use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let path = Path::new("./input.txt");
    let display = path.display();
    
    // println!("{}", display);

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

    // let l1: Vec<&str> = s.split("\r\n").collect().split(" ").collect();
    let l1: Vec<&str> = s.lines().collect();
    // println!("{:?}", l1);

    for line_val in l1 {
        // dbg!(line_val);
        let x:Vec<i32> = line_val.split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        // dbg!(&x);
        
        v1.push(x[0]);
        v2.push(x[1]);
        
        // println!();
    }

    v1.sort();
    v2.sort();

    // both are 1000 elements
    dbg!(&v1.len());
    dbg!(&v2.len());


    let pairs:Vec<(i32, i32)> = zip(v1, v2).collect();
    // dbg!(&z);

    let output: u32 = pairs.into_iter()
        .filter_map(|x| Some((x.0 - x.1).abs()))
        .fold(0u32, |sum, i| sum + (i as u32));

    println!("Output: {}", output);


}
