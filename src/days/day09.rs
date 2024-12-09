use std::collections::BinaryHeap;
use std::fmt::Debug;

use crate::{read_input, Solution, SolutionPair};

#[derive(Clone, Copy, Debug)]
enum DiskType {
    FILE(usize),
    FREE,
}

#[derive(Clone, Debug)]
struct Disk {
    disk_type: DiskType,
    start: usize,
    size: usize,
}

fn parse_input() -> Vec<Disk> {
    let mut i_cur = 0;
    read_input!(09)
        .trim()
        .bytes()
        .map(|c| c - 48)
        .enumerate()
        .map(|(i, size)| {
            let size = size as usize;
            let disk_type = if i % 2 == 0 {
                DiskType::FILE(i / 2)
            } else {
                DiskType::FREE
            };
            let start = i_cur;
            i_cur += size;
            Disk {
                disk_type,
                start,
                size,
            }
        })
        .collect()
}

fn sum_interval(start: usize, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        start * len + ((len - 1) * len) / 2
    }
}

fn part1(mut disks: Vec<Disk>) -> usize {
    let mut result = 0;
    let mut i_disk = if disks.len() == 0 {
        disks.len() - 2
    } else {
        disks.len() - 1
    };
    let mut i = 0;
    while i <= i_disk {
        let (disks1, disks2) = disks.split_at_mut(i + 1);
        let disk = &mut disks1[i];
        let checksum = match disk.disk_type {
            DiskType::FILE(val) => {
                let checksum = val * sum_interval(disk.start, disk.size);
                i += 1;
                checksum
            }
            DiskType::FREE => {
                let fill_disk = &mut disks2[i_disk - i - 1];

                let size = if disk.size > fill_disk.size {
                    i_disk -= 2;
                    fill_disk.size
                } else {
                    i += 1;
                    disk.size
                };

                let val = match fill_disk.disk_type {
                    DiskType::FILE(val) => val,
                    DiskType::FREE => 0, // cannot happen
                };

                let checksum = val * sum_interval(disk.start, size);
                disk.start += size;
                disk.size -= size;
                fill_disk.size -= size;

                checksum
            }
        };

        result += checksum;
    }

    result
}

fn part2(mut disks: Vec<Disk>) -> usize {
    let mut result = 0;

    let max_size = disks.iter().map(|disk| disk.size).max().unwrap_or(0);
    let mut heaps = vec![BinaryHeap::new(); max_size + 1];
    for disk in &disks {
        match disk.disk_type {
            DiskType::FILE(val) => {
                heaps[disk.size].push((val * 2, disk.start));
            }
            DiskType::FREE => {}
        }
    }

    let mut i = 0;
    while i < disks.len() {
        let (disks1, disks2) = disks.split_at_mut(i + 1);
        let disk = &mut disks1[i];
        let checksum = match disk.disk_type {
            DiskType::FILE(val) => {
                let checksum = val * sum_interval(disk.start, disk.size);
                i += 1;
                checksum
            }
            DiskType::FREE => {
                let i_disk = match find_fill_disk(disk.start, disk.size, &mut heaps) {
                    Some(i) => i,
                    None => {
                        i += 1;
                        continue;
                    }
                };
                let fill_disk = &mut disks2[i_disk - i - 1];

                let size = fill_disk.size;
                let val = match fill_disk.disk_type {
                    DiskType::FILE(val) => val,
                    DiskType::FREE => 0, // cannot happen
                };

                let checksum = val * sum_interval(disk.start, size);
                disk.start += size;
                disk.size -= size;
                fill_disk.size = 0;

                checksum
            }
        };

        result += checksum;
    }

    result
}

fn find_fill_disk(
    start: usize,
    size: usize,
    heaps: &mut [BinaryHeap<(usize, usize)>],
) -> Option<usize> {
    let mut disks = vec![None; size];
    for size in 1..=size {
        let heap = &mut heaps[size];
        while let Some(disk) = heap.pop() {
            if disk.1 > start {
                disks[size - 1] = Some(disk);
                break;
            }
        }
    }

    let mut index = None;
    if let Some(max) = disks
        .iter()
        .filter(|disk| disk.is_some())
        .map(|disk| disk.unwrap().0)
        .max()
    {
        for size in 1..=size {
            if let Some(disk) = disks[size - 1].take() {
                if disk.0 == max {
                    index = Some(disk.0);
                } else {
                    heaps[size].push(disk);
                }
            }
        }
    }

    index
}

pub fn solve() -> SolutionPair {
    let disks = parse_input();

    let sol1 = part1(disks.clone());
    let sol2 = part2(disks);

    (Solution::from(sol1), Solution::from(sol2))
}
