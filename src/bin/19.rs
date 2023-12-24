
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

type Input = (Vec<Vec<(Option<(u8, bool, u16)>, usize)>>, Vec<[u16; 4]>, usize);

pub fn parse(input: &str) -> Input {
    let mut il = input.lines();
    let mut rv = vec![];
    let mut name_indices = vec![("A", 1), ("R", 0)];
    for l in il.by_ref().take_while(|l| !l.is_empty()) {
        let (name, data) = l.split_once('{').unwrap();
        let ln = find_index(&mut name_indices, name);
        let uv = data.split(',').map(|s| {
            if let Some(s) = s.strip_suffix('}') {
                (None, find_index(&mut name_indices, s))
            } else {
                let sb = s.as_bytes();
                let (a, b) = (sb[0], sb[1]);
                let a = match a {
                    b'x' => 0,
                    b'm' => 1,
                    b'a' => 2,
                    b's' => 3,
                    _ => unreachable!(),
                };
                let (n, x) = s[2..].split_once(':').unwrap();
                let n = n.parse::<u16>().expect(n);
                (Some((a, b == b'>', n)), find_index(&mut name_indices, x))
            }
        }).collect();
        rv.push((ln, uv));
    }
    rv.sort_unstable_by_key(|z| z.0);
    let fixed_rv = rv.into_iter().map(|z| z.1)
        .collect::<Vec<_>>();
    let xv = il.map(|l| {
        let mut xmas = [0u16; 4];
        for (i, n) in l[1..l.len()-1].splitn(4, ',').enumerate() {
            xmas[i] = n[2..].parse().expect(n);
        }
        xmas
    }).collect();
    let iin = find_index(&mut name_indices, "in");
    (fixed_rv, xv, iin)
}

pub fn part1(&(ref rv, ref xv, iin): &Input) -> Option<u64> {
    let mut total = 0;
    for xmas in xv {
        let mut i = iin;
        while i >= 2 {
            for &(a, ni) in &rv[i - 2] {
                let ma = match a {
                    Some((x, false, n)) => xmas[x as usize] < n,
                    Some((x, true, n)) => xmas[x as usize] > n,
                    None => true
                };
                if ma {
                    i = ni;
                    break;
                }
            }
        }
        if i == 1 {
            total += xmas.iter().fold(0u64, |acc, x| acc + (*x as u64));
        }
    }
    Some(total)
}

pub fn part2(&(ref rv, _, iin): &Input) -> Option<u64> {
    let mut total = 0;
    let mut r_xmas = vec![(iin, [(1u16, 4000u16); 4])];
    while let Some((i, mut xmas)) = r_xmas.pop() {
        match i {
            0 => {}
            1 => {
                total += xmas.into_iter().fold(1u64, |acc, x| acc * (x.1 - x.0 + 1) as u64);
            }
            _ => {
                for &(a, ni) in &rv[i - 2] {
                    match a {
                        Some((x, false, n)) => {
                            if xmas[x as usize].0 < n {
                                let mut nx = xmas;
                                nx[x as usize].1 = nx[x as usize].1.min(n - 1);
                                r_xmas.push((ni, nx));
                            }
                            xmas[x as usize].0 = n;
                        }
                        Some((x, true, n)) => {
                            if xmas[x as usize].1 > n {
                                let mut nx = xmas;
                                nx[x as usize].0 = nx[x as usize].0.max(n + 1);
                                r_xmas.push((ni, nx));
                            }
                            xmas[x as usize].1 = n;
                        }
                        None => {
                            r_xmas.push((ni, xmas));
                        }
                    }
                }
            }
        }
    }
    Some(total)
}

aoc2023::solve!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(parse, part1, 19114);
    }

    #[test]
    fn test_part2() {
        assert_ex!(parse, part2, 167_409_079_868_000);
    }
}