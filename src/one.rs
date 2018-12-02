use std::collections::HashSet;

pub fn solve(s: &String)
{
    let (s1, s2) = testable_solve(s);
    println!("Solutions are: {}, {}", s1, s2)
}

fn testable_solve(s: &String) -> (i32, i32)
{
    (solve_part_one(s), solve_part_two(s))
}

#[test]
fn should_solve_sample_input() {
    assert_eq!((0, 0), testable_solve(&"+1\n-1\n".to_string()));
    assert_eq!((4, 10), testable_solve(&"+3\n+3\n+4\n-2\n-4\n".to_string()));
    assert_eq!((4, 5), testable_solve(&"-6\n+3\n+8\n+5\n-6\n".to_string()));
    assert_eq!((1, 14), testable_solve(&"+7\n+7\n-2\n-7\n-4\n".to_string()));
    assert_eq!((3, 2), testable_solve(&"+1\n-2\n+3\n+1\n".to_string()));
}

fn solve_part_one(s: &String) -> i32
{
    s.lines()
        .map(|l| { l.parse().unwrap() })
        .fold(0, |acc, change: i32| acc + change)
}

#[test]
fn should_solve_part_one_sample_input() {
    assert_eq!(0, solve_part_one(&"+1\n-1\n".to_string()));
    assert_eq!(4, solve_part_one(&"+3\n+3\n+4\n-2\n-4\n".to_string()));
    assert_eq!(4, solve_part_one(&"-6\n+3\n+8\n+5\n-6\n".to_string()));
    assert_eq!(1, solve_part_one(&"+7\n+7\n-2\n-7\n-4\n".to_string()));
    assert_eq!(3, solve_part_one(&"+1\n-2\n+3\n+1\n".to_string()));
}

fn solve_part_two(s: &String) -> i32 {
    let mut initial_set = HashSet::new();
    initial_set.insert(0);

    s.lines()
        .map(|l| { l.parse().unwrap() })
        .cycle()
        .scan((0, initial_set),
              |(acc, seen), change: i32|
                  {
                      *acc += change;

                      if seen.contains(acc) {
                          return Some(Some(acc.clone()))
                      }

                      seen.insert(acc.clone());

                      return Some(None); //This is weird, but scan stops as soon as you return None
                  })
        .find(|v| v.is_some())
        .unwrap()
        .unwrap()
}

#[test]
fn should_solve_part_two_sample_input() {
    assert_eq!(0, solve_part_two(&"+1\n-1\n".to_string()));
    assert_eq!(10, solve_part_two(&"+3\n+3\n+4\n-2\n-4\n".to_string()));
    assert_eq!(5, solve_part_two(&"-6\n+3\n+8\n+5\n-6\n".to_string()));
    assert_eq!(14, solve_part_two(&"+7\n+7\n-2\n-7\n-4\n".to_string()));
    assert_eq!(2, solve_part_two(&"+1\n-2\n+3\n+1\n".to_string()));
}
