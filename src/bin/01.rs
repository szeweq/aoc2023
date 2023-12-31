
const fn twodigit(a: u8, b: u8) -> Option<u32> {
    if !a.is_ascii_digit() || !b.is_ascii_digit() {
        return None;
    }
    Some(((a - b'0') * 10 + (b - b'0')) as u32)
}

pub fn part1(input: &str) -> Option<u32> {
    input.lines().map(|l| {
        let b = l.as_bytes();
        let fd = b.iter().position(u8::is_ascii_digit)?;
        let ld = b.iter().rposition(u8::is_ascii_digit)?;
        twodigit(b[fd], b[ld])
    }).sum()
}

const WORD2NUM: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn part2(input: &str) -> Option<u32> {
    input.lines().map(|l| {
        let b = l.as_bytes();
        let (mut fd, mut ld) = (None, None);
        for i in 0..b.len() {
            let z = b[i];
            if z.is_ascii_digit() {
                if fd.is_none() {
                    fd = Some(z);
                }
                ld = Some(z);
            } else {
                for (j, w) in WORD2NUM.iter().enumerate() {
                    let sz = i + w.len();
                    if sz <= b.len() && &l[i..sz] == *w {
                        let z = b'1' + j as u8;
                        if fd.is_none() {
                            fd = Some(z);
                        }
                        ld = Some(z);
                        break;
                    }
                }
            }
        }
        twodigit(fd?, ld?)
    }).sum()
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex_part;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex_part!(1, part1, 142);
    }

    #[test]
    fn test_part2() {
        assert_ex_part!(2, part2, 281);
    }
}