use std::collections::HashSet;

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
    fn find_s(&self) -> Option<(usize, u8)> {
        let spos = self.data.iter().position(|&c| c == b'S')?;
        let dir = if let Some(b'|' | b'7' | b'F') = spos.checked_sub(self.offset).and_then(|p| self.data.get(p)) {
            0
        } else if let Some(b'|' | b'L' | b'J') = self.data.get(spos + self.offset) {
            1
        } else if let Some(b'-' | b'L' | b'F') = spos.checked_sub(1).and_then(|p| self.data.get(p)) {
            2
        } else if let Some(b'-' | b'7' | b'J') = self.data.get(spos + 1) {
            3
        } else {
            return None;
        };
        Some((spos, dir))
    }
    fn traverse_loop(&self, p: usize, dir: u8) -> Vec<usize> {
        let go = self.offset as isize;
        let ap = [-go, go, -1, 1];
        let mut av = (p, dir);
        let mut v = vec![p];
        loop {
            let Some(np) = av.0.checked_add_signed(ap[av.1 as usize]) else { break; };
            let Some(nextdir) = self.data.get(np)
                .and_then(|&pb| next_dir(av.1, pb)) else { break; };
            if np == p {
                break;
            }
            v.push(np);
            av = (np, nextdir);
        }
        v
    }
}

const fn next_dir(dbit: u8, pb: u8) -> Option<u8> {
    Some(match (dbit, pb) {
        (0, b'|') | (2, b'L') | (3, b'J') => 0,
        (1, b'|') | (2, b'F') | (3, b'7') => 1,
        (0, b'7') | (1, b'J') | (2, b'-') => 2,
        (0, b'F') | (1, b'L') | (3, b'-') => 3,
        _ => return None,
    })
}

pub fn parse_grid(s: &str) -> (Grid, usize, u8) {
    let grid = Grid::from_str(s);
    let (spos, dir) = grid.find_s().unwrap();
    (grid, spos, dir)
}

pub fn part1(&(ref grid, spos, dir): &(Grid, usize, u8)) -> Option<usize> {
    let steps = grid.traverse_loop(spos, dir);
    Some(steps.len() / 2)
}

pub fn part2(&(ref grid, spos, dir): &(Grid, usize, u8)) -> Option<u32> {
    let v = grid.traverse_loop(spos, dir);
    let (mut total, mut inside) = (0, false);
    let valid = dir == 0;
    let set = v.into_iter().collect::<HashSet<_>>();
    for p in 0..grid.data.len() {
        if set.contains(&p) {
            match grid.data[p] {
                b'|' | b'J' | b'L' => inside = !inside,
                b'S' if valid => inside = !inside,
                _ => {}
            }
        } else {
            total += inside as u32;
        }
        if p % grid.offset == grid.offset - 1 {
            inside = false;
        }
    }
    Some(total)
}

aoc2023::solve!(parse_grid, part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex_part;
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_ex_part!(1, parse_grid, part1, 4);
    }

    #[test]
    fn test_part1_ex2() {
        assert_ex_part!(2, parse_grid, part1, 8);
    }

    #[test]
    fn test_part2_ex3() {
        assert_ex_part!(3, parse_grid, part2, 4);
    }

    #[test]
    fn test_part2_ex4() {
        assert_ex_part!(4, parse_grid, part2, 8);
    }

    #[test]
    fn test_part2_ex5() {
        assert_ex_part!(5, parse_grid, part2, 10);
    }
}