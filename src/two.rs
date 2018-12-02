use std::collections::HashMap;

pub fn solve(s: &String) {
    println!("Solutions are: {}", testable_solve(s))
}

pub fn testable_solve(s: &String) -> usize {
    let (twos, threes) = s.lines().fold((0, 0), |(twos, threes), line| {
        (
            if has_n(line, 2) { twos + 1 } else { twos },
            if has_n(line, 3) { threes + 1 } else { threes },
        )
    });

    twos * threes
}

#[test]
fn test_testable_solve() {
    assert_eq!(
        12,
        testable_solve(&"abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n".to_string())
    )
}

fn has_n(s: &str, n: usize) -> bool {
    let mut letters = HashMap::new();
    for c in s.chars() {
        let count = letters.entry(c).or_insert(0);
        *count += 1;
    }

    letters.values().into_iter().any(|count| *count == n)
}

#[test]
fn test_has_n() {
    assert_eq!(false, has_n("abcdef", 2));
    assert_eq!(true, has_n("bababc", 2));
    assert_eq!(true, has_n("abbcde", 2));
    assert_eq!(false, has_n("abcccd", 2));
    assert_eq!(true, has_n("aabcdd", 2));
    assert_eq!(true, has_n("abcdee", 2));
    assert_eq!(false, has_n("ababab", 2));

    assert_eq!(false, has_n("abcdef", 3));
    assert_eq!(true, has_n("bababc", 3));
    assert_eq!(false, has_n("abbcde", 3));
    assert_eq!(true, has_n("abcccd", 3));
    assert_eq!(false, has_n("aabcdd", 3));
    assert_eq!(false, has_n("abcdee", 3));
    assert_eq!(true, has_n("ababab", 3));
}
