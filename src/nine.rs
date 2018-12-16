pub fn solve(s: &String) {
    let trimmed = &s.trim().to_string();
    println!("Started with {}", trimmed.len());

    let part_one_solution = solve_part_one(trimmed);
    println!("Solutions are: {}", part_one_solution)
}

fn solve_part_one(s: &String) -> usize {
    let split: Vec<&str> = s.split(" ").collect();

    let players: usize = split.get(0).unwrap().parse().unwrap();
    let last_marble: usize = split.get(6).unwrap().parse().unwrap();

    let mut circle = vec![0];
    let mut scores = vec![0; players];
    let mut current_player = 0;
    let mut current_marble_idx: usize = 0;

    for marble in 1..last_marble {
        let circle_size = circle.len();
        if marble % 23 == 0 {
            scores[current_player] += marble;
            let also_remove_offset: isize = current_marble_idx as isize - 7;
            let also_remove_idx = if also_remove_offset.ge(&0) {
                also_remove_offset as usize
            } else {
                (circle_size as isize + also_remove_offset) as usize
            };
            scores[current_player] += circle.remove(also_remove_idx);
            current_marble_idx = also_remove_idx;
        } else {
            let insert_idx = (current_marble_idx + 1) % circle_size + 1;
            circle.insert(insert_idx, marble);
            current_marble_idx = insert_idx;
        }

        current_player = (current_player + 1) % scores.len();

        //        println!("{}, {:?}", current_player, circle)
    }

    scores.iter().fold(0, |a, b| a.max(*b))
}

#[test]
fn test_solve_part_one() {
    assert_eq!(
        32,
        solve_part_one(&"9 players; last marble is worth 25 points".to_string())
    );
    assert_eq!(
        8317,
        solve_part_one(&"10 players; last marble is worth 1618 points".to_string())
    );
    assert_eq!(
        146373,
        solve_part_one(&"13 players; last marble is worth 7999 points".to_string())
    );
    assert_eq!(
        54718,
        solve_part_one(&"21 players; last marble is worth 6111 points".to_string())
    );
    assert_eq!(
        37305,
        solve_part_one(&"30 players; last marble is worth 5807 points".to_string())
    );
}
