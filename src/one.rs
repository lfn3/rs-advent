use std::collections::HashSet;

pub fn solve(s: &String)
{
    let (s1, s2) = testable_solve(s);
    println!("Solutions are: {}, {}", s1, s2)
}

fn testable_solve(s: &String) -> (i32, i32)
{
    let mut seen = HashSet::new();
    seen.insert(0);

    let mut current_freq = 0;
    let mut solution_one = 0;

    for i in 0.. {
        let mut found = false;
//        println!("Running over {} lines", s.lines().count());

        for line in s.lines() {
            let change: i32 = line.parse().unwrap();

//            println!("Current frequency {}, change of {}; resulting frequency {}", current_freq, change, current_freq + change);

            current_freq += change;

            if seen.contains(&current_freq) {
                found = true;
                break;
            }

            seen.insert(current_freq.clone());
        }
//        println!("=== REPEATING ===");

        if i == 0 {
            solution_one = current_freq.clone();
        }

        if found {
            break;
        }
    }

    (solution_one, current_freq)
}

#[test]
fn should_solve_sample_input() {
    assert_eq!((0, 0), testable_solve(&"+1\n-1\n".to_string()));
    assert_eq!((4, 10), testable_solve(&"+3\n+3\n+4\n-2\n-4\n".to_string()));
    assert_eq!((4, 5), testable_solve(&"-6\n+3\n+8\n+5\n-6\n".to_string()));
    assert_eq!((1, 14), testable_solve(&"+7\n+7\n-2\n-7\n-4\n".to_string()));
    assert_eq!((3, 2), testable_solve(&"+1\n-2\n+3\n+1\n".to_string()));
}