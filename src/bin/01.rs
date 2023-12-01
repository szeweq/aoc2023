
pub fn part1(input: &str) -> Option<u32> {
    let numrange = b'0'..=b'9';
    input.lines().map(|l| {
        let b = l.as_bytes();
        let fd = b.iter().position(|&z| numrange.contains(&z)).unwrap();
        let ld = b.iter().rposition(|&z| numrange.contains(&z)).unwrap();
        std::str::from_utf8(&[b[fd], b[ld]]).ok()?.parse::<u32>().ok()
    }).sum()
}

const WORD2NUM: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn part2(input: &str) -> Option<u32> {
    let numrange = b'0'..=b'9';
    input.lines().map(|l| {
        let b = l.as_bytes();
        let mut digits = Vec::new();
        let mut i = 0;
        while i < b.len() {
            let z = b[i];
            if numrange.contains(&z) {
                digits.push(z);
            } else {
                for (j, w) in WORD2NUM.iter().enumerate() {
                    let sz = i + w.len();
                    if sz <= b.len() && &l[i..sz] == *w {
                        digits.push(b'1' + j as u8);
                        break;
                    }
                }
            }
            i += 1;
        }
        std::str::from_utf8(&[digits[0], digits[digits.len() - 1]]).ok()?.parse::<u32>().ok()
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