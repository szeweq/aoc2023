
fn parse_numbers(input: &str) -> Vec<u8> {
    let mut v: Vec<u8> = input.split(' ').filter(|s| !s.is_empty()).map(|s| s.trim().parse().unwrap()).collect();
    v.dedup();
    v.sort();
    v
}

fn parse_card_matches(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|l| {
        let (_, nums) = l.split_once(':').unwrap();
        let (wn, yn) = nums.split_once('|').unwrap();
        let wn = parse_numbers(wn);
        let yn = parse_numbers(yn);
        yn.iter().filter(|n| wn.contains(n)).count()
    })
}

pub fn part1(input: &str) -> Option<usize> {
    Some(parse_card_matches(input).filter_map(|cnt| {
        if cnt > 0 {
            Some(1 << (cnt - 1))
        } else {
            None
        }
    }).sum())
}

pub fn part2(input: &str) -> Option<u32> {
    let card_matches: Vec<_> = parse_card_matches(input).collect();
    let mut num_owned = vec![1; card_matches.len()];
    for (i, cnt) in card_matches.iter().enumerate() {
        for j in (i+1)..(num_owned.len().min(i+1+cnt)) {
            num_owned[j] += num_owned[i];
        }
    }
    Some(num_owned.iter().sum())
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(part1, 13);
    }

    #[test]
    fn test_part2() {
        assert_ex!(part2, 30);
    }
}
