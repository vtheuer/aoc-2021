use crate::day::Day;

pub struct Day09 {
    disk_map: Vec<u8>,
}

impl Day<'_> for Day09 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            disk_map: input.bytes().take_while(|&b| b >= b'0').map(|b| b - b'0').collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut disk: Vec<Option<usize>> = self
            .disk_map
            .iter()
            .fold((true, 0, Vec::new()), |(file, id, mut disk), &length| {
                let block = if file { Some(id) } else { None };
                disk.append(&mut vec![block; length as usize]);
                (!file, id + if file { 1 } else { 0 }, disk)
            })
            .2;

        let mut i = 0;
        let mut j = disk.len() - 1;

        while i < j {
            if disk[i].is_some() {
                i += 1;
            } else if disk[j].is_none() {
                j -= 1;
            } else {
                disk[i] = disk[j];
                disk[j] = None;
                i += 1;
                j -= 1;
            }
        }

        disk.into_iter()
            .enumerate()
            .take_while(|(_, id)| id.is_some())
            .map(|(i, id)| i * id.unwrap())
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        let mut disk = self
            .disk_map
            .iter()
            .fold((0, true, Vec::new()), |(id, file, mut disk), &length| {
                disk.push((if file { Some(id) } else { None }, length));
                (id + if file { 1 } else { 0 }, !file, disk)
            })
            .2;

        let mut i = disk.len() - 1;

        while i > 0 {
            if disk[i].0.is_some() {
                let file_length = disk[i].1;
                if let Some((j, &(_, length))) = disk
                    .iter()
                    .enumerate()
                    .take_while(|&(j, _)| j < i)
                    .find(|(_, &(id, length))| id.is_none() && length >= file_length)
                {
                    disk[j] = disk[i];
                    let mut hole_length = file_length;
                    while i < disk.len() && disk[i].0.is_none() {
                        hole_length += disk[i].1;
                        disk.remove(i);
                    }
                    disk[i] = (None, hole_length);

                    let remaining_length = length - file_length;
                    if remaining_length > 0 {
                        disk.insert(j + 1, (None, remaining_length));
                    } else {
                        i -= 1;
                    }
                } else {
                    i -= 1;
                }
            } else {
                i -= 1;
            }
        }

        disk.into_iter()
            .fold((0, 0), |(mut checksum, mut i), (file, length)| {
                if let Some(id) = file {
                    for _ in 0..length {
                        checksum += i * id;
                        i += 1;
                    }
                } else {
                    i += length as usize;
                }

                (checksum, i)
            })
            .0
    }
}
