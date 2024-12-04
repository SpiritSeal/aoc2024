
// fn is_safe(report: Vec<i32>) -> bool {
//     report.windows(2).all(|x, y| y == x + 1 || y == x + 2 || y == x + 3)
// }

fn main() {
    println!("Hello, world!");

    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    // Can I do it functionally?
    let stage1 = input
        .lines()
        .into_iter()
        .map(|line_val|
            line_val
            .split_whitespace()
            .map(|y| y.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .windows(2)
                // .fold(0, |x, y| x - y)
            .map(|x| x[0] - x[1])
            .collect::<Vec<i32>>()
        )
        .collect::<Vec<Vec<i32>>>()
        // .windows(2)
    ;

    // dbg!(&stage1);

    let increasing_count = stage1
        // .collect::<Vec<Vec<i32>>>()  // Collect into Vec<Vec<i32>> first
        .clone()
        .into_iter()
        .filter(|report| {
            // // Compare each pair of vectors
            // let current = &report[0];
            // let next = &report[1];
            
            // // Check if every element in next is greater than corresponding element in current
            // current.len() == next.len() && 
            // current.iter()
            //     .zip(next.iter())
            //     .all(|(curr, nxt)| (nxt > curr) && (*nxt <= *curr + 3))
            report
                .into_iter()
                .all(|val| *val > 0 && *val <= 3)
        })
        .count()  // Count the number of increasing sequences
        // .collect::<Vec<Vec<i32>>>()
    ;

    let decreasing_count = stage1
        .clone()
        // .collect::<Vec<Vec<i32>>>()  // Collect into Vec<Vec<i32>> first
        .into_iter()
        .filter(|report| {
            // // Compare each pair of vectors
            // let current = &report[0];
            // let next = &report[1];
            
            // // Check if every element in next is greater than corresponding element in current
            // current.len() == next.len() && 
            // current.iter()
            //     .zip(next.iter())
            //     .all(|(curr, nxt)| (nxt > curr) && (*nxt <= *curr + 3))
            report
                .into_iter()
                .all(|val| *val < 0 && *val >= -3)
        })
        .count()  // Count the number of increasing sequences
        // .collect::<Vec<Vec<i32>>>()
    ;


    // .collect::<Vec<Vec<u32>>>()
    // dbg!(stage1.clone());
    // dbg!(increasing_count);
    // dbg!(decreasing_count);
    let answer = increasing_count + decreasing_count;
    dbg!(answer);
    // dbg!(&output.clone());

}


