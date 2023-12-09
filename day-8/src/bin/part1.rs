use std::collections::HashMap;

fn main() {
    println!("{}", part1(include_str!("input.txt")));
}

fn part1(input: &str) -> i32 {
    let mut lines = input.lines();

    let lr_indices: Vec<_> = lines
        .next()
        .unwrap()
        .chars()
        .map(|char| match char {
            'L' => 0,
            'R' => 1,
            _ => panic!("what the hell"),
        })
        .collect();

    let mut map = HashMap::new();
    lines.next();

    for line in lines {
        let thing = line
            .chars()
            .filter(|char| match char {
                '=' | '(' | ',' | ')' => false,
                _ => true,
            })
            .collect::<String>();

        let things = thing
            .split_whitespace()
            .map(|stupid| stupid.to_owned())
            .collect::<Vec<_>>();

        map.insert(things[0].clone(), [things[1].clone(), things[2].clone()]);
    }

    let mut key = "AAA".to_string();
    let mut zzz_found = false;
    let mut index = 0;
    let mut steps = 0;

    while !zzz_found {
        key = map[&key][lr_indices[index % lr_indices.len()]].clone();

        if key == "ZZZ" {
            zzz_found = true;
        }

        index += 1;
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_works() {
        assert_eq!(
            part1(
                "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"
            ),
            6
        )
    }
}
