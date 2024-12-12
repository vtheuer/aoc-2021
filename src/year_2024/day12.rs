use crate::day::Day;
use crate::util::direction::Direction;
use crate::util::direction::Direction::*;
use crate::util::grid::Grid;
use ahash::AHashSet;
use std::collections::VecDeque;
use std::convert::identity;

pub struct Day12 {
    grid: Grid<u8>,
}

impl Day12 {
    fn get_price<T>(
        &self,
        init: fn() -> T,
        add: fn(&mut T, ((usize, usize), Direction)),
        count: fn(T) -> usize,
    ) -> usize {
        self.grid
            .indices()
            .scan(Grid::init(self.grid.width, self.grid.height, false), |visited, p| {
                if visited[p] {
                    return Some(0);
                }

                let e = self.grid[p];
                let mut todo = VecDeque::from([p]);
                let mut area = 0;
                let mut fences = init();
                while let Some(neighbor) = todo.pop_front() {
                    if visited[neighbor] {
                        continue;
                    }

                    visited[neighbor] = true;
                    area += 1;

                    for d in [Up, Right, Down, Left] {
                        if let Some(o) = self.grid.next_index(neighbor, d).filter(|&o| self.grid[o] == e) {
                            if !visited[o] {
                                todo.push_back(o);
                            }
                        } else {
                            add(&mut fences, (neighbor, d));
                        }
                    }
                }

                Some(area * count(fences))
            })
            .sum()
    }
}

impl Day<'_> for Day12 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            grid: Grid::parse(input, identity),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.get_price(|| 0, |perimeter, _| *perimeter += 1, identity)
    }

    fn part_2(&self) -> Self::T2 {
        self.get_price(
            AHashSet::default,
            |fences, fence| {
                fences.insert(fence);
            },
            count_sides,
        )
    }
}

fn count_sides(fences: AHashSet<((usize, usize), Direction)>) -> usize {
    let mut used = AHashSet::default();
    let mut todo = VecDeque::from_iter(fences.clone());
    let mut sides = 0;

    while let Some(fence) = todo.pop_front() {
        if used.contains(&fence) {
            continue;
        }

        let ((x, y), d) = fence;
        used.insert(fence);
        sides += 1;
        match d {
            Up | Down => {
                let mut nx = x as isize - 1;
                while nx >= 0 && fences.contains(&((nx as usize, y), d)) {
                    used.insert(((nx as usize, y), d));
                    nx -= 1;
                }
                nx = x as isize + 1;
                while fences.contains(&((nx as usize, y), d)) {
                    used.insert(((nx as usize, y), d));
                    nx += 1;
                }
            }
            Left | Right => {
                let mut ny = y as isize - 1;
                while ny >= 0 && fences.contains(&((x, ny as usize), d)) {
                    used.insert(((x, ny as usize), d));
                    ny -= 1;
                }
                ny = y as isize + 1;
                while fences.contains(&((x, ny as usize), d)) {
                    used.insert(((x, ny as usize), d));
                    ny += 1;
                }
            }
        }
    }

    sides
}
