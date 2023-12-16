use std::{borrow::Cow, num::NonZeroUsize};

type Row<'a> = (Cow<'a, [u8]>, Box<[u8]>);

fn parse_groups(input: &str, part2: bool) -> impl Iterator<Item = Row<'_>> + '_ {
    input.lines().map(move |l| {
        let (s, n) = l.split_once(' ').unwrap();
        let n = n.split(',').map(|x| x.parse().unwrap()).collect::<Box<_>>();
        let sb = s.as_bytes();
        if part2 {
            ([sb; 5].join(&b'?').into(), n.repeat(5).into_boxed_slice())
        } else {
            (sb.into(), n)
        }
    })
}

fn possible(l: &[u8], n: &[u8]) -> usize {
    let l = if l.last() == Some(&b'.') { &l[..l.len() - 1] } else { l };
    let mut vl = Vec::with_capacity(l.len() + 1);
    vl.push(b'.');
    vl.extend_from_slice(l);
    let sz = vl.len() + 1;

    let (mut oldstate, mut newstate) = (vec![0; sz], vec![0; sz]);
    oldstate[0] = 1;

    for i in 1..vl.len() {
        if vl[i] == b'#' { break; }
        oldstate[i] = 1;
    }

    for &cnt in n {
        let cnt = cnt as usize;
        let mut grp = 0;
        for (i, &c) in vl.iter().enumerate() {
            if c == b'.' { grp = 0; } else { grp += 1; }
            if c != b'#' {
                newstate[i + 1] += newstate[i];
            }
            if grp >= cnt && vl[i - cnt] != b'#' {
                newstate[i + 1] += oldstate[i - cnt];
            }
        }
        oldstate.iter_mut().for_each(|x| *x = 0);
        (oldstate, newstate) = (newstate, oldstate);
    }

    oldstate[sz - 1]
}

fn solve(input: &str, part2: bool) -> Option<NonZeroUsize> {
    NonZeroUsize::new(parse_groups(input, part2).map(|(l, n)| possible(&l, &n)).sum())
}

pub fn part1(input: &str) -> Option<NonZeroUsize> {
    solve(input, false)
}

pub fn part2(input: &str) -> Option<NonZeroUsize> {
    solve(input, true)
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex_opt;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex_opt!(part1, NonZeroUsize::new(21));
    }

    #[test]
    fn test_part2() {
        assert_ex_opt!(part2, NonZeroUsize::new(525_152));
    }
}