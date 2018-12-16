use core::borrow::BorrowMut;

pub fn solve(s: &String) {
    let trimmed = &s.trim().to_string();
    println!("Started with {}", trimmed.len());

    let part_one_solution = solve_part_one(trimmed);
    let part_two_solution = reduce_without_all(part_one_solution.clone());
    println!(
        "Solutions are: {}, {}",
        part_one_solution.len(),
        part_two_solution
    )
}

fn solve_part_one(s: &String) -> String {
    reduce(&mut s.chars().into_iter())
}

fn reduce(i: &mut Iterator<Item = char>) -> String {
    i.fold(String::new(), |acc, c| {
        if acc.ends_with(flip(c)) {
            acc.get(0..(acc.len() - 1)).unwrap().to_string()
        } else {
            format!("{}{}", acc, c)
        }
    })
}

#[test]
fn test_solve_part_one() {
    assert_eq!(
        "dabCBAcaDA".to_string(),
        solve_part_one(&"dabAcCaCBAcCcaDA".to_string())
    );
    assert_eq!("".to_string(), solve_part_one(&"aABb".to_string()));
    assert_eq!(
        "A".to_string(),
        solve_part_one(&"XxSsdDIijNnJACszZScZfFhHQyYrRqz".to_string())
    );
    assert_eq!("".to_string(), solve_part_one(&"abBA".to_string()));
    assert_eq!("abAB".to_string(), solve_part_one(&"abAB".to_string()));
}

fn reduce_without_all(s: String) -> usize {
    let mut best = s.len();

    for p in 'a' as u8..'z' as u8 {
        best =
            best.min(reduce_without(p as char, s.clone().chars().into_iter().borrow_mut()).len());
    }

    best
}

#[test]
fn test_reduce_without_all() {
    assert_eq!(4, reduce_without_all("dabAcCaCBAcCcaDA".to_string()));
}

fn reduce_without(polymer: char, i: &mut Iterator<Item = char>) -> String {
    i.fold(String::new(), |acc, c| {
        if c == polymer || c == flip(polymer) {
            acc
        } else if acc.ends_with(flip(c)) {
            acc.get(0..(acc.len() - 1)).unwrap().to_string()
        } else {
            format!("{}{}", acc, c)
        }
    })
}

#[test]
fn test_reduce_without() {
    assert_eq!(
        "dbCBcD".to_string(),
        reduce_without('a', "dabAcCaCBAcCcaDA".chars().into_iter().borrow_mut())
    );
    assert_eq!(
        "daCAcaDA".to_string(),
        reduce_without('b', "dabAcCaCBAcCcaDA".chars().into_iter().borrow_mut())
    );
    assert_eq!(
        "daDA".to_string(),
        reduce_without('c', "dabAcCaCBAcCcaDA".chars().into_iter().borrow_mut())
    );
    assert_eq!(
        "abCBAc".to_string(),
        reduce_without('d', "dabAcCaCBAcCcaDA".chars().into_iter().borrow_mut())
    );
}

fn flip(c: char) -> char {
    if c.is_lowercase() {
        c.to_ascii_uppercase()
    } else {
        c.to_ascii_lowercase()
    }
}

#[test]
fn test_flip() {
    assert_eq!('a', flip('A'));
    assert_eq!('A', flip('a'));
}
