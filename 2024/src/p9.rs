use advent_of_code_2024::{read_file, Part, Which};

pub fn p9(choice: Which, part: Part) {
    let file_data: String = read_file(9, choice, None);
    let now = std::time::SystemTime::now();
    match part {
        Part::One => part1(&file_data),
        Part::Two => part2(&file_data),
    }
    match now.elapsed() {
        Ok(elapsed) => println!("Runtime: {} microseconds", elapsed.as_micros()),
        _ => panic!(),
    }
}
#[derive(Clone)]
struct Block {
    file_id: Option<usize>,
}

fn part1(data: &str) {
    let mut file_count: usize = 0;
    let mut blocks = vec![];
    data.chars().enumerate().for_each(|(map_idx, chr)| {
        if map_idx % 2 == 0 {
            for _ in 0..chr.to_digit(10).unwrap() as usize {
                blocks.push(Block {
                    file_id: Some(file_count.clone()),
                });
            }

            file_count += 1;
        } else {
            for _ in 0..chr.to_digit(10).unwrap() as usize {
                blocks.push(Block { file_id: None })
            }
        }
    });

    let mut right = blocks.len() - 1;
    let mut left = 0;
    while left < right {
        if let Some(_file_id) = blocks[left].file_id {
            left += 1;
            continue;
        }
        if let None = blocks[right].file_id {
            right -= 1;
            continue;
        }
        blocks.swap(left, right);
    }
    let mut check_sum = 0;
    blocks.iter().enumerate().for_each(|(block_idx, block)| {
        if let None = block.file_id {
            return;
        }
        check_sum += block_idx * block.file_id.unwrap();
    });
    println!("{check_sum}");
}
#[derive(Clone)]
struct Region {
    is_file: bool,
    file_idx: Option<usize>,
    size: usize,
    start_idx: usize,
}

fn part2(data: &str) {
    let mut file_count: usize = 0;
    let mut curr_block_idx: usize = 0;
    let mut regions: Vec<Region> = vec![];
    data.chars().enumerate().for_each(|(map_idx, chr)| {
        if map_idx % 2 == 0 {
            regions.push(Region {
                is_file: true,
                file_idx: Some(file_count),
                size: chr.to_digit(10).unwrap() as usize,
                start_idx: curr_block_idx,
            });
            curr_block_idx += chr.to_digit(10).unwrap() as usize;

            file_count += 1;
        } else {
            regions.push(Region {
                is_file: false,
                file_idx: None,
                size: chr.to_digit(10).unwrap() as usize,
                start_idx: curr_block_idx,
            });
            curr_block_idx += chr.to_digit(10).unwrap() as usize;
        }
    });
    let mut left = 0;
    let mut right = regions.len() - 1;
    while left < right {
        if regions[left].is_file || regions[left].size == 0 {
            left += 1;
            continue;
        }
        if !regions[right].is_file || regions[right].size == 0 {
            right -= 1;
            continue;
        }
        let can_swap = regions.iter().enumerate().find(|(idx, region)| {
            idx < &right && !region.is_file && region.size >= regions[right].size
        });
        if let Some((left_swap_idx, left_swap_region)) = can_swap {
            let empty_region_to_populate_start_idx = regions[left_swap_idx].start_idx;
            let extra_space = left_swap_region.size - regions[right].size;
            let mut data_region = regions[right].clone();

            regions.remove(right);
            regions.insert(
                right,
                Region {
                    is_file: false,
                    file_idx: None,
                    size: data_region.size,
                    start_idx: data_region.start_idx,
                },
            );

            regions.remove(left_swap_idx);
            data_region.start_idx = empty_region_to_populate_start_idx;
            regions.insert(left_swap_idx, data_region.clone());
            if extra_space > 0 {
                regions.insert(
                    left_swap_idx + 1,
                    Region {
                        file_idx: None,
                        is_file: false,
                        size: extra_space,
                        start_idx: data_region.start_idx + data_region.size,
                    },
                );
            }
            left = 0;
            right = if extra_space > 0 { right } else { right - 1 };
        } else {
            left = 0;
            right -= 1;
        }
    }
    let mut check_sum = 0;
    regions.iter().for_each(|region| {
        if let None = region.file_idx {
            return;
        }
        for idx in region.start_idx..region.start_idx + region.size {
            check_sum += idx * region.file_idx.unwrap();
        }
    });
    println!("{check_sum}");
}
