use std::fs::read_to_string;
use std::time::Instant;
fn main() {
    println!("{}", part1(&"input.txt"));
    let start_time = Instant::now();
    println!("{}", part2(&"input.txt"));
    println!("{:?}", start_time.elapsed().as_secs());
}
#[derive(Debug, Clone)]
struct ResourceMap {
    source_starts: Vec<isize>,
    dest_starts: Vec<isize>,
    range_lens: Vec<isize>,
}
impl ResourceMap {
    fn new() -> Self {
        ResourceMap {
            source_starts: vec![],
            dest_starts: vec![],
            range_lens: vec![],
        }
    }
}

fn part1(filepath: &str) -> isize {
    let file = read_to_string(&filepath).unwrap();
    let seeds = file
        .lines()
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|seed_num| seed_num.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    let resource_maps = get_resource_map(&file);

    seeds
        .iter()
        .map(|seed_num| get_seed_correspondence(&&resource_maps, seed_num))
        .min()
        .unwrap()
}

fn get_resource_map(file: &str) -> Vec<ResourceMap> {
    let mut resource_maps: Vec<ResourceMap> = vec![];
    let mut curr_resource_map = ResourceMap::new();
    file.lines().skip(2).for_each(|line| {
        if line.contains(|c: char| c.is_ascii_alphanumeric()) {
            if line.starts_with(|c: char| !c.is_numeric()) {
                //we've found a new map
                resource_maps.push(curr_resource_map.clone());
                curr_resource_map = ResourceMap::new();
            } else {
                let new_line_nums = line
                    .trim()
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>();

                curr_resource_map.dest_starts.push(new_line_nums[0]);
                curr_resource_map.source_starts.push(new_line_nums[1]);
                curr_resource_map.range_lens.push(new_line_nums[2]);
            }
        }
    });
    resource_maps.push(curr_resource_map);
    resource_maps.remove(0); //due to my poor file parsing skills
    return resource_maps;
}

fn get_seed_correspondence(maps: &Vec<ResourceMap>, seed_num: &isize) -> isize {
    let mut curr_num = seed_num.clone();
    for map in maps {
        for (source_start_index, source_start) in map.source_starts.iter().enumerate() {
            let gap: isize = curr_num - *source_start;
            if gap < map.range_lens[source_start_index] && gap >= 0 {
                //if the source number falls into this range, convert it and break
                curr_num = gap + map.dest_starts[source_start_index];
                break;
            }
            //if the source nmber doesn't fall into this range check the next one
            //no change to the number if not within the ranges
        }
    }
    curr_num as isize
}
#[derive(Debug, Clone)]
struct SeedRange {
    minimum: isize,
    range: isize,
}
impl SeedRange {
    fn new() -> Self {
        SeedRange {
            minimum: 0,
            range: 0,
        }
    }
}

fn part2(filepath: &str) -> isize {
    let file = read_to_string(&filepath).unwrap();
    let seeds_strs = file
        .lines()
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();
    let mut seeds_ranges: Vec<SeedRange> = vec![];
    let mut curr_seed: SeedRange = SeedRange::new();
    for (index, num_str) in seeds_strs.iter().enumerate() {
        if index % 2 == 0 {
            //seed number
            curr_seed.minimum = num_str.parse::<isize>().unwrap().clone();
        } else {
            curr_seed.range = num_str.parse::<isize>().unwrap().clone();
            seeds_ranges.push(curr_seed.clone());
            curr_seed = SeedRange::new();
        }
    }

    let resource_maps = get_resource_map(&file);

    seeds_ranges
        .iter()
        .map(|seed_range: &SeedRange| {
            println!("Starting a range");
            (seed_range.minimum..seed_range.minimum + seed_range.range)
                .map(|seed_num: isize| get_seed_correspondence(&resource_maps, &seed_num))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

