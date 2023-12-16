use std::num::NonZeroUsize;


fn parse_grids(input: &str) -> impl Iterator<Item = &[&[u8]]> + '_ {
    let mut l = input.lines();
    let mut vcur = vec![];
    std::iter::from_fn(move || {
        vcur.clear();
        vcur.extend(l.by_ref().take_while(|l| !l.is_empty()).map(str::as_bytes));
        if vcur.is_empty() {
            None
        } else {
            // SAFETY: Trick the borrow checker to share the contents of the slice
            Some(unsafe { std::mem::transmute(&vcur[..]) })
        }
    })
}

fn reflect_row(v: &[&[u8]], diff: usize) -> usize {
    let (w, h) = (v[0].len(), v.len());
    'o: for y in 1..h {
        let mut rd = 0;
        let (mut sa, mut sb) = (y - 1, y);
        loop {
            let (va, vb) = (v[sa], v[sb]);
            for x in 0..w {
                if va[x] != vb[x] {
                    rd += 1;
                    if rd > diff { continue 'o; } // Stop the count!
                }
            }
            if sa == 0 || sb == h - 1 { break; }
            sa -= 1;
            sb += 1;
        }
        if rd == diff {
            return y;
        }
    }
    0
}
fn reflect_col(v: &[&[u8]], diff: usize) -> usize {
    let w = v[0].len();
    'o: for x in 1..w {
        let mut cd = 0;
        let (mut sa, mut sb) = (x - 1, x);
        loop {
            for &y in v {
                if y[sa] != y[sb] {
                    cd += 1;
                    if cd > diff { continue 'o; } // Stop the count!
                }
            }
            if sa == 0 || sb == w - 1 { break; }
            sa -= 1;
            sb += 1;
        }
        if cd == diff {
            return x;
        }
    }
    0
}

fn solve(input: &str, diff: usize) -> Option<NonZeroUsize> {
    NonZeroUsize::new(parse_grids(input).map(|v| 100 * reflect_row(v, diff) + reflect_col(v, diff)).sum())
}

pub fn part1(input: &str) -> Option<NonZeroUsize> {
    solve(input, 0)
}

pub fn part2(input: &str) -> Option<NonZeroUsize> {
    solve(input, 1)
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex_opt;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex_opt!(part1, NonZeroUsize::new(405));
    }

    #[test]
    fn test_part2() {
        assert_ex_opt!(part2, NonZeroUsize::new(400));
    }
}