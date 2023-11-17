use fnv::FnvHashMap;

use crate::day::Day;

pub struct Day23 {
    elves: Vec<Vec<bool>>,
}

fn elves_iter(elves: &[Vec<bool>]) -> impl Iterator<Item = (usize, usize)> + '_ {
    elves
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().filter(|&(_, &e)| e).map(move |(x, _)| (x, y)))
}

fn has_no_neighbors(elves: &[Vec<bool>], (x, y): (usize, usize)) -> bool {
    (-1..=1)
        .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)).filter(|&d| d != (0, 0)))
        .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
        .filter(|(nx, ny)| (0..elves[0].len() as isize).contains(nx) && (0..elves.len() as isize).contains(ny))
        .all(|(nx, ny)| !elves[ny as usize][nx as usize])
}

fn expand(elves: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let expand_west = elves.iter().any(|row| row[0]);
    let expand_east = elves.iter().any(|row| *row.last().unwrap());
    let expand_north = elves[0].iter().any(|&e| e);
    let expand_south = elves[elves.len() - 1].iter().any(|&e| e);

    if !expand_west && !expand_east && !expand_north && !expand_south {
        return elves;
    }

    let mut new_elves = vec![];
    let new_width = elves[0].len() + expand_west as usize + expand_east as usize;

    if expand_north {
        new_elves.push(vec![false; new_width]);
    }

    new_elves.append(
        &mut elves
            .iter()
            .map(|row| {
                let mut new_row = vec![];
                if expand_west {
                    new_row.push(false);
                }
                new_row.append(&mut row.clone());
                if expand_east {
                    new_row.push(false);
                }
                new_row
            })
            .collect(),
    );

    if expand_south {
        new_elves.push(vec![false; new_width]);
    }

    new_elves
}

const DELTAS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

impl Day23 {
    fn spread(&self, max_rounds: usize) -> (usize, Vec<Vec<bool>>) {
        let mut elves = self.elves.clone();

        for r in 0..max_rounds {
            elves = expand(elves);
            let new_positions: FnvHashMap<(usize, usize), (usize, usize)> = FnvHashMap::from_iter(
                elves_iter(&elves)
                    .filter_map(|(x, y)| {
                        if has_no_neighbors(&elves, (x, y)) {
                            None
                        } else {
                            (r..)
                                .take(4)
                                .map(|d| DELTAS[d % 4])
                                .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
                                .map(|(nx, ny)| (nx as usize, ny as usize))
                                .find(|&(nx, ny)| {
                                    if nx == x {
                                        !elves[ny][nx - 1] && !elves[ny][nx] && !elves[ny][nx + 1]
                                    } else {
                                        !elves[ny - 1][nx] && !elves[ny][nx] && !elves[ny + 1][nx]
                                    }
                                })
                                .map(|new_position| ((x, y), new_position))
                        }
                    })
                    .fold(
                        FnvHashMap::default(),
                        |mut new_positions: FnvHashMap<(usize, usize), Option<(usize, usize)>>, (elf, new_position)| {
                            new_positions
                                .entry(new_position)
                                .and_modify(|e| *e = None)
                                .or_insert(Some(elf));
                            new_positions
                        },
                    )
                    .into_iter()
                    .filter_map(|(new_position, elf)| elf.map(|e| (e, new_position))),
            );

            if new_positions.is_empty() {
                return (r, elves);
            }

            for ((x, y), (nx, ny)) in new_positions {
                elves[y][x] = false;
                elves[ny][nx] = true;
            }
        }

        (max_rounds, elves)
    }
}

impl Day<'_> for Day23 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            elves: input.lines().map(|l| l.bytes().map(|c| c == b'#').collect()).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let (_, elves) = self.spread(10);
        let min_y = elves
            .iter()
            .enumerate()
            .find(|(_, row)| row.iter().any(|&e| e))
            .map(|(y, _)| y)
            .unwrap();
        let max_y = elves
            .iter()
            .enumerate()
            .rfind(|(_, row)| row.iter().any(|&e| e))
            .map(|(y, _)| y)
            .unwrap();
        let min_x = (0..elves[0].len())
            .find(|&x| (0..elves.len()).any(|y| elves[y][x]))
            .unwrap();
        let max_x = (0..elves[0].len())
            .rfind(|&x| (0..elves.len()).any(|y| elves[y][x]))
            .unwrap();

        (max_x - min_x + 1) * (max_y - min_y + 1) - elves_iter(&self.elves).count()
    }

    fn part_2(&self) -> Self::T2 {
        self.spread(usize::MAX).0 + 1
    }
}
