

fn hash(b: &str) -> usize {
    b.bytes().fold(0, |h, c| ((h + c as usize) * 17) & 0xFF)
}

pub fn part1(input: &str) -> Option<usize> {
    Some(input.split(',').map(hash).sum())
}

pub fn part2(input: &str) -> Option<usize> {
    const V: Vec<(&str, usize)> = vec![];
    let mut boxes = [V; 256];
    for s in input.split(',') {
        if let Some(l) = s.strip_suffix('-') {
            boxes[hash(l)].retain(|x| x.0 != l);
        } else {
            let l = &s[..s.len() - 2];
            let n = (s.as_bytes()[s.len() - 1] - b'0') as usize;
            let vb = &mut boxes[hash(l)];
            if let Some(x) = vb.iter_mut().find(|x| x.0 == l) {
                x.1 = n;
            } else {
                vb.push((l, n));
            }
        }
    }
    Some(boxes.into_iter().enumerate().map(|(i, v)| {
        (i + 1) * v.into_iter().enumerate().map(|(j, (_, n))| (j + 1) * n).sum::<usize>()
    }).sum())
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(part1, 1320);
    }

    #[test]
    fn test_part2() {
        assert_ex!(part2, 145);
    }
}