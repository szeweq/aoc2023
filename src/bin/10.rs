use std::collections::HashSet;

struct Grid {
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
    fn find_s(&mut self) -> Option<(usize, Box<[u8]>)> {
        let spos = self.data.iter().position(|&c| c == b'S')?;
        let mut dirflag = [0u8; 4];
        let mut di = 0;
        if let Some(b'|' | b'7' | b'F') = spos.checked_sub(self.offset).and_then(|p| self.data.get(p)) {
            dirflag[di] = 0;
            di += 1;
        }
        if let Some(b'|' | b'L' | b'J') = self.data.get(spos + self.offset) {
            dirflag[di] = 1;
            di += 1;
        }
        if let Some(b'-' | b'L' | b'F') = spos.checked_sub(1).and_then(|p| self.data.get(p)) {
            dirflag[di] = 2;
            di += 1;
        }
        if let Some(b'-' | b'7' | b'J') = self.data.get(spos + 1) {
            dirflag[di] = 3;
            di += 1;
        }
        Some((spos, dirflag[..di].into()))
    }
    fn traverse_loop(&self, p: usize, dirs: &[u8]) -> (usize, HashSet<usize>) {
        let go = self.offset as isize;
        let ap = [-go, go, -1, 1];
        let mut set = HashSet::new();
        set.insert(p);
        let mut av = dirs.iter().map(|&df| (p, df)).collect::<Vec<_>>();
        let (mut i, mut j) = (0, 1);
        loop {
            let (cp, dirflag) = av[i];
            let Some(np) = cp.checked_add_signed(ap[dirflag as usize]) else { break; };
            let Some(nextdir) = self.data.get(np)
                .and_then(|&pb| next_dir(dirflag, pb)) else { break; };
            if set.insert(np) {
                av[i] = (np, nextdir);
            } else {
                break;
            }
            i = (i + 1) % av.len();
            if i == 0 {
                j += 1;
            }
        }
        (j, set)
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

pub fn part1(input: &str) -> Option<usize> {
    let mut grid = Grid::from_str(input);
    let (spos, dirs) = grid.find_s()?;
    let (steps, _) = grid.traverse_loop(spos, &dirs);
    Some(steps)
}

pub fn part2(input: &str) -> Option<u32> {
    let mut grid = Grid::from_str(input);
    let (spos, dirs) = grid.find_s()?;
    let (_, set) = grid.traverse_loop(spos, &dirs);
    let (mut total, mut inside) = (0, false);
    let valid = dirs.contains(&0);
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

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex_part;
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_ex_part!(1, part1, 4);
    }

    #[test]
    fn test_part1_ex2() {
        assert_ex_part!(2, part1, 8);
    }

    #[test]
    fn test_part2_ex3() {
        assert_ex_part!(3, part2, 4);
    }

    #[test]
    fn test_part2_ex4() {
        assert_ex_part!(4, part2, 8);
    }

    #[test]
    fn test_part2_ex5() {
        assert_ex_part!(5, part2, 10);
    }
}