
fn num_or_panic(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

fn iter_nums(s: &str, part2: bool) -> Box<dyn Iterator<Item = usize> + '_> {
    let nums = s.split_ascii_whitespace().skip(1);
    if part2 {
        let ns = nums.collect::<String>();
        Box::new(std::iter::once(num_or_panic(&ns)))
    } else {
        Box::new(nums.map(num_or_panic))
    }
}

fn parse(input: &str, part2: bool) -> Option<impl Iterator<Item = (usize, usize)> + '_> {
    let mut iter = input.lines().take(2);
    let (st, sd) = (iter.next()?, iter.next()?);
    let t_iter = iter_nums(st, part2);
    let d_iter = iter_nums(sd, part2);
    Some(t_iter.zip(d_iter))
}

fn tries((t, d): (usize, usize)) -> usize {
    let delta = t * t - 4 * d;
    let delta_sqrt = f64::sqrt(delta as f64) as usize;
    let mut x1 = (t - delta_sqrt) / 2;
    let mut x2 = t - x1; // The "rest" of time
    if d >= x1 * (t - x1) {
        x1 += 1;
    }
    if d >= x2 * (t - x2) {
        x2 -= 1;
    }
    x2 - x1 + 1
}

pub fn part1(input: &str) -> Option<usize> {
    Some(parse(input, false)?.map(tries).product())
}

pub fn part2(input: &str) -> Option<usize> {
    parse(input, true)?.next().map(tries)
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(part1, 288);
    }

    #[test]
    fn test_part2() {
        assert_ex!(part2, 71503);
    }
}