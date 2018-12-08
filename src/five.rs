use core::borrow::BorrowMut;

pub fn solve(s: &String) {
    println!("Started with {}", s.trim().len());
    println!("Solutions are: {}", testable_solve(&s.trim().to_string()))
}

pub fn testable_solve(s: &String) -> usize {
    reduce_multiple(&mut s.chars().into_iter()).len()
}

#[test]
fn test_testable_solve()
{
    assert_eq!(10, testable_solve(&"dabAcCaCBAcCcaDA".to_string()));
    assert_eq!(0, testable_solve(&"abBA".to_string()));
    assert_eq!(4, testable_solve(&"abAB".to_string()));
    assert_eq!(6, testable_solve(&"aabAAB".to_string()));
}

fn reduce_multiple(i : &mut Iterator<Item = char>) -> String {
    i.fold(String::new(), |acc, c| {
        if acc.ends_with(flip(c)) {
            acc.get(0..(acc.len() - 1)).unwrap().to_string()
        } else {
            format!("{}{}", acc, c)
        }
    })
}

#[test]
fn test_reduce_multiple()
{
    assert_eq!("dabCBAcaDA".to_string(),
               reduce_multiple("dabAcCaCBAcCcaDA".chars().into_iter().borrow_mut()));
    assert_eq!("".to_string(),
               reduce_multiple("aABb".chars().into_iter().borrow_mut()));
    assert_eq!("A".to_string(),
               reduce_multiple("XxSsdDIijNnJACszZScZfFhHQyYrRqz".chars().into_iter().borrow_mut()));
}

fn flip(c : char) -> char
{
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