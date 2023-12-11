
pub struct Grid {
    data: Box<[u8]>,
    offset: usize,
}
impl Grid {
    fn from_str(s: &str) -> Self {
        let mut lines = s.lines().peekable();
        let line_len = lines.peek().map_or(0, |l| l.len());
        Self {
            data: lines.flat_map(str::as_bytes).copied().collect::<Box<_>>(),
            offset: line_len,
        }
    }
    fn find_pos(&self) -> Vec<[usize; 2]> {
        self.data.iter().enumerate().filter_map(|(i, &c)| {
            (c == b'#').then_some([i % self.offset, i / self.offset])
        }).collect()
    }
}
fn parse_grid(input: &str) -> Grid {
    Grid::from_str(input)
}

fn space_points(vpos: &mut [[usize; 2]], w: usize, h: usize, sz: usize) {
    for (r, d) in [(0..w, 0), (0..h, 1)] {
        for i in r.rev() {
            if vpos.iter().all(|p| p[d] != i) {
                vpos.iter_mut().filter(|p| p[d] > i).for_each(|p| p[d] += sz);
            }
        }
    }
}

pub fn solve(grid: &Grid, sz: usize) -> Option<usize> {
    let mut vpos = grid.find_pos();
    space_points(&mut vpos, grid.offset, grid.data.len() / grid.offset, sz);
    Some(vpos.iter().enumerate().flat_map(|(i, &[x1, y1])| {
        vpos.iter().skip(i + 1).map(move |&[x2, y2]| x2.abs_diff(x1) + y2.abs_diff(y1))
    }).sum())
}

pub fn part1(grid: &Grid) -> Option<usize> {
    solve(grid, 1)
}

pub fn part2(grid: &Grid) -> Option<usize> {
    solve(grid, 999999)
}

pub fn test_part2_with_10(grid: &Grid) -> Option<usize> {
    solve(grid, 9)
}
pub fn test_part2_with_100(grid: &Grid) -> Option<usize> {
    solve(grid, 99)
}

aoc2023::solve!(parse_grid, part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(parse_grid, part1, 374);
    }

    #[test]
    fn test_part2_but_with_10() {
        assert_ex!(parse_grid, test_part2_with_10, 1030);
    }

    #[test]
    fn test_part2_but_with_100() {
        assert_ex!(parse_grid, test_part2_with_100, 8410);
    }
}