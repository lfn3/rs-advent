use std::collections::HashSet;

pub fn solve(s: &String)
{
    let mut seen = HashSet::new();
    seen.insert(0);

    let mut current_freq = 0;

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
            println!("Solution to part 1 is: {}", current_freq)
        }

        if found {
            break;
        }
    }

    println!("Solution to part 2 is: {}", current_freq)
}
