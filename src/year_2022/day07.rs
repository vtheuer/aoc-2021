use std::clone;
use std::fmt::format;

use fnv::FnvHashMap;

use Line::*;

use crate::day::Day;

#[derive(Debug)]
enum Line<'a> {
    Cd(&'a str),
    File(usize),
}

pub struct Day07 {
    used: usize,
    dir_sizes: Vec<usize>,
}

fn parent(path: &str) -> String {
    let (left, _) = path.rsplit_once('/').unwrap();
    if left.is_empty() {
        String::from('/')
    } else {
        left.to_string()
    }
}

fn append_path(path: &str, name: &str) -> String {
    if path.len() == 1 {
        format!("/{}", name)
    } else {
        format!("{}/{}", path, name)
    }
}

impl Day<'_> for Day07 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let lines = input
            .lines()
            .filter(|l| match l.as_bytes()[0] {
                b'$' => l.as_bytes()[2] == b'c',
                c => c != b'd',
            })
            .map(|l| match l.as_bytes()[0] {
                b'$' => Cd(&l[5..]),
                _ => File(l.split_once(' ').map(|(s, _)| s.parse().unwrap()).unwrap()),
            })
            .collect::<Vec<_>>();
        let root = String::from('/');
        let (_, tree) = lines.iter().fold(
            (root.clone(), FnvHashMap::from_iter([(root.clone(), 0)].into_iter())),
            |(wd, mut tree), line| match line {
                Cd(dir) => (
                    match dir.as_bytes()[0] {
                        b'.' => parent(&wd),
                        b'/' => root.clone(),
                        _ => {
                            let path = append_path(&wd, dir);
                            tree.entry(path.clone()).or_insert(0);
                            path
                        }
                    },
                    tree,
                ),
                File(size) => {
                    let mut path = wd.clone();
                    loop {
                        tree.entry(path.clone())
                            .and_modify(|e| *e += *size)
                            .or_insert_with(|| unreachable!("{} not found", &path));
                        if path.len() == 1 {
                            break;
                        }
                        path = parent(&path);
                    }
                    (wd, tree)
                }
            },
        );
        Self {
            used: tree[&'/'.to_string()],
            dir_sizes: tree.values().copied().collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.dir_sizes.iter().filter(|&&size| size <= 100_000).sum()
    }

    fn part_2(&self) -> Self::T2 {
        let min_to_delete = 30_000_000 - (70_000_000 - self.used);
        self.dir_sizes
            .iter()
            .filter(|&&size| size >= min_to_delete)
            .min()
            .copied()
            .unwrap()
    }
}
