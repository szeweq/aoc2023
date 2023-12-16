
type Input<'a> = (&'a [u8], Vec<([u8; 3], [usize; 2])>);

const unsafe fn triple(b: &str, i: usize) -> [u8; 3] {
    *(b.as_ptr().add(i) as *const [u8; 3])
}

fn parse(input: &str) -> Input {
    let mut iter = input.lines();
    let rl = iter.next().unwrap().as_bytes();
    iter.next().unwrap();
    let mut m = iter.map(|l| {
        assert!(l.len() >= 16, "Dude...");
        // SAFETY: The string has enough bytes
        let kr = unsafe { triple(l, 0) };
        let ar = unsafe { triple(l, 7) };
        let br = unsafe { triple(l, 12) };
        (kr, ar, br)
    }).collect::<Vec<_>>();
    m.sort_unstable_by_key(|x| x.0);
    let vi = m.iter().map(|&(kk, vl, vr)| {
        let il = m.binary_search_by_key(&vl, |x| x.0).ok()?;
        let ir = m.binary_search_by_key(&vr, |x| x.0).ok()?;
        Some((kk, [il, ir]))
    }).collect::<Option<Vec<_>>>().unwrap();
    (rl, vi)
}

fn cycle_turns(r: &[u8]) -> impl Iterator<Item = bool> + '_ {
    r.iter().cycle().map(|&b| match b {
        b'L' => false,
        b'R' => true,
        _ => unreachable!()
    })
}

pub fn part1((r, vi): &Input) -> Option<u32> {
    let (mut current, mut steps) = (0, 0);
    for x in cycle_turns(r) {
        let p = vi[current].1;
        let next = p[x as usize];
        steps += 1;
        if vi[next].0 == *b"ZZZ" {
            break;
        }
        current = next;
    }
    Some(steps)
}

pub fn part2((r, vi): &Input) -> Option<u64> {
    let current = vi.iter().enumerate()
        .filter_map(|(i, r)| if r.0[2] == b'A' { Some(i) } else { None });
    Some(current.fold(1, |total, mut cc| {
        let mut steps = 0;
        for x in cycle_turns(r) {
            let p = vi[cc].1;
            let next = p[x as usize];
            steps += 1;
            assert!(next != cc, "cycle: {cc:?} -> {next:?}");
            if vi[next].0[2] == b'Z' {
                break;
            }
            cc = next;
        }
        num::integer::lcm(total, steps)
    }))
}

aoc2023::solve!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex_part;
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_ex_part!(1, parse, part1, 2);
    }

    #[test]
    fn test_part1_ex2() {
        assert_ex_part!(2, parse, part1, 6);
    }

    #[test]
    fn test_part2() {
        assert_ex_part!(3, parse, part2, 6);
    }
}