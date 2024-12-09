advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy)]
struct Block {
    start_offset: usize,
    size: usize,
    value: i64,
}

fn parse(input: &str) -> Vec<i64> {
    let mut file_id = 0;
    let mut emmpty_space = false;
    let disk: Vec<i64> = input
        .chars()
        .flat_map(|c| {
            let block_size: u32 = c.to_digit(10).unwrap();
            let mut block_content = file_id;
            if emmpty_space {
                block_content = -1;
            } else {
                file_id += 1;
            }
            emmpty_space = !emmpty_space;
            vec![block_content; block_size as usize]
        })
        .collect();
    disk
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk: Vec<i64> = parse(input);

    let mut dst_idx: usize = 0;
    let mut src_idx = disk.len() - 1;

    while dst_idx < src_idx {
        while disk[src_idx] == -1 {
            src_idx -= 1;
        }

        while disk[dst_idx] != -1 {
            dst_idx += 1;
        }

        if dst_idx >= src_idx {
            break;
        }
        disk.swap(dst_idx, src_idx);

        src_idx -= 1;
        dst_idx += 1;
    }

    let mut res = 0;
    for (x, block) in disk.into_iter().enumerate() {
        if block == -1 {
            break;
        }
        res += x as u64 * block as u64;
    }

    Some(res)
}

fn second_parse(input: &str) -> Vec<Block> {
    let mut file_id = 0;
    let mut start_offset: usize = 0;
    let mut empty_space = false;
    let disk: Vec<Block> = input
        .chars()
        .filter_map(|c| {
            let size: u32 = c.to_digit(10).unwrap();
            let mut value = file_id;
            if empty_space {
                value = -1;
            } else {
                file_id += 1;
            }
            empty_space = !empty_space;
            if size == 0 {
                None
            } else {
                let block = Block {
                    start_offset,
                    size: size as usize,
                    value,
                };
                start_offset += size as usize;
                Some(block)
            }
        })
        .collect();
    disk
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk: Vec<Block> = second_parse(input);

    let mut j = disk.len() - 1;

    //println!("{:?}", disk);

    while j > 0 {
        //println!("Index {} block {:?}", j, disk[j]);

        //if it's a file, let's try to find a free space
        if disk[j].value != -1 {
            for i in 0..disk.len() {
                if (i < j) && (disk[i].value == -1) && (disk[i].size >= disk[j].size) {
                    //println!("Found Index {} block {:?}", i, disk[i]);
                    let original_offset = disk[i].start_offset;
                    disk[i].start_offset = disk[j].start_offset;
                    disk[j].start_offset = original_offset;
                    disk.swap(i, j);
                    //now j contains the free space
                    if disk[j].size != disk[i].size {
                        let original_size = disk[j].size;
                        //resize the free space moved
                        //equal to file moved
                        disk[j].size = disk[i].size;

                        //insert the remaining free space after the file
                        disk.insert(
                            i + 1,
                            Block {
                                start_offset: disk[i].start_offset + disk[i].size,
                                size: original_size - disk[i].size,
                                value: -1,
                            },
                        );
                        j += 1;
                    }
                    break;
                }
            }
        }
        j -= 1;
    }

    //println!("{:?}", disk);

    Some(
        disk.iter()
            .filter_map(|b| {
                if b.value == -1 {
                    None
                } else {
                    let mut value: u64 = 0;
                    for i in b.start_offset..b.start_offset + b.size {
                        value += i as u64 * b.value as u64;
                        //println!("{}", value);
                    }
                    Some(value)
                }
            })
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
