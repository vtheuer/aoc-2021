use crate::day::Day;

pub struct Day11 {
    seats: Vec<Vec<Option<bool>>>,
    width: isize,
    height: isize,
}

impl Day11 {
    fn is_valid(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn find_seat(&self, seats: &[Vec<Option<bool>>], (px, py): (isize, isize), (dx, dy): (isize, isize)) -> bool {
        let (mut x, mut y) = (px + dx, py + dy);

        while self.is_valid(x, y) && seats[y as usize][x as usize].is_none() {
            x += dx;
            y += dy;
        }

        self.is_valid(x, y) && seats[y as usize][x as usize] == Some(true)
    }

    fn find_count(
        &self,
        max_neighbours: usize,
        count_neighbours: impl Fn(&Vec<Vec<Option<bool>>>, (isize, isize)) -> usize,
    ) -> usize {
        let mut seats = self.seats.clone();
        let mut changed = seats
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, seat)| seat.and(Some((x, y))))
            })
            .collect::<Vec<_>>();

        while !changed.is_empty() {
            let mut new_seats = seats.clone();
            let mut new_changed = Vec::with_capacity(changed.len());
            for (x, y) in changed {
                let neighbours = count_neighbours(&seats, (x as isize, y as isize));
                new_seats[y][x] = seats[y][x].map(|occupied| {
                    if occupied && neighbours >= max_neighbours {
                        new_changed.push((x, y));
                        false
                    } else if !occupied && neighbours == 0 {
                        new_changed.push((x, y));
                        true
                    } else {
                        occupied
                    }
                });
            }
            seats = new_seats;
            changed = new_changed;
        }

        seats
            .iter()
            .map(|row| row.iter().filter(|&&seat| seat == Some(true)).count())
            .sum()
    }
}

const NEIGHBOURS: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

impl Day<'_> for Day11 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let seats = input
            .lines()
            .map(|l| l.bytes().map(|b| if b == b'L' { Some(false) } else { None }).collect())
            .collect::<Vec<Vec<_>>>();
        let width = seats[0].len() as isize;
        let height = seats.len() as isize;
        Day11 { seats, width, height }
    }

    fn part_1(&self) -> Self::T1 {
        self.find_count(4, |seats, (x, y)| {
            NEIGHBOURS
                .iter()
                .map(|&(i, j)| (x + i, y + j))
                .filter(|&(i, j)| self.is_valid(i, j))
                .filter(|&(i, j)| seats[j as usize][i as usize] == Some(true))
                .count()
        })
    }

    fn part_2(&self) -> Self::T2 {
        self.find_count(5, |seats, position| {
            NEIGHBOURS
                .iter()
                .filter(|&&direction| self.find_seat(seats, position, direction))
                .count()
        })
    }
}
