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
fn parse_next_modules<'a>(name_indices: &mut Vec<(&'a str, usize)>, p: &'a str) -> Vec<usize> {
    let mut v = p.split(", ").map(|s| find_index(name_indices, s)).collect::<Vec<_>>();
    v.sort_unstable();
    v
}

type Input = (Vec<(bool, Vec<usize>)>, Vec<u64>, usize);

fn parse_modules(input: &str) -> Input {
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
        .map(|(_, a, p)| (a, parse_next_modules(&mut name_indices, p)))
        .collect::<Vec<_>>();
    let masks = (0..v.len()).map(|i| {
        if v[i].0 {
            (0..v.len()).fold(0u64, |mut m, j| {
                if v[j].1.binary_search(&i).is_ok() {
                    m |= 1 << j;
                }
                m
            })
        } else {
            0
        }
    }).collect::<Vec<_>>();
    (v, masks, irx)
}

pub fn part1((mv, masks, _): &Input) -> Option<u32> {
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

pub fn part2((mv, masks, irx): &Input) -> Option<u64> {
    // "rx" is not in the list. We need to find the parent node.
    let anc = mv.iter().enumerate()
        .find_map(|(i, (_, v))| v.binary_search(irx).is_ok().then_some(i))?;
    // The parent node should have multiple ancestors.
    let ancestors_rx = masks[anc];
    let ancestors_len = ancestors_rx.count_ones() as usize;
    let mut states = vec![0u64; mv.len()];
    let mut q = VecDeque::new();
    let mut history = vec![0; mv.len()];
    let mut counts = vec![0; mv.len()];
    let mut lcm = vec![];
    for t in 0.. {
        q.push_back((0, false, None));
        while let Some((i, sig, from)) = q.pop_front() {
            if !sig {
                if history[i] > 0 && counts[i] == 2 && ancestors_rx & (1 << i) != 0 {
                    lcm.push(t - history[i]);
                    if lcm.len() == ancestors_len {
                        return Some(lcm.iter().fold(1u64, |acc, i| acc.lcm(i)));
                    }
                }
                history[i] = t;
                counts[i] += 1;
            }
            if i >= mv.len() {
                // It may simply not happen.
                // if i == *irx && !sig {
                //     return Some(t);
                // }
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

aoc2023::solve!(parse_modules, part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex_part;
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_ex_part!(1, parse_modules, part1, 32000000);
    }

    #[test]
    fn test_part1_ex2() {
        assert_ex_part!(2, parse_modules, part1, 11687500);
    }
}