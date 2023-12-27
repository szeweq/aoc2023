use aoc2023::util::Grid;

const ARROWS: [u8; 4] = *b"^>v<";

fn graph_map(grid: &Grid, slopes: bool) -> Vec<Vec<(usize, usize)>> {
    let mut vpts = vec![1, grid.data.len() - 2];
    for (i, &c) in grid.data.iter().enumerate() {
        let mut f = 0;
        for d in 0..4u8 {
            match grid.next_pos(i, d).and_then(|j| grid.data.get(j)) {
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
        let mut visited = vec![false; grid.data.len()];
        visited[ii] = true;
        q.push((ii, 0));
        while let Some((i, d)) = q.pop() {
            let ai = ARROWS.iter().position(|&a| grid.data[i] == a).unwrap_or(4) as u8;
            for di in 0..4u8 {
                if slopes && ai < 4 && ai != di {
                    continue;
                }
                let Some(np) = grid.next_pos(i, di) else { continue };
                match grid.data.get(np) {
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
    let grid = Grid::from_data(input);
    let rmap = graph_map(&grid, true);
    let (mut dvisited, mut r) = (vec![false; rmap.len()], 0);
    dfs(0, 0, &rmap, &mut dvisited, rmap.len() - 1, &mut r);
    Some(r)
}

pub fn part2(input: &str) -> Option<usize> {
    let grid = Grid::from_data(input);
    let rmap = graph_map(&grid, false);
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