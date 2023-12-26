use std::collections::HashSet;
use aoc2023::util;

pub fn parse_grid(s: &str) -> (util::Grid, usize) {
    let g = util::Grid::from_data(s);
    let start = g.data.len() / 2;
    (g, start)
}

fn part1(input: &str, steps: usize) -> usize {
    let (g, start) = parse_grid(input);
    let odd_bit = steps & 1;
    let mut checked = g.data.iter().map(|c| *c == b'#').collect::<Vec<_>>();
    let (mut set, mut set_next) = (Vec::new(), Vec::new());
    let mut reach = 0;
    checked[start] = true;
    set.push(start);
    for st in 0..=steps {
        for &p in &set {
            if st & 1 == odd_bit {
                reach += 1;
            }
            for dir in 0..4 {
                if let Some(np) = g.next_pos(p, dir) {
                    if !checked[np] {
                        checked[np] = true;
                        set_next.push(np);
                    }
                }
            }
        }
        set.clear();
        std::mem::swap(&mut set, &mut set_next);
    }
    reach
}

pub fn part1_solve(input: &str) -> Option<usize> {
    Some(part1(input, 64))
}
pub fn part1_test(input: &str) -> Option<usize> {
    Some(part1(input, 6))
}

fn walk_wrapped<const N: usize>(g: &util::Grid, start: usize, steps: &[usize; N]) -> [usize; N] {
    let mut vw = [0; N];
    let mut checked = HashSet::new();
    let (mut set, mut set_next) = (Vec::new(), Vec::new());
    let mut reach = HashSet::new();
    let sp = [(start % g.offset) as isize, (start / g.offset) as isize];
    checked.insert(sp);
    set.push(sp);
    let mut st = 0;
    let odd_bit = steps[0] & 1;
    let mut si = 0;
    while si < steps.len() {
        for &p in &set {
            if st & 1 == odd_bit {
                reach.insert(p);
            }
            for dir in [[0, -1], [1, 0], [0, 1], [-1, 0]] {
                let np = [p[0] + dir[0], p[1] + dir[1]];
                let x = (np[0].rem_euclid(g.offset as isize)) as usize;
                let y = (np[1].rem_euclid(g.offset as isize)) as usize;
                if g.data[y * g.offset + x] == b'#' {
                    continue;
                }
                if checked.insert(np) {
                    set_next.push(np);
                }
            }
        }
        set.clear();
        std::mem::swap(&mut set, &mut set_next);
        st += 1;
        if st == steps[si]+1 {
            vw[si] = reach.len();
            si += 1;
        }
    }
    vw
}

fn part2(input: &str) -> Option<usize> {
    const STEPS: usize = 26_501_365;
    let (g, start) = parse_grid(input);
    let vx = [g.offset / 2, 3 * g.offset / 2, 5 * g.offset / 2];
    let vy = walk_wrapped(&g, start, &vx);
    // Use Lagrange polynomial
    let mut result = 0.0;
    for i in 0..3 {
        let mut term = vy[i] as f64;
        for j in 0..3 {
            if i != j {
                let num = (STEPS - vx[j]) as f64;
                let den = vx[i] as f64 - vx[j] as f64;
                term *= num / den;
            }
        }
        result += term;
    }
    Some(result as usize)
}

pub fn part2_test(input: &str) -> Option<[usize; 6]> {
    let (g, start) = parse_grid(input);
    Some(walk_wrapped(&g, start, &[6, 10, 50, 100, 500, 1000]))
}

aoc2023::solve!(part1_solve, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(part1_test, 16);
    }

    #[test]
    fn test_part2() {
        assert_ex!(part2_test, [16, 50, 1594, 6536, 167_004, 668_6977]);
    }
}