
fn parse_numbers(input: &str) -> Vec<u8> {
    let mut v: Vec<u8> = input.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
    v.dedup();
    v.sort_unstable();
    v
}

pub fn parse_card_matches(input: &str) -> Vec<usize> {
    input.lines().map(|l| {
        let (wn, yn) = l.split_once(':').and_then(|(_, nums)| nums.split_once('|')).unwrap();
        let wn = parse_numbers(wn);
        let yn = parse_numbers(yn);
        let mut cnt = 0;
        let (mut i, mut j) = (0, 0);
        while i < wn.len() && j < yn.len() {
            match wn[i].cmp(&yn[j]) {
                std::cmp::Ordering::Less => {
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    j += 1;
                }
                std::cmp::Ordering::Equal => {
                    cnt += 1;
                    i += 1;
                    j += 1;
                }
            }
        }
        cnt
    }).collect()
}

pub fn part1(input: &[usize]) -> Option<usize> {
    Some(input.iter().filter_map(|&cnt| (cnt > 0).then_some(1 << (cnt - 1))).sum())
}

pub fn part2(input: &[usize]) -> Option<u32> {
    let mut num_owned = vec![1; input.len()];
    for (i, cnt) in input.iter().enumerate() {
        for j in (i+1)..(num_owned.len().min(i+1+cnt)) {
            num_owned[j] += num_owned[i];
        }
    }
    Some(num_owned.iter().sum())
}

aoc2023::solve!(parse_card_matches, part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(parse_card_matches, part1, 13);
    }

    #[test]
    fn test_part2() {
        assert_ex!(parse_card_matches, part2, 30);
    }
}
