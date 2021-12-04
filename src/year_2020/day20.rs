use fnv::{FnvHashMap, FnvHashSet};

use crate::day::Day;
use crate::util::Joinable;
use Side::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    fn opposite(&self) -> Self {
        match self {
            Top => Bottom,
            Right => Left,
            Bottom => Top,
            Left => Right,
        }
    }

    fn direction(&self) -> (i8, i8) {
        match self {
            Top => (0, -1),
            Right => (1, 0),
            Bottom => (0, 1),
            Left => (-1, 0),
        }
    }

    fn next(&self) -> Self {
        match self {
            Top => Right,
            Right => Bottom,
            Bottom => Left,
            Left => Top,
        }
    }

    fn rotate(&self, times: u8) -> Self {
        if times == 0 {
            *self
        } else {
            self.next().rotate(times - 1)
        }
    }
}

const TILE_SIZE: usize = 10;

#[derive(Debug)]
struct Tile {
    id: usize,
    pixels: Vec<Vec<bool>>,
    top: (u16, u16),
    right: (u16, u16),
    bottom: (u16, u16),
    left: (u16, u16),
}

fn border<'a>(side: impl Iterator<Item = &'a bool>) -> (u16, u16) {
    let ltr = side.fold(0, |r, &p| ((r << 1) | if p { 1 } else { 0 }));
    let rtl = (0..TILE_SIZE).fold(0, |r, i| (r << 1) | ((ltr & (1 << i)) >> i));
    (ltr, rtl)
}

fn borders_match((a, _): (u16, u16), (b, br): (u16, u16)) -> Option<bool> {
    if a == b {
        // println!("match:\n{:#012b}\n{:#012b}", a, b);
        Some(false)
    } else if a == br {
        // println!("match flip:\n{:#012b}\n{:#012b}", a, br);
        Some(true)
    } else {
        None
    }
}

// a b c d    m i e a
// e f g h => n j f b
// i j k l    o k g c
// m n o p    p l h d

fn rotate(pixels: &[Vec<bool>]) -> Vec<Vec<bool>> {
    (0..TILE_SIZE)
        .map(|y| (0..TILE_SIZE).map(|x| pixels[TILE_SIZE - 1 - x][y]).collect())
        .collect()
}

fn flipv(pixels: &[Vec<bool>]) -> Vec<Vec<bool>> {
    (0..TILE_SIZE).map(|y| pixels[TILE_SIZE - 1 - y].clone()).collect()
}

fn fliph(pixels: &[Vec<bool>]) -> Vec<Vec<bool>> {
    (0..TILE_SIZE)
        .map(|y| {
            let mut row = pixels[y].clone();
            row.reverse();
            row
        })
        .collect()
}

fn get_border(pixels: &[Vec<bool>], side: Side) -> Vec<bool> {
    match side {
        Top => pixels[0].clone(),
        Right => pixels.iter().map(|row| row[TILE_SIZE - 1]).collect::<Vec<_>>(),
        Bottom => pixels[TILE_SIZE - 1].iter().copied().collect::<Vec<_>>(),
        Left => pixels.iter().map(|row| row[0]).collect::<Vec<_>>(),
    }
}

impl Tile {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();

        let id = lines.next().unwrap()[5..9].parse().unwrap();
        let pixels = lines
            .map(|l| l.bytes().map(|c| c == b'#').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Tile {
            id,
            top: border(pixels[0].iter()),
            right: border(pixels.iter().map(|row| &row[TILE_SIZE - 1])),
            bottom: border(pixels[TILE_SIZE - 1].iter()),
            left: border(pixels.iter().map(|row| &row[0])),
            pixels,
        }
    }

    fn side(&self, side: Side) -> (u16, u16) {
        match side {
            Top => self.top,
            Right => self.right,
            Bottom => self.bottom,
            Left => self.left,
        }
    }

    fn matches(&self, side: Side, other: &Self) -> Option<(Side, bool)> {
        let self_side = self.side(side);
        [Top, Right, Bottom, Left].iter().find_map(|&other_side| {
            borders_match(self_side, other.side(other_side)).map(|flipped| (other_side, flipped))
        })
    }

    fn get_border(&self, side: Side) -> Vec<bool> {
        match side {
            Top => self.pixels[0].clone(),
            Right => self.pixels.iter().map(|row| row[TILE_SIZE - 1]).collect::<Vec<_>>(),
            Bottom => self.pixels[TILE_SIZE - 1].clone(),
            Left => self.pixels.iter().map(|row| row[0]).collect::<Vec<_>>(),
        }
    }

    fn matches2(&self, side: Side, other: &Self) -> Option<(u8, bool)> {
        let border = get_border(&self.pixels, side);
        let mut other_pixels = other.pixels.clone();

        for rotation in 0..4 {
            let other_border = get_border(&other_pixels, side.opposite());
            if border == other_border {
                return Some((rotation, false));
            } else if border == other_border.into_iter().rev().collect::<Vec<_>>() {
                return Some((rotation, true));
            } else {
                other_pixels = rotate(&other_pixels);
            }
        }

        None
    }
}

