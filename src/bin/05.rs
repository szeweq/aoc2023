use std::ops::Range;

fn parse_range(s: &str) -> Option<(Range<usize>, isize)> {
    let mut iter = s.splitn(3, ' ').map(|s| s.parse::<usize>().unwrap());
    let (r_dest, r_src, n) = (iter.next()?, iter.next()?, iter.next()?);
    Some((r_src..(r_src + n), r_dest as isize - r_src as isize))
}

type Guide = [Vec<(Range<usize>, isize)>; 7];

fn parse_guide(s: &str) -> (Box<[usize]>, Guide) {
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
        m.sort_unstable_by_key(|(r, _)| r.start);
    }
    (seeds.collect(), maps)
}

fn range_cmp(r: &Range<usize>, n: usize) -> std::cmp::Ordering {
    if r.contains(&n) {
        std::cmp::Ordering::Equal
    } else {
        r.start.cmp(&n)
    }
}

fn guide_pos(pos: usize, maps: &Guide) -> usize {
    maps.iter().fold(pos, |dest, m|
        m.binary_search_by(|(r, _)| range_cmp(r, dest))
            .map_or(dest, |fr| dest.wrapping_add_signed(m[fr].1))
    )
}

pub fn part1((seeds, maps): &(Box<[usize]>, Guide)) -> Option<usize> {
    seeds.iter().map(|&seed| guide_pos(seed, maps)).min()
}

/// This is an inverse (still bruteforce) solution
pub fn part2((seeds, maps): &(Box<[usize]>, Guide)) -> Option<usize> {
    let mut seed_ranges = seeds.chunks(2)
        .map(|s| s[0]..(s[0]+s[1]))
        .collect::<Box<_>>();
    seed_ranges.sort_unstable_by_key(|r| r.start);
    let mut inv_guide = maps.clone();
    for gv in &mut inv_guide {
        for (r, n) in &mut *gv {
            *r = (r.start.wrapping_add_signed(*n))..(r.end.wrapping_add_signed(*n));
            *n = -(*n);
        }
        gv.sort_unstable_by_key(|(r, _)| r.start);
    }
    inv_guide.reverse();
    (1..).map(|i| (i, guide_pos(i, &inv_guide)))
        .find_map(|(i, seed)| seed_ranges.binary_search_by(|r| range_cmp(r, seed)).ok().map(|_| i)
        )
}


aoc2023::solve!(parse_guide, part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(parse_guide, part1, 35);
    }

    #[test]
    fn test_part2() {
        assert_ex!(parse_guide, part2, 46);
    }
}