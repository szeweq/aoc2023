use std::{collections::HashMap, rc::Rc};

struct Grid {
    data: Box<str>,
    offset: usize,
}
impl Grid {
    fn from_str(s: &str) -> Self {
        let mut lines = s.lines().peekable();
        let line_len = lines.peek().map_or(0, |l| l.len());
        Self {
            data: lines.collect::<String>().into(),
            offset: line_len,
        }
    }
    fn line_at(&self, i: usize) -> &str {
        &self.data[i * self.offset..(i + 1) * self.offset]
    }
    fn chunks(&self) -> std::slice::Chunks<'_, u8> {
        self.data.as_bytes().chunks(self.offset)
    }
}

enum Gear {
    Single(u32),
    Pair(u32, u32),
    Fluke
}
impl Gear {
    fn put(&mut self, v: u32) {
        *self = match self {
            Self::Single(x) => Self::Pair(*x, v),
            _ => Self::Fluke
        }
    }
}

fn find_numbers(s: &[u8]) -> impl Iterator<Item = (&[u8], usize)> + '_ {
    let mut i = 0;
    std::iter::from_fn(move || {
        if i >= s.len() {
            None
        } else {
            let next_digit = i + s[i..].iter().position(u8::is_ascii_digit)?;
            let num_end = s[next_digit..].iter().position(|c| !c.is_ascii_digit()).map_or(s.len(), |f| next_digit + f);
            let num = s.get(next_digit..num_end);
            i = num_end;
            num.map(|n| (n, next_digit))
        }
    })
}

fn check_dots(s: &str, from: usize, to: usize) -> bool {
    s[from..(to.min(s.len()))].bytes().all(|b| b == b'.')
}
fn star_pos(s: &str, from: usize, to: usize) -> Option<usize> {
    s[from..(to.min(s.len()))].bytes().position(|b| b == b'*').map(|p| p + from)
}

pub fn part1(input: &str) -> Option<u32> {
    let g = Rc::from(Grid::from_str(input));
    let height = g.data.len() / g.offset;
    Some(g.clone().chunks().enumerate().flat_map(|(i, line)| {
        let cg = g.clone();
        find_numbers(line).filter_map(move |(n, j)| {
            let nlen = n.len();
            let mut sides = [None, None, None, None];
            if j > 0 {
                sides[0] = Some(line[j - 1] == b'.');
            }
            if j + nlen < cg.offset {
                sides[1] = Some(line[j + nlen] == b'.');
            }
            if i > 0 {
                sides[2] = Some(check_dots(cg.line_at(i - 1), j.saturating_sub(1), j+nlen+1));
            }
            if i + 1 < height {
                sides[3] = Some(check_dots(cg.line_at(i + 1), j.saturating_sub(1), j+nlen+1));
            }

            if sides.iter().filter_map(|s| *s).any(|b| !b) {
                cg.line_at(i)[j..j+nlen].parse::<u32>().ok()
            } else {
                None
            }
        })
    }).sum())
}

pub fn part2(input: &str) -> Option<u32> {
    let g = Grid::from_str(input);
    let height = g.data.len() / g.offset;
    let mut m: HashMap<usize, Gear> = HashMap::new();
    for (i, line) in g.chunks().enumerate() {
        for (n, j) in find_numbers(line) {
            let nlen = n.len();
            let mut sides = [None, None, None, None];
            let sn = g.line_at(i)[j..j+nlen].parse::<u32>().ok()?;
            if j > 0 && line[j - 1] == b'*' {
                sides[0] = Some(i * g.offset + j - 1);
            }
            if j + nlen < g.offset && line[j + nlen] == b'*' {
                sides[1] = Some(i * g.offset + j + nlen);
            }
            if i > 0 {
                if let Some(x) = star_pos(g.line_at(i - 1), j.saturating_sub(1), j+nlen+1) {
                    sides[2] = Some((i - 1) * g.offset + x);
                }
            }
            if i + 1 < height {
                if let Some(x) = star_pos(g.line_at(i + 1), j.saturating_sub(1), j+nlen+1) {
                    sides[3] = Some((i + 1) * g.offset + x);
                }
            }

            for s in sides.iter().filter_map(|s| *s) {
                m.entry(s).and_modify(|z| z.put(sn)).or_insert(Gear::Single(sn));
            }
        }
    }
    Some(m.iter().filter_map(|(_, v)| {
        match *v {
            Gear::Pair(a, b) => Some(a * b),
            _ => None
        }
    }).sum())
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(part1, 4361);
    }

    #[test]
    fn test_part2() {
        assert_ex!(part2, 467_835);
    }
}
