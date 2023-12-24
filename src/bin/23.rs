
const ARROWS: [u8; 4] = *b"^>v<";

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
    const fn next_pos(&self, p: usize, dir: u8) -> Option<usize> {
        // 0 = up, 1 = right, 2 = down, 3 = left
        Some(match dir {
            0 if p >= self.offset => p - self.offset,
            1 if (p + 1) % self.offset != 0 => p + 1,
            2 if p < self.data.len() - self.offset => p + self.offset,
            3 if p % self.offset != 0 => p - 1,
            _ => { return None }
        })
    }
    fn graph_map(&self, slopes: bool) -> Vec<Vec<(usize, usize)>> {
        let mut vpts = vec![1, self.data.len() - 2];
        for (i, &c) in self.data.iter().enumerate() {
            let mut f = 0;
            for d in 0..4u8 {
                match self.next_pos(i, d).and_then(|j| self.data.get(j)) {
                    Some(&aa) if aa != b'#' => { f += 1; }
                    _ => {}
                }
            }
            if f > 2 && c != b'#' {
                vpts.push(i);
            }
        }
        vpts.sort_unstable();
        let mut rmap = vec![vec![]; vpts.len()];
        let mut q = Vec::with_capacity(4);
        for (id, &ii) in vpts.iter().enumerate() {
            let iv = &mut rmap[id];
            let mut visited = vec![false; self.data.len()];
            visited[ii] = true;
            q.push((ii, 0));
            while let Some((i, d)) = q.pop() {
                let ai = ARROWS.iter().position(|&a| self.data[i] == a).unwrap_or(4) as u8;
                for di in 0..4u8 {
                    if slopes && ai < 4 && ai != di {
                        continue;
                    }
                    let Some(np) = self.next_pos(i, di) else { continue };
                    match self.data.get(np) {
                        Some(&aa) if aa != b'#' => {
                            if !visited[np] {
                                visited[np] = true;
                                vpts.binary_search(&np).map_or_else(
                                    |_| { q.push((np, d+1)); },
                                    |idx| { iv.push((idx, d+1)); }
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        rmap
    }
}

fn dfs(i: usize, d: usize, rmap: &[Vec<(usize, usize)>], visited: &mut [bool], end: usize, r: &mut usize) {
    visited[i] = true;
    if i == end {
        *r = (*r).max(d);
    }
    for &(j, md) in &rmap[i] {
        if !visited[j] {
            dfs(j, md + d, rmap, visited, end, r);
        }
    }
    visited[i] = false;
}

pub fn part1(input: &str) -> Option<usize> {
    let grid = Grid::from_str(input);
    let rmap = grid.graph_map(true);
    let (mut dvisited, mut r) = (vec![false; rmap.len()], 0);
    dfs(0, 0, &rmap, &mut dvisited, rmap.len() - 1, &mut r);
    Some(r)
}

pub fn part2(input: &str) -> Option<usize> {
    let grid = Grid::from_str(input);
    let rmap = grid.graph_map(false);
    let (mut dvisited, mut r) = (vec![false; rmap.len()], 0);
    dfs(0, 0, &rmap, &mut dvisited, rmap.len() - 1, &mut r);
    Some(r)
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(part1, 94);
    }

    #[test]
    fn test_part2() {
        assert_ex!(part2, 154);
    }
}