pub struct Day20 {
    tiles: Vec<Tile>,
    // matches: FnvHashMap<usize, FnvHashMap<Side, (usize, Side, bool)>>,
}

fn format_tile(pixels: &[Vec<bool>]) -> String {
    format!(
        "{}\n",
        pixels
            .iter()
            .map(|row| { row.iter().map(|p| if *p { '#' } else { '.' }).collect::<String>() })
            .join("\n")
    )
}

impl Day20 {
    fn get_tile(&self, id: usize) -> &Tile {
        self.tiles.iter().find(|tile| tile.id == id).unwrap()
    }
}

fn print_r(r: &FnvHashMap<usize, ((i8, i8), u8, bool)>) {
    let positions = r.iter().map(|(&i, &(p, _, _))| (p, i)).collect::<FnvHashMap<_, _>>();
    let min_x = positions.keys().map(|&(x, _)| x).min().unwrap();
    let max_x = positions.keys().map(|&(x, _)| x).max().unwrap();
    let min_y = positions.keys().map(|&(_, y)| y).min().unwrap();
    let max_y = positions.keys().map(|&(_, y)| y).max().unwrap();

    println!(
        "{}\n",
        (min_y..=max_y)
            .map(|y| (min_x..=max_x)
                .map(|x| match positions.get(&(x, y)) {
                    Some(&i) => format!("{:02x?}", i),
                    None => String::from("[]"),
                })
                .collect::<String>())
            .join("\n")
    );
}

impl Day<'_> for Day20 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let tiles = input
            .split("\n\n")
            .filter(|t| !t.is_empty())
            .map(Tile::new)
            .collect::<Vec<_>>();
        Day20 {
            // matches: tiles
            //     .iter()
            //     .enumerate()
            //     .flat_map(|(i, tile)| tiles.iter().skip(i + 1).map(move |other| (tile, other)))
            //     .filter_map(|(tile, other)| tile.matches(other).map(|m| (tile.id, other.id, m)))
            //     .fold(
            //         FnvHashMap::default(),
            //         |mut matches, (tile_id, other_id, (tile_side, other_side, flipped))| {
            //             matches
            //                 .entry(tile_id)
            //                 .or_insert_with(FnvHashMap::default)
            //                 .insert(tile_side, (other_id, other_side, flipped));
            //             matches
            //                 .entry(other_id)
            //                 .or_insert_with(FnvHashMap::default)
            //                 .insert(other_side, (tile_id, tile_side, flipped));
            //             matches
            //         },
            //     ),
            tiles,
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut r = FnvHashMap::default();
        r.insert(0, ((0, 0), 0, false));
        let mut to_find = vec![(0, Top), (0, Right), (0, Bottom), (0, Left)];

        while !to_find.is_empty() {
            let (index, side_to_match) = to_find.pop().unwrap();
            let tile = &self.tiles[index];
            let match_result = self
                .tiles
                .iter()
                .enumerate()
                .filter(|(i, _)| !r.contains_key(i))
                .find_map(|(i, other)| {
                    tile.matches2(side_to_match, other)
                        .map(|(rotation, flip)| (i, rotation, flip))
                });

            if let Some((next, rotation, flip)) = match_result {
                let &((x, y), _, _) = r.get(&index).unwrap();
                let (dx, dy) = side_to_match.direction();
                let (px, py) = (x + dx, y + dy);
                r.insert(next, ((px, py), rotation, flip));
                print_r(&r);
                [Top, Right, Bottom, Left]
                    .iter()
                    .map(|&side| {
                        let (ndx, ndy) = side.direction();
                        (side, (px + ndx, py + ndy))
                    })
                    .filter(|&(_, p)| r.iter().all(|(_, &(op, _, _))| p != op))
                    .for_each(|(side, _)| to_find.push((next, side)));
            } else {
                println!("nothing matches {:?} side of {:02x?}", side_to_match, index);
            }
        }

        0
    }

    fn part_2(&self) -> Self::T2 {
        // let mut previous = self
        //     .matches
        //     .iter()
        //     .find_map(|(&id, others)| {
        //         if others.len() == 2 {
        //             others
        //                 .get(&Right)
        //                 .and_then(|_| others.get(&Bottom))
        //                 .map(|_| id)
        //         } else {
        //             None
        //         }
        //     })
        //     .unwrap();
        //
        // print_tile(&self.get_tile(previous).pixels);
        //
        // for _ in 1..12 {
        //     let &(id, match_side, flip) = self.matches.get(&previous).unwrap().get(&Right).unwrap();
        //     let tile = self.get_tile(id);
        //     let mut pixels = tile.pixels.clone();
        //
        //     for _ in 0..match_side.required_rotations(Right.opposite()) {
        //         let rotated = rotate(&pixels);
        //         pixels = rotated;
        //     }
        //
        //     if flip {
        //         let flipped = flipv(&pixels);
        //         pixels = flipped;
        //     }
        //
        //     print_tile(&pixels);
        //     previous = id;
        // }

        0
    }
}
