use std::ops::Range;

use rayon::iter::{ParallelIterator, IntoParallelIterator};

fn parse_range(s: &str) -> Option<(Range<usize>, isize)> {
    let mut iter = s.splitn(3, ' ').map(|s| s.parse::<usize>().unwrap());
    let (r_dest, r_src, n) = (iter.next()?, iter.next()?, iter.next()?);
    Some((r_src..(r_src + n), r_dest as isize - r_src as isize))
}

type Guide = [Vec<(Range<usize>, isize)>; 7];

fn parse_guide(s: &str) -> (Vec<usize>, Guide) {
    let mut iter = s.lines();
    let seeds = iter.next().unwrap()
        .split_ascii_whitespace().skip(1).map(|s| s.parse::<usize>().unwrap());
    let niter = iter.skip(1);
    let mut maps = [vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    let mut map_id = 0;
    for line in niter {
        if line.is_empty() {
            map_id += 1;
        } else if !line.ends_with(':') {
            maps[map_id].push(parse_range(line).unwrap());
        }
    }
    for m in &mut maps {
        m.sort_by_key(|(r, _)| r.start);
    }
    (seeds.collect(), maps)
}

fn seed_to_location(seed: usize, maps: &Guide) -> usize {
    maps.iter().fold(seed, |mut dest, m| {
        for (r, n) in m {
            if r.contains(&dest) {
                dest = dest.saturating_add_signed(*n);
                break;
            }
        }
        dest
    })
}

pub fn part1(input: &str) -> Option<usize> {
    let (seeds, maps) = parse_guide(input);
    seeds.into_iter().map(|seed| seed_to_location(seed, &maps)).min()
}

/// This is a bruteforce solution!
pub fn part2_bf(input: &str) -> Option<usize> {
    let (seeds, maps) = parse_guide(input);
    seeds.chunks(2)
        .filter_map(|s| (s[0]..(s[0]+s[1])).map(|seed| seed_to_location(seed, &maps)).min())
        .min()
}

/// This is a bruteforce solution with parallelization (using rayon)
pub fn part2_bf_par(input: &str) -> Option<usize> {
    let (seeds, maps) = parse_guide(input);
    seeds.chunks(2).filter_map(|s| (s[0]..(s[0]+s[1])).into_par_iter().map(|seed| seed_to_location(seed, &maps)).min())
        .min()
}

aoc2023::solve!(part1, part2_bf_par);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(part1, 35);
    }

    #[test]
    fn test_part2_bf() {
        assert_ex!(part2_bf, 46);
    }

    #[test]
    fn test_part2_bf_par() {
        assert_ex!(part2_bf_par, 46);
    }
}