use std::vec::IntoIter;

type IterRows = IntoIter<Box<[i32]>>;

fn calc_diff_tree(n: Box<[i32]>) -> IterRows {
    let mut rows = vec![n];
    while let Some(lv) = rows.last().and_then(|v| v.iter().any(|x| *x != 0).then_some(v)) {
        rows.push(lv.windows(2).map(|w| w[1] - w[0]).collect());
    }
    rows.into_iter()
}

fn parse_nums(input: &str) -> impl Iterator<Item = IterRows> + '_ {
    input.lines().map(|l| calc_diff_tree(
        l.split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    ))
}

pub fn part1(input: &str) -> Option<i32> {
    Some(parse_nums(input)
        .map(|rows| rows.map(|v| v[v.len() - 1]).sum::<i32>())
        .sum()
    )
}

pub fn part2(input: &str) -> Option<i32> {
    Some(parse_nums(input)
        .map(|rows| rows.rfold(0, |acc, v| v[0] - acc))
        .sum()
    )
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(part1, 114);
    }

    #[test]
    fn test_part2() {
        assert_ex!(part2, 2);
    }
}