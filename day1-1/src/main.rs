use std::io::prelude::*;
use std::iter::zip;

fn main() {
    //dan -- include_bytes! removes a lot of boilerplate for FS. Includes the bytes into the binary at comp time.
    let s = include_bytes!("../input.txt");

    // create vectors from each of the columns
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    // dan -- can only collect into owned type, shouldn't make a different
    let l1: Vec<String> = s.lines().map(|line| line.unwrap()).collect();

    for line_val in l1 {
        let x: Vec<i32> = line_val
            .split_whitespace()
            //dan -- unwrap, filter_mapping on error can cause a real headache
            .map(|x| x.parse().unwrap())
            .collect();

        v1.push(x[0]);
        v2.push(x[1]);
    }

    v1.sort();
    v2.sort();

    // both are 1000 elements btw
    // dbg!(&v1.len());
    // dbg!(&v2.len());

    let pairs: Vec<(i32, i32)> = zip(v1, v2).collect();
    // dbg!(&z);

    let output: u32 = pairs
        .into_iter()
        //dan -- I think this was overlooked, just gonna fix
        .map(|x| (x.0 - x.1).abs())
        .fold(0u32, |sum, i| sum + (i as u32));

    println!(
        "
    Answer: {}",
        output
    );
}
