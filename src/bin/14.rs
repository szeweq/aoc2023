use std::collections::HashMap;


type Pt = [u32; 2];
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
    fn find_rocks(&self) -> (Vec<Pt>, Vec<Pt>) {
        let o = self.offset as u32;
        let (mut round_rocks, mut cube_rocks) = (Vec::with_capacity(8), Vec::with_capacity(8));
        for (i, &c) in self.data.iter().enumerate() {
            let i = i as u32;
            match c {
                b'O' => round_rocks.push([i / o, i % o]),
                b'#' => cube_rocks.push([i / o, i % o]),
                _ => (),
            }
        }
        // These should be already sorted
        (round_rocks, cube_rocks)
    }
}

fn partition(vc: &[Pt], y: u32, x: u32) -> usize {
    vc.partition_point(|&[cy, cx]| cy < y || (cy == y && cx < x))
}
fn optmin(a: Option<u32>, b: Option<u32>, or: u32) -> u32 {
    match (a, b) {
        (Some(a), Some(b)) => a.min(b),
        (Some(a), None) => a,
        (None, Some(b)) => b,
        (None, None) => or
    }
}
fn optmax(a: Option<u32>, b: Option<u32>, or: u32) -> u32 {
    match (a, b) {
        (Some(a), Some(b)) => a.max(b),
        (Some(a), None) => a,
        (None, Some(b)) => b,
        (None, None) => or
    }
}

fn collapse_north(vr: &mut [Pt], vc: &[Pt]) {
    for i in 0..vr.len() {
        let [y, x] = vr[i];
        let ci = partition(vc, y, x);
        let next_cube = vc[..ci].iter().rfind(|c| c[1] == x).map(|c| c[0] + 1);
        let next_round = vr[..i].iter().rfind(|r| r[1] == x).map(|r| r[0] + 1);
        let next = optmax(next_cube, next_round, 0);
        if next < y {
            let ni = partition(&vr[..i], next, x);
            if ni < i {
                vr[ni..=i].rotate_right(1);
            }
            vr[ni][0] = next;
        }
    }
}
fn collapse_east(vr: &mut [Pt], vc: &[Pt], lx: u32) {
    for i in (0..vr.len()).rev() {
        let [y, x] = vr[i];
        let ci = partition(vc, y, x);
        let next_cube = vc[ci..].iter().find(|&c| c[0] == y && c[1] > x).map(|c| c[1] - 1);
        let next_round = vr[i+1..].iter().find(|&r| r[0] == y && r[1] > x).map(|r| r[1] - 1);
        let next = optmin(next_cube, next_round, lx);
        if next > x {
            let ni = partition(&vr[i..], y, next) + i;
            if ni > i {
                vr[i..ni].rotate_left(1);
            }
            vr[ni-1][1] = next;
        }
    }
}
fn collapse_south(vr: &mut [Pt], vc: &[Pt], ly: u32) {
    for i in (0..vr.len()).rev() {
        let [y, x] = vr[i];
        let ci = partition(vc, y, x);
        let next_cube = vc[ci..].iter().find(|&c| c[1] == x).map(|c| c[0] - 1);
        let next_round = vr[i+1..].iter().find(|&r| r[1] == x).map(|r| r[0] - 1);
        let next = optmin(next_cube, next_round, ly);
        if next > y {
            let ni = partition(&vr[i..], next, x) + i;
            if ni > i {
                vr[i..ni].rotate_left(1);
            }
            vr[ni-1][0] = next;
        }
    }
}
fn collapse_west(vr: &mut [Pt], vc: &[Pt]) {
    for i in 0..vr.len() {
        let [y, x] = vr[i];
        let ci = partition(vc, y, x);
        let next_cube = vc[..ci].iter().rfind(|c| c[0] == y && c[1] < x).map(|c| c[1] + 1);
        let next_round = vr[..i].iter().rfind(|r| r[0] == y && r[1] < x).map(|r| r[1] + 1);
        let next = optmax(next_cube, next_round, 0);
        if next < x {
            let ni = partition(&vr[..i], y, next);
            if ni < i {
                vr[ni..=i].rotate_right(1);
            }
            vr[ni][1] = next;
        }
    }
}

fn collapse_cycle(vr: &mut [Pt], vc: &[Pt], lx: u32, ly: u32) {
    collapse_north(vr, vc);
    collapse_west(vr, vc);
    collapse_south(vr, vc, ly);
    collapse_east(vr, vc, lx);
}

fn calc_damage(vr: Vec<Pt>, h: u32) -> u32 {
    vr.into_iter().map(|r| h - r[0]).sum()
}

pub fn part1(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input);
    let (mut round_rocks, cube_rocks) = grid.find_rocks();
    collapse_north(&mut round_rocks, &cube_rocks);
    Some(calc_damage(round_rocks, (grid.data.len() / grid.offset) as u32))
}

pub fn part2(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input);
    let (w, h) = (grid.offset as u32, (grid.data.len() / grid.offset) as u32);
    let (mut round_rocks, cube_rocks) = grid.find_rocks();
    let mut map = HashMap::new();
    map.insert(round_rocks.clone(), 0);
    for i in 1.. {
        collapse_cycle(&mut round_rocks, &cube_rocks, w - 1, h - 1);
        if map.contains_key(&round_rocks) {
            break;
        }
        map.insert(round_rocks.clone(), i);
    }
    let cstart = map[&round_rocks];
    let cycle_len = map.len() - cstart;
    let ckey = (1_000_000_000 - cstart) % cycle_len + cstart;
    map.into_iter().find(|(_, i)| *i == ckey).map(|(v, _)| calc_damage(v, h))
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(part1, 136);
    }

    #[test]
    fn test_part2() {
        assert_ex!(part2, 64);
    }
}