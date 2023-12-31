
struct CubeSet {
    red: u8,
    green: u8,
    blue: u8,
}

fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = CubeSet> + '_> + '_ {
    input.lines().map(|l| {
        let b = l.as_bytes();
        let sep = b.iter().position(|&b| b == b':').unwrap();
        l[sep+1..].split(';').map(|s| {
            s.split(',').map(|zs| {
                let ns = zs.trim_start();
                let nextword = ns.bytes().position(|b| b == b' ').unwrap();
                let num = ns[..nextword].parse().unwrap();
                let letter = ns.as_bytes()[nextword+1];
                (letter, num)
            }).fold(CubeSet { red: 0, green: 0, blue: 0 }, |mut cube, (letter, num)| {
                match letter {
                    b'r' => cube.red = num,
                    b'g' => cube.green = num,
                    b'b' => cube.blue = num,
                    _ => unreachable!(),
                }
                cube
            })
            

        })
    })
}

pub fn part1(input: &str) -> Option<usize> {
    let bag = CubeSet { red: 12, green: 13, blue: 14 };
    let games = parse(input);
    let mut total = 0;
    for (i, mut game) in games.enumerate() {
        if game.all(|cs| cs.red <= bag.red && cs.green <= bag.green && cs.blue <= bag.blue) {
            total += i + 1;
        }
    }
    Some(total)
}

pub fn part2(input: &str) -> Option<usize> {
    let games = parse(input);
    let mut total = 0;
    for game in games {
        let highest = game.fold(CubeSet { red: 0, green: 0, blue: 0 }, |mut hcs, cs| {
            if cs.red > hcs.red {
                hcs.red = cs.red;
            }
            if cs.green > hcs.green {
                hcs.green = cs.green;
            }
            if cs.blue > hcs.blue {
                hcs.blue = cs.blue;
            }
            hcs
        });
        total += highest.red as usize * highest.green as usize * highest.blue as usize;
    }
    Some(total)
}

aoc2023::solve!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc2023::assert_ex;
    use super::*;

    #[test]
    fn test_part1() {
        assert_ex!(part1, 8);
    }

    #[test]
    fn test_part2() {
        assert_ex!(part2, 2286);
    }
}