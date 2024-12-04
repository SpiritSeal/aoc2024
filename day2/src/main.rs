
// fn is_safe(report: Vec<i32>) -> bool {
//     report.windows(2).all(|x, y| y == x + 1 || y == x + 2 || y == x + 3)
// }

fn main() {
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
    ;

    let increasing_count = stage1
        .clone()
        .into_iter()
        .filter(|report| {
            report
                .into_iter()
                .all(|val| *val > 0 && *val <= 3)
        })
        .count()  // Count the number of increasing sequences
        // .collect::<Vec<Vec<i32>>>()
    ;

    let decreasing_count = stage1
        .clone()
        .into_iter()
        .filter(|report| {
            report
                .into_iter()
                .all(|val| *val < 0 && *val >= -3)
        })
        .count()  // Count the number of increasing sequences
        // .collect::<Vec<Vec<i32>>>()
    ;

    let part_1_answer = increasing_count + decreasing_count;
    println!("Part 1 Answer: {}", part_1_answer)

}


