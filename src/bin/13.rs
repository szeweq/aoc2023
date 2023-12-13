
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
    for y in 1..h {
        let msz = y.min(h - y);
        let rd = (0..msz).flat_map(|sz| {
            (0..w).filter(move |&x| v[y - sz - 1][x] != v[y + sz][x])
        }).count();
        if rd == diff {
            return y;
        }
    }
    0
}
fn reflect_col(v: &[&[u8]], diff: usize) -> usize {
    let (w, h) = (v[0].len(), v.len());
    for x in 1..w {
        let msz = x.min(w - x);
        let cd = (0..msz).flat_map(|sz| {
            (0..h).filter(move |&y| v[y][x - sz - 1] != v[y][x + sz])
        }).count();
        if cd == diff {
            return x;
        }
    }
    0
}

fn solve(input: &str, diff: usize) -> Option<usize> {
    Some(parse_grids(input).map(|v| 100 * reflect_row(v, diff) + reflect_col(v, diff)).sum())
}

pub fn part1(input: &str) -> Option<usize> {
    solve(input, 0)
}

pub fn part2(input: &str) -> Option<usize> {
    solve(input, 1)
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(part1, 405);
    }

    #[test]
    fn test_part2() {
        assert_ex!(part2, 400);
    }
}