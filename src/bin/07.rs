use std::num::NonZeroUsize;


const fn card_number(b: u8, j: bool) -> u8 {
    match b {
        b'2'..=b'9' => b - b'1',
        b'T' => 9,
        b'J' => if j { 0 } else { 10 },
        b'Q' => 11,
        b'K' => 12,
        b'A' => 13,
        _ => 0
    }
}

fn parse_hand(line: &str, j: bool, checker: fn([u8; 5]) -> u8) -> ([u8; 6], usize) {
    let lb = line.as_bytes();
    let mut ch = [0u8; 6];
    // SAFETY: Fitting the pointer because stable Rust does not allow cutting const-size slices.
    let h: &mut [u8; 5] = unsafe { &mut *(&mut ch[1] as *mut u8 as *mut [u8; 5]) };
    for i in 0..5 {
        h[i] = card_number(lb[i], j);
    }
    ch[0] = checker(*h);
    let bid = line[6..].parse::<u16>().unwrap() as usize;
    (ch, bid)
}

const fn rank(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 0) => 1, // One pair
        (1, 1) => 2, // Two pair
        (2, 0) => 3, // Three of a kind
        (2, 1) => 4, // Full house
        (3, _) => 5, // Four of a kind
        (4, _) => 6, // (?) Five of a kind
        _ => 0 // High card (?)
    }
}

/// Sorts the hand in place (it seems that const-size slices can be sorted even faster)
fn sort5(h: &mut [u8; 5]) {
    for i in 0..4 {
        for j in i+1..5 {
            if h[i] > h[j] {
                h.swap(i, j);
            }
        }
    }
}

fn check_hand(mut hand: [u8; 5]) -> u8 {
    sort5(&mut hand);
    let mut vc = [0; 4];
    let mut last = hand[0];
    let mut i = 0;
    for &c in &hand[1..] {
        if c == last {
            vc[i] += 1;
        } else {
            last = c;
            i += 1;
        }
    }
    vc.sort_unstable_by(|a, b| b.cmp(a));
    rank(vc[0], vc[1])
}
fn check_hand_joker(mut hand: [u8; 5]) -> u8 {
    if hand == [0, 0, 0, 0, 0] {
        return 6;
    }
    let mut i = 0;
    let mut j = 0;
    let mut uniq = [0; 5];
    let mut jpos = [0; 4];
    for (k, &c) in hand.iter().enumerate() {
        match c {
            0 => {
                jpos[j] = k;
                j += 1;
            },
            c => {
                if !uniq.contains(&c) {
                    uniq[i] = c;
                    i += 1;
                }
            }
        }
    }
    let mut r = check_hand(hand);
    if j > 0 {
        let uniq = &uniq[..i];
        let jpos = &jpos[..j];
        for &n in uniq {
            for &k in jpos {
                hand[k] = n;
            }
            let nr = check_hand(hand);
            if nr > r {
                r = nr;
            }
        }
    }
    r
}

fn solve(input: &str, j: bool, checker: fn([u8; 5]) -> u8) -> Option<NonZeroUsize> {
    let mut hands = input.lines().map(|l| parse_hand(l, j, checker)).collect::<Vec<_>>();
    hands.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    NonZeroUsize::new(hands.into_iter().enumerate().map(|(i, h)| (1 + i) * h.1).sum())
}

pub fn part1(input: &str) -> Option<NonZeroUsize> {
    solve(input, false, check_hand)
}

pub fn part2(input: &str) -> Option<NonZeroUsize> {
    solve(input, true, check_hand_joker)
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex_opt;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex_opt!(part1, NonZeroUsize::new(6440));
    }

    #[test]
    fn test_part2() {
        assert_ex_opt!(part2, NonZeroUsize::new(5905));
    }
}