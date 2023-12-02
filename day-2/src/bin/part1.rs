use std::collections::HashMap;

fn main() {
    println!("{}", part1(include_str!("input.txt")));
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;

    'games: for line in input.lines() {
        let mut counts = vec![];
        let mut colors = vec![];
        let stupid_ass_let_binding_borrow_checker_god_damn_it = line
            .chars()
            .skip_while(|&x| x != ':')
            .skip(1)
            .collect::<String>()
            .replace(';', "")
            .replace(',', "");

        for (i, count_or_color) in stupid_ass_let_binding_borrow_checker_god_damn_it
            .split_whitespace()
            .enumerate()
        {
            if i % 2 == 0 {
                counts.push(count_or_color.parse::<i32>().unwrap());
                continue;
            }

            colors.push(count_or_color.trim());
        }

        let color_count_map = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

        for (count, color) in counts.into_iter().zip(colors) {
            if count > color_count_map[color] {
                continue 'games;
            }
        }

        sum += line
            .split("Game ")
            .skip(1)
            .collect::<String>()
            .split(':')
            .nth(0)
            .unwrap()
            .parse::<i32>()
            .unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_works() {
        assert_eq!(
            part1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        )
    }
}
