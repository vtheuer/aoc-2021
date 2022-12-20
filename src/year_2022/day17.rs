use std::cmp::max;
use std::convert::identity;
use std::io::Read;

use crate::day::Day;
use crate::util::Joinable;

pub struct Day17 {
    pushes: Vec<isize>,
}

fn collides(grid: &[Vec<bool>], shape: &[Vec<bool>], x: isize, y: usize) -> bool {
    for (sy, row) in shape.iter().enumerate() {
        for (sx, &c) in row.iter().enumerate() {
            if c && grid[y + sy][x as usize + sx] {
                // println!(
                //     "collides at {}({}+{}), {}({}+{})",
                //     x as usize + sx,
                //     x,
                //     sx,
                //     y + sy,
                //     y,
                //     sy
                // );
                return true;
            }
        }
    }
    false
}

fn can_move_horizontally(grid: &[Vec<bool>], shape: &[Vec<bool>], x: isize, y: usize, dx: isize) -> bool {
    (if dx == -1 {
        x > 0
    } else {
        x < 7 - shape[0].len() as isize
    }) && !collides(grid, shape, x + dx, y)
}

fn can_move_down(grid: &[Vec<bool>], shape: &[Vec<bool>], x: isize, y: usize) -> bool {
    y > 0 && !collides(grid, shape, x, y - 1)
}

fn print(grid: &[Vec<bool>], shape: &[Vec<bool>], height: usize, width: usize, x: isize, y: usize) {
    println!(
        "\n{}\n+-------+",
        grid.iter()
            .enumerate()
            .rev()
            .map(|(dy, row)| format!(
                "|{}|",
                row.iter()
                    .enumerate()
                    .map(|(dx, cell)| {
                        if dy >= y
                            && dy < y + height
                            && dx >= x as usize
                            && dx < x as usize + width
                            && shape[dy - y][dx - x as usize]
                        {
                            '@'
                        } else if *cell {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            ))
            .join("\n")
    );
}

impl Day<'_> for Day17 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            pushes: input
                .lines()
                .next()
                .unwrap()
                .bytes()
                .map(|c| if c == b'<' { -1 } else { 1 })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let shapes_vec = vec![
            vec![vec![true; 4]],
            vec![
                vec![false, true, false],
                vec![true, true, true],
                vec![false, true, false],
            ],
            vec![
                vec![true, true, true],
                vec![false, false, true],
                vec![false, false, true],
            ],
            vec![vec![true; 1]; 4],
            vec![vec![true; 2]; 2],
        ];
        let mut shapes = shapes_vec.iter().cycle();
        let mut pushes = self.pushes.iter().cycle();
        let mut grid: Vec<Vec<bool>> = vec![];

        for _ in 0..2022 {
            let highest_rock = grid
                .iter()
                .enumerate()
                .find(|(_, row)| !row.iter().any(|&b| b))
                .map(|(i, _)| i)
                .unwrap_or(0);
            let shape = shapes.next().unwrap();
            let height = shape.len();
            let width = shape[0].len();
            for _ in grid.len()..highest_rock + height + 3 {
                grid.push(vec![false; 7]);
            }
            let mut x = 2;
            let mut y = highest_rock + 3;
            // print(&grid, shape, height, width, x, y);

            loop {
                let dx = *pushes.next().unwrap();
                if can_move_horizontally(&grid, shape, x, y, dx) {
                    x += dx;
                }
                // println!("{}", if dx > 0 { '>' } else { '<' });
                // print(&grid, shape, height, width, x, y);
                if can_move_down(&grid, shape, x, y) {
                    y -= 1;
                    // println!("v");
                    // print(&grid, shape, height, width, x, y);
                } else {
                    // print(&grid, shape, height, width, x, y);
                    // println!("rock is at rest");
                    break;
                }
            }

            for (sy, row) in shape.iter().enumerate() {
                for (sx, cell) in row.iter().enumerate() {
                    if *cell {
                        grid[y + sy][x as usize + sx] = true;
                    }
                }
            }
        }

        // println!(
        //     "\n{}\n+-------+",
        //     grid.iter()
        //         .rev()
        //         .map(|row| format!(
        //             "|{}|",
        //             row.iter().map(|cell| if *cell { '#' } else { '.' }).collect::<String>()
        //         ))
        //         .join("\n")
        // );

        grid.len() - grid.iter().rev().filter(|row| row.iter().all(|&c| !c)).count()
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
