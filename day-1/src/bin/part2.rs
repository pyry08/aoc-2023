use std::collections::HashMap;

fn main() {
    println!("{}", part2(include_str!("input.txt")));
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;

    let spelling_digit_mapping = HashMap::from([
        ("one".to_owned(), '1'),
        ("two".to_owned(), '2'),
        ("three".to_owned(), '3'),
        ("four".to_owned(), '4'),
        ("five".to_owned(), '5'),
        ("six".to_owned(), '6'),
        ("seven".to_owned(), '7'),
        ("eight".to_owned(), '8'),
        ("nine".to_owned(), '9'),
    ]);

    for line in input.lines() {
        let mut first_digit = '\0';
        let mut first_digit_spelled = String::new();

        'first_digit_loop: for char in line.trim().chars() {
            if char.is_ascii_digit() {
                first_digit = char;
                break;
            }

            first_digit_spelled.push(char);

            for key in spelling_digit_mapping.keys() {
                if first_digit_spelled.contains(key) {
                    first_digit = spelling_digit_mapping[key];
                    first_digit_spelled.clear();
                    break 'first_digit_loop;
                }
            }
        }

        let mut last_digit = '\0';
        let mut last_digit_spelled = String::new();

        'last_digit_loop: for char in line.chars().rev() {
            if char.is_ascii_digit() {
                last_digit = char;
                break;
            }

            last_digit_spelled.push(char);

            for key in spelling_digit_mapping.keys() {
                if last_digit_spelled.contains(key.chars().rev().collect::<String>().as_str()) {
                    last_digit = spelling_digit_mapping[key];
                    last_digit_spelled.clear();
                    break 'last_digit_loop;
                }
            }
        }

        sum += vec![first_digit, last_digit]
            .into_iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2_works() {
        assert_eq!(
            super::part2(
                "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
            ),
            281
        )
    }
}
