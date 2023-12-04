
struct Card {
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
}
impl Card {
    fn matches(&self) -> usize {
        self.numbers.iter().filter(|n| self.winning_numbers.contains(n)).count()
    }
}

fn parse_numbers(input: &str) -> Vec<u8> {
    let mut v: Vec<u8> = input.split(' ').filter(|s| !s.is_empty()).map(|s| s.trim().parse().unwrap()).collect();
    v.dedup();
    v.sort();
    v
}

fn parse_cards(input: &str) -> impl Iterator<Item = Card> + '_ {
    input.lines().map(|l| {
        let (_, nums) = l.split_once(':').unwrap();
        let (wn, yn) = nums.split_once('|').unwrap();
        Card {
            winning_numbers: parse_numbers(wn),
            numbers: parse_numbers(yn),
        }
    })
}

pub fn part1(input: &str) -> Option<usize> {
    Some(parse_cards(input).filter_map(|c| {
        let cnt = c.matches();
        if cnt > 0 {
            Some(1 << (cnt - 1))
        } else {
            None
        }
    }).sum())
}

pub fn part2(input: &str) -> Option<u32> {
    let cards: Vec<_> = parse_cards(input).collect();
    let mut num_owned = vec![1; cards.len()];
    for (i, c) in cards.iter().enumerate() {
        let cnt = c.matches();
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
