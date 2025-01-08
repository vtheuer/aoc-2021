use crate::day::Day;
use crate::util::direction::Direction;
use crate::util::direction::Direction::*;
use std::cmp::Ordering;
use std::cmp::Ordering::*;

pub struct Day21<'a> {
    codes: Vec<&'a str>,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum DirKey {
    Dir(Direction),
    A,
}
use DirKey::*;

impl PartialOrd<Self> for DirKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DirKey {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Dir(d), Dir(o)) => d.ordinal().cmp(&o.ordinal()),
            (Dir(_), A) => Less,
            (A, Dir(_)) => Greater,
            (A, A) => Equal,
        }
    }
}

fn numpad_cost(f: u8, t: u8) -> Vec<Direction> {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    match f.cmp(&t) {
        Less => {}
        Equal => return Vec::new(),
        Greater => {
            return numpad_cost(t, f).into_iter().map(|d| d.opposite()).collect();
        }
    }

    let fv = match f {
        0 | 10 => -1,
        _ => (f as i8 - 1) / 3,
    };
    let tv = if t == 10 { -1 } else { (t as i8 - 1) / 3 };
    let v = tv - fv;

    let fh = match f {
        0 => 1,
        10 => 2,
        _ => (f as i8 - 1) % 3,
    };
    let th = if t == 10 { 2 } else { (t as i8 - 1) % 3 };
    let h = th - fh;

    let mut r = Vec::with_capacity(5);

    if v > 0 {
        for _ in 0..v {
            r.push(Up);
        }
    } else if v < 0 {
        for _ in 0..v.unsigned_abs() {
            r.push(Down);
        }
    }

    if h > 0 {
        for _ in 0..h {
            r.push(Right);
        }
    } else if h < 0 {
        for _ in 0..h.unsigned_abs() {
            r.push(Left);
        }
    }

    r
}

fn dirpad_cost(f: DirKey, t: DirKey) -> Vec<Direction> {
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    match f.cmp(&t) {
        Less => {}
        Equal => return Vec::new(),
        Greater => {
            return dirpad_cost(t, f).into_iter().map(|d| d.opposite()).collect();
        }
    }

    match (f, t) {
        (Dir(Up), Dir(Right)) => vec![Right, Down],
        (Dir(Up), Dir(Down)) => vec![Down],
        (Dir(Up), Dir(Left)) => vec![Down, Left],
        (Dir(Up), A) => vec![Right],
        (Dir(Right), Dir(Down)) => vec![Left],
        (Dir(Right), Dir(Left)) => vec![Left, Left],
        (Dir(Right), A) => vec![Up],
        (Dir(Down), Dir(Left)) => vec![Left],
        (Dir(Down), A) => vec![Up, Right],
        (Dir(Left), A) => vec![Up, Right, Right],
        _ => unreachable!(),
    }
}

fn move_to(((vd, vn), (hd, hn)): ((Direction, usize), (Direction, usize))) -> String {
    if hn > vn {
        return move_to(((hd, hn), (vd, vn)));
    }

    let mut keys = String::new();
    if vn > 0 {
        keys.push_str(&['^', '>', 'v', '<'][vd.ordinal()].to_string().repeat(vn));
    }
    if hn > 0 {
        keys.push_str(&['^', '>', 'v', '<'][hd.ordinal()].to_string().repeat(hn));
    }
    keys
}

type Path = Vec<DirKey>;

fn press_dirs(directions: Path, depth: u8) -> Vec<Path> {
    if depth == 0 {
        let mut r = directions.clone();
        r.push(A);
        vec![r]
    } else {
        let to_permute = directions
            .into_iter()
            .scan(A, |position, next| {
                let r = press_dirs(
                    dirpad_cost(*position, next).into_iter().map(|d| Dir(d)).collect(),
                    depth - 1,
                );
                *position = next;
                Some(r)
            })
            .flat_map(|e| e.into_iter())
            .collect::<Vec<_>>();
        dbg!(&to_permute);
        permutations(&to_permute)
            .into_iter()
            .map(|patrs|)
            .map(|parts| {
                parts.into_iter().fold(Vec::new(), |mut acc, mut part| {
                    acc.append(&mut part);
                    acc
                })
            })
            .collect()
    }
}

fn to_num(position: u8, next: u8, depth: u8) -> Vec<DirKey> {
    press_dirs(numpad_cost(position, next).into_iter().map(|d| Dir(d)).collect(), depth)
        .into_iter()
        .fold(Vec::new(), |shortest, v| {
            if shortest.is_empty() || v.len() < shortest.len() {
                v
            } else {
                shortest
            }
        })
}

fn keys(code: &str, depth: u8) -> Vec<DirKey> {
    let mut keys = Vec::new();

    let mut position = 10;
    for next in code.bytes().map(|b| if b == b'A' { 10 } else { b - b'0' }) {
        keys.append(&mut to_num(position, next, depth));
        position = next;
    }

    keys
}

fn permutations<T: Clone>(s: &[T]) -> Vec<Vec<T>> {
    if s.len() == 1 {
        vec![s.to_vec()]
    } else {
        let mut dirs = s.to_vec();
        let mut r = Vec::new();
        for i in 0..s.len() {
            dirs.swap(0, i);
            for mut sub_permutation in permutations(&dirs[1..]) {
                let mut permutation = vec![dirs[0].clone()];
                permutation.append(&mut sub_permutation);
                r.push(permutation);
            }
            dirs.swap(0, i);
        }
        r
    }
}

fn format(direction: &[DirKey]) -> String {
    direction
        .iter()
        .map(|&d| match d {
            Dir(d) => d.to_char(),
            A => 'A',
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        dbg!(format(&keys("2", 0)));
        dbg!(format(&keys("2", 1)));
        // dbg!(format(&keys("2", 2)));
    }

    // #[test]
    // fn test2() {
    //     assert_eq!(keys("029A", 0), String::from("<A^A^^>AvvvA"));
    //     assert_eq!(keys("029A", 1), String::from("<<vA>>^A<A>A<AAv>A^Av<AAA^>A"));
    //     assert_eq!(
    //         keys("029A", 2),
    //         String::from("<<vAA>A^>AvAA^<A>A<<vA>>^AvA^A<<vA>>^AAv<A>A^A<A>Av<A<A>>^AAA<Av>A^A")
    //     );
    //     assert_eq!(
    //         keys("379A", 2),
    //         String::from("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A")
    //     );
    //     // assert_eq!(key_presses("029A", 0), 12);
    //     // assert_eq!(key_presses("029A", 1), 28);
    //     // assert_eq!(key_presses("029A", 2), 68);
    // }
}

impl<'a> Day<'a> for Day21<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self {
            codes: input.lines().collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.codes
            .iter()
            .map(|&code| keys(code, 2).len() * code.strip_suffix('A').unwrap().parse::<usize>().unwrap())
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
