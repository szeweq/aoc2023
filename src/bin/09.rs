
fn calculate_diff_tree(n: Vec<i32>) -> Vec<Vec<i32>> {
    let mut rows = vec![n];
    while let Some(lv) = rows.last().and_then(|v| v.iter().any(|x| *x != 0).then_some(v)) {
        rows.push(lv.windows(2).map(|w| w[1] - w[0]).collect());
    }
    rows
}

fn parse_nums(input: &str) -> impl Iterator<Item = Vec<Vec<i32>>> + '_ {
    input.lines().map(|l| calculate_diff_tree(l.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect()))
}

pub fn part1(input: &str) -> Option<i32> {
    parse_nums(input).map(|mut rows| {
        for i in (0..rows.len()-1).rev() {
            let d = rows[i + 1].last()? + rows[i].last()?;
            rows[i].push(d);
        }
        rows[0].last().copied()
    }).sum()
}

pub fn part2(input: &str) -> Option<i32> {
    parse_nums(input).map(|mut rows| {
        for i in (0..rows.len()-1).rev() {
            let d = rows[i][0] - rows[i + 1][0];
            rows[i].insert(0, d);
        }
        Some(rows[0][0])
    }).sum()
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