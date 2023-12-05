use std::cmp::Ordering;

use crate::day::Day;

type Map = Vec<(usize, usize, usize)>;

#[derive(Debug)]
pub struct Day05 {
    seeds: Vec<usize>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

fn _get(m: &Map, n: usize, f: fn((usize, usize, usize)) -> usize, t: fn((usize, usize, usize)) -> usize) -> usize {
    match m
        .binary_search_by(|&e| {
            let start = f(e);
            let length = e.2;
            if n < start {
                Ordering::Greater
            } else if n > f(e) + length - 1 {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
        .map(|i| m[i])
    {
        Ok(e) => t(e) + n - f(e),
        _ => n,
    }
}

fn get(m: &Map, n: usize) -> usize {
    _get(m, n, |e| e.1, |e| e.0)
}

fn reverse(m: &Map, n: usize) -> usize {
    _get(m, n, |e| e.0, |e| e.1)
}

impl Day05 {
    fn location(&self, seed: usize) -> usize {
        [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ]
        .into_iter()
        .fold(seed, |n, map| get(map, n))
    }

    fn is_valid_seed(&self, seed: usize) -> bool {
        self.seeds
            .chunks(2)
            .map(|c| (c[0], c[1]))
            .any(|(start, length)| (start..start + length).contains(&seed))
    }
}

fn parse(part: &str) -> Map {
    let mut map: Map = part
        .lines()
        .skip(1)
        .map_while(|l| {
            let mut numbers = l.split(' ').map(|n| n.parse().unwrap());
            Some((numbers.next()?, numbers.next()?, numbers.next()?))
        })
        .collect();
    map.sort_by_key(|&(_, start, _)| start);
    map
}

impl Day<'_> for Day05 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let mut parts = input.split("\n\n");
        let seeds = parts
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect();
        let mut maps = parts.map(parse);
        Self {
            seeds,
            seed_to_soil: maps.next().unwrap(),
            soil_to_fertilizer: maps.next().unwrap(),
            fertilizer_to_water: maps.next().unwrap(),
            water_to_light: maps.next().unwrap(),
            light_to_temperature: maps.next().unwrap(),
            temperature_to_humidity: maps.next().unwrap(),
            humidity_to_location: maps.next().unwrap(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.seeds.iter().map(|&s| self.location(s)).min().unwrap()
    }

    fn part_2(&self) -> Self::T2 {
        let mut humidity_to_location = self.humidity_to_location.clone();
        humidity_to_location.sort();

        let mut temperature_to_humidity = self.temperature_to_humidity.clone();
        temperature_to_humidity.sort();

        let mut light_to_temperature = self.light_to_temperature.clone();
        light_to_temperature.sort();

        let mut water_to_light = self.water_to_light.clone();
        water_to_light.sort();

        let mut fertilizer_to_water = self.fertilizer_to_water.clone();
        fertilizer_to_water.sort();

        let mut soil_to_fertilizer = self.soil_to_fertilizer.clone();
        soil_to_fertilizer.sort();

        let mut seed_to_soil = self.seed_to_soil.clone();
        seed_to_soil.sort();

        let mut seed = 0;
        let mut location = 0;
        while !self.is_valid_seed(seed) {
            location += 1;
            let humidity = reverse(&humidity_to_location, location);
            let temperature = reverse(&temperature_to_humidity, humidity);
            let light = reverse(&light_to_temperature, temperature);
            let water = reverse(&water_to_light, light);
            let fertilizer = reverse(&fertilizer_to_water, water);
            let soil = reverse(&soil_to_fertilizer, fertilizer);
            seed = reverse(&seed_to_soil, soil);
        }
        location
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let day05 = Day05::new(input);
        dbg!(day05.part_2());
    }
}
