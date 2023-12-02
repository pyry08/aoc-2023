fn main() {
    println!("{}", part1(include_str!("input.txt")));
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut first_digit = None;
        let mut last_digit = '\0';

        for char in line.chars() {
            if char.is_ascii_digit() {
                if let None = first_digit {
                    first_digit = Some(char);
                }

                last_digit = char;
            }
        }

        sum += vec![first_digit.unwrap(), last_digit]
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
    fn part1_works() {
        assert_eq!(
            super::part1(
                "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
            ),
            142
        )
    }
}
