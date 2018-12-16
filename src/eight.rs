pub fn solve(s: &String) {
    let trimmed = &s.trim().to_string();
    println!("Started with {}", trimmed.len());

    let part_one_solution = solve_part_one(trimmed);
    println!("Solutions are: {}", part_one_solution)
}

fn solve_part_one(s: &String) -> usize {
    let metadata = get_metadata(&mut s.split(' ').map(|s| str::parse(s).unwrap()));

    metadata.iter().fold(0, |a, b| a + *b)
}

fn get_metadata(data: &mut Iterator<Item = usize>) -> Vec<usize> {
    let (child_count, metadata_count) = read_header(data);

    let mut expected_headers = vec![child_count];
    let mut expected_metadata = vec![metadata_count];
    let mut metadata = vec![];

    loop {
        let headers_to_go = expected_headers.pop().unwrap_or(0);

        if headers_to_go == 0 {
            let metadata_to_take = expected_metadata.pop();
            if metadata_to_take.is_none() {
                break;
            }

            metadata.append(&mut take_metadata(data, metadata_to_take.unwrap()));
        } else {
            let (child_count, metadata_count) = read_header(data);
            expected_headers.push(headers_to_go - 1);
            expected_headers.push(child_count);
            expected_metadata.push(metadata_count);
        }
    }

    metadata
}

fn take_metadata(data: &mut Iterator<Item = usize>, count: usize) -> Vec<usize> {
    return data.take(count).collect();
}

fn read_header(data: &mut Iterator<Item = usize>) -> (usize, usize) {
    let children_count = data.next().unwrap();
    let metadata_count = data.next().unwrap();

    return (children_count, metadata_count);
}

#[test]
fn test_solve_part_one() {
    assert_eq!(1, solve_part_one(&"0 1 1".to_string()));
    assert_eq!(6, solve_part_one(&"0 2 1 5".to_string()));
    assert_eq!(
        138,
        solve_part_one(&"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_string())
    );
}
