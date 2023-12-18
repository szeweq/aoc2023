use std::num::NonZeroU64;

fn map_part1(l: &str) -> (u8, u64) {
    let bl = l.as_bytes();
    let dir = match bl[0] {
        b'R' => 0,
        b'D' => 1,
        b'L' => 2,
        b'U' => 3,
        _ => unreachable!()
    };
    let num = match bl[2] {
        b'1' => if bl[3] == b'0' { 10 } else { 1 },
        b'2'..=b'9' => bl[2] - b'0',
        _ => unreachable!()
    } as u64;
    (dir, num)
}
fn map_part2(l: &str) -> (u8, u64) {
    let (_, b) = l.split_once('#').unwrap();
    let hex = u32::from_str_radix(&b[..6], 16).unwrap();
    ((hex & 3) as u8, (hex >> 4) as u64)
}
const fn pt_move(pt: (i64, i64), dir: u8, dist: u64) -> (i64, i64) {
    match dir {
        0 => (pt.0.wrapping_add_unsigned(dist), pt.1),
        1 => (pt.0, pt.1.wrapping_add_unsigned(dist)),
        2 => (pt.0.wrapping_sub_unsigned(dist), pt.1),
        3 => (pt.0, pt.1.wrapping_sub_unsigned(dist)),
        _ => unreachable!()
    }
}
fn solve(input: &str, mapper: fn(&str) -> (u8, u64)) -> Option<NonZeroU64> {
    let mut spos = (0, 0);
    let (mut perim, mut sum) = (0, 0);
    for (dir, num) in input.lines().map(mapper) {
        let npos = pt_move(spos, dir, num);
        sum += (spos.1 + npos.1) * (spos.0 - npos.0);
        perim += num;
        spos = npos;
    }
    NonZeroU64::new(perim.wrapping_add_signed(sum) / 2 + 1)
}

pub fn part1(input: &str) -> Option<NonZeroU64> {
    solve(input, map_part1)
}

pub fn part2(input: &str) -> Option<NonZeroU64> {
    solve(input, map_part2)
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex_opt;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex_opt!(part1, NonZeroU64::new(62));
    }

    #[test]
    fn test_part2() {
        assert_ex_opt!(part2, NonZeroU64::new(952408144115));
    }
}