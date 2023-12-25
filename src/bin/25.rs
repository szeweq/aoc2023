use std::collections::VecDeque;

use rand::prelude::*;

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

fn parse_diagram(input: &str) -> Vec<Vec<usize>> {
    let mut name_indices = vec![];
    let dv = input.lines().map(|l| {
        let index = find_index(&mut name_indices, &l[..3]);
        let conns = l[5..].split_ascii_whitespace().map(|s| find_index(&mut name_indices, s)).collect::<Vec<_>>();
        (index, conns)
    }).collect::<Vec<_>>();
    let mut di = vec![vec![]; name_indices.len()];
    for (i, con) in dv {
        for j in con {
            di[i].push(j);
            di[j].push(i);
        }
    }
    di
}

fn path2(diagram: &[Vec<usize>], q: &mut VecDeque<usize>, a: usize, b: usize) -> Vec<usize> {
    let dl = diagram.len();
    let mut hist = vec![dl; dl];
    hist[a] = a;
    q.push_back(a);
    while let Some(i) = q.pop_front() {
        for &j in &diagram[i] {
            if hist[j] == dl {
                hist[j] = i;
                q.push_back(j);
            }
        }
    }
    if hist[b] == dl {
        return vec![];
    }
    let (mut v, mut i) = (vec![], b);
    while i != a {
        v.push(i);
        i = hist[i];
    }
    v.push(a);
    v.reverse();
    v
}

fn path_len(diagram: &[Vec<usize>], q: &mut VecDeque<usize>, from: usize, ecut: &[(usize, usize)]) -> usize {
    let (mut cnt, mut visited) = (1, vec![false; diagram.len()]);
    visited[from] = true;
    q.push_back(from);
    while let Some(i) = q.pop_front() {
        for &j in &diagram[i] {
            if !visited[j] && !ecut.contains(&(i, j)) && !ecut.contains(&(j, i)) {
                visited[j] = true;
                cnt += 1;
                q.push_back(j);
            }
        }
    }
    cnt
}

pub fn part1(input: &str) -> Option<usize> {
    let diagram = parse_diagram(input);
    let ur = rand::distributions::Uniform::new(0, diagram.len());
    let mut a_to_b = vec![];
    let mut dq = VecDeque::new();
    for i in 1.. {
        let a = ur.sample(&mut rand::thread_rng());
        let b = ur.sample(&mut rand::thread_rng());
        if a == b {
            continue;
        }
        
        let spath = path2(&diagram, &mut dq, a, b);
        if spath.is_empty() {
            continue;
        }
        for i in 1..spath.len() {
            let mut ai = spath[i - 1];
            let mut bi = spath[i];
            if ai > bi {
                std::mem::swap(&mut ai, &mut bi);
            }
            match a_to_b.binary_search_by_key(&(ai, bi), |&(a, b, _)| (a, b)) {
                Ok(i) => {
                    a_to_b[i].2 += 1;
                }
                Err(i) => {
                    a_to_b.insert(i, (ai, bi, 1usize));
                }
            }
        }
        if i % 20 == 0 {
            let mut copy_a_to_b = a_to_b.clone();
            copy_a_to_b.sort_unstable_by_key(|x| std::cmp::Reverse(x.2));
            let ecut = copy_a_to_b[..3].iter().map(|x| (x.0, x.1)).collect::<Vec<_>>();
            let la = path_len(&diagram, &mut dq, ecut[0].0, &ecut);
            let lb = path_len(&diagram, &mut dq, ecut[1].0, &ecut);
            if la + lb == diagram.len() {
                return Some(la * lb);
            }
        }
    }
    None
}

pub const fn part2(_input: &str) -> Option<&'static str> {
    Some("Merry Christmas!")
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(part1, 54);
    }
}