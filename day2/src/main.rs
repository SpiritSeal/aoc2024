

fn part_1(input: &str) {
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



fn part_2(input: &str) {
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
            let valid_check = report
                .into_iter()
                // .all(|val| *val > 0 && *val <= 3)
                .fold(1 as i32,
                    |running, difference| {
                        running - (if !(*difference > 0 && *difference <= 3) {1} else {0})
                    }
                );
            dbg!(report);
            dbg!(valid_check);
            if valid_check >= 0 {true} else {false}
        })
        // .count()  // Count the number of increasing sequences
        .collect::<Vec<Vec<i32>>>()
    ;

    let decreasing_count = stage1
        .clone()
        .into_iter()
        .filter(|report| {
            let valid_check = report
                .into_iter()
                // .all(|val| *val > 0 && *val <= 3)
                .fold(1 as i32,
                    |running, difference| {
                        running - (if !(*difference < 0 && *difference >= -3) {1} else {0})
                    }
                );
            if valid_check >= 0 {true} else {false}
        })
        // .count()  // Count the number of increasing sequences
        .collect::<Vec<Vec<i32>>>()
    ;


    dbg!(&increasing_count);
    dbg!(&decreasing_count);

    // let part_2_answer = increasing_count + decreasing_count;
    // println!("Part 2 Answer: {}", part_2_answer)
}


fn main() {
    // let input = include_str!("../input.txt");
    let input = include_str!("../test.txt");

    part_1(input);
    part_2(input);



}


