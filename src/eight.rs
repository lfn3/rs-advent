pub fn solve(s: &String) {
    let trimmed = &s.trim().to_string();
    println!("Started with {}", trimmed.len());

    let part_one_solution = solve_part_one(trimmed);
    println!("Solutions are: {}", part_one_solution)
}

fn solve_part_one(s: &String) -> usize {
    let metadata = get_metadata(&mut s.split(' ').map(|s| { str::parse(s).unwrap() }));

    metadata.iter().fold(0, |a, b| { a + *b })
}

fn get_metadata(data: &mut Iterator<Item = usize>) -> Vec<usize> {
    let (child_count, metadata_count) = read_header(data);

    if child_count == 0 {
        return take_metadata(data, metadata_count);
    } else {
        let mut metadata = vec![];
        for _i in 0..child_count {
            metadata.append(&mut get_metadata(data));
        }
        metadata.append(&mut take_metadata(data, metadata_count));
        return metadata;
    }
}

fn take_metadata(data: &mut Iterator<Item = usize>, count: usize) -> Vec<usize> {
    return data.take(count).collect();
}

fn read_header(data: &mut Iterator<Item=usize>) -> (usize, usize) {
    let children_count = data.next().unwrap();
    let metadata_count = data.next().unwrap();

    return (children_count, metadata_count);
}

#[test]
fn test_solve_part_one() {
    assert_eq!(1, solve_part_one(&"0 1 1".to_string()));
    assert_eq!(6, solve_part_one(&"0 2 1 5".to_string()));
    assert_eq!(138, solve_part_one(&"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_string()));
}