use std::collections::VecDeque;

use num::Integer;


fn find_index<'a>(v: &mut Vec<(&'a str, usize)>, s: &'a str) -> usize {
    match v.binary_search_by_key(&s, |(z, _)| z) {
        Ok(i) => v[i].1,
        Err(ins) => {
            let i = v.len();
            v.insert(ins, (s, i));
            i
        }
    }
}

fn parse_modules(input: &str) -> (Vec<(bool, Vec<usize>)>, usize) {
    let mut name_indices = vec![("broadcaster", 0)];
    let mut v = input.lines().map(|l| {
        let (name, points) = l.split_once(" -> ").unwrap();
        let (i, a) = if name == "broadcaster" {
            (0, false)
        } else {
            let lb = l.as_bytes();
            let a = match lb[0] {
                b'%' => false,
                b'&' => true,
                _ => unreachable!(),
            };
            let name = find_index(&mut name_indices, &name[1..]);
            (name, a)
        };
        (i, a, points)
    }).collect::<Vec<_>>();
    v.sort_unstable_by_key(|z| z.0);
    let irx = find_index(&mut name_indices, "rx");
    let v = v.into_iter()
        .map(|(_, a, p)| (a, p.split(", ").map(|s| find_index(&mut name_indices, s)).collect()))
        .collect();
    (v, irx)
}

pub fn part1(input: &str) -> Option<u32> {
    let (mv, _) = parse_modules(input);
    let masks = (0..mv.len()).map(|i| {
        if mv[i].0 {
            (0..mv.len()).fold(0u64, |mut m, j| {
                if mv[j].1.contains(&i) {
                    m |= 1 << j;
                }
                m
            })
        } else {
            0
        }
    }).collect::<Vec<_>>();
    let mut states = vec![0u64; mv.len()];
    let mut lohi = [0, 0];
    let mut q = VecDeque::new();
    for _ in 0..1000 {
        q.push_back((0, false, None));
        while let Some((i, sig, from)) = q.pop_front() {
            lohi[sig as usize] += 1;
            if i >= mv.len() {
                continue;
            }
            let (a, v) = &mv[i];
            let cs = if i == 0 {
                sig
            } else if *a {
                if let Some(from) = from {
                    if sig {
                        states[i] |= 1u64 << from;
                    } else {
                        states[i] &= (1u64 << from) ^ u64::MAX;
                    }
                } else {
                    continue;
                }
                states[i] != masks[i]
            } else if !sig {
                let cs = states[i] == 0;
                states[i] = cs as u64;
                cs
            } else {
                continue;
            };
            q.extend(v.iter().map(|&ni| (ni, cs, Some(i))));
        }
    }
    Some(lohi[0] * lohi[1])
}

pub fn part2(input: &str) -> Option<u64> {
    let (mv, irx) = parse_modules(input);
    let masks = (0..mv.len()).map(|i| {
        if mv[i].0 {
            (0..mv.len()).fold(0u64, |mut m, j| {
                if mv[j].1.contains(&i) {
                    m |= 1 << j;
                }
                m
            })
        } else {
            0
        }
    }).collect::<Vec<_>>();
    // "rx" is not in the list. We need to find the parent node.
    let anc = mv.iter().enumerate()
        .find_map(|(i, (_, v))| v.contains(&irx).then_some(i))?;
    // The parent node should have multiple ancestors.
    let ancestors_rx = mv.iter().enumerate()
        .filter_map(|(i, (_, v))| v.contains(&anc).then_some(i))
        .collect::<Vec<_>>();
    let mut states = vec![0u64; mv.len()];
    let mut q = VecDeque::new();
    let mut history = vec![0; mv.len()];
    let mut counts = vec![0; mv.len()];
    let mut lcm = vec![];
    for t in 0.. {
        q.push_back((0, false, None));
        while let Some((i, sig, from)) = q.pop_front() {
            if !sig {
                if history[i] > 0 && counts[i] == 2 && ancestors_rx.contains(&i) {
                    lcm.push(t - history[i]);
                }
                history[i] = t;
                counts[i] += 1;
            }
            if lcm.len() == ancestors_rx.len() {
                return Some(lcm.iter().fold(1u64, |acc, i| acc.lcm(i)));
            }
            if i >= mv.len() {
                if i == irx && !sig {
                    return Some(t);
                }
                continue;
            }
            let (a, v) = &mv[i];
            let cs = if i == 0 {
                sig
            } else if *a {
                if let Some(from) = from {
                    if sig {
                        states[i] |= 1u64 << from;
                    } else {
                        states[i] &= (1u64 << from) ^ u64::MAX;
                    }
                } else {
                    continue;
                }
                states[i] != masks[i]
            } else if !sig {
                let cs = states[i] == 0;
                states[i] = cs as u64;
                cs
            } else {
                continue;
            };
            q.extend(v.iter().map(|&ni| (ni, cs, Some(i))));
        }
    }
    None
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex_part;
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_ex_part!(1, part1, 32000000);
    }

    #[test]
    fn test_part1_ex2() {
        assert_ex_part!(2, part1, 11687500);
    }
}