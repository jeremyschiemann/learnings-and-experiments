use std::fs::File;
use std::io::{Read, Write};

use fxhash::FxHashMap;
use std::fmt::{Debug, Formatter};
use std::time::Instant;
use std::{io, iter, thread};

struct StationData {
    min: i64,
    max: i64,
    count: usize,
    sum: i64,
}

impl StationData {
    fn add(&mut self, new_val: i64) {
        self.max = self.max.max(new_val);
        self.min = self.min.min(new_val);
        self.count += 1;
        self.sum += new_val;
    }

    fn merge(&mut self, other: &Self) {
        self.max = self.max.max(other.max);
        self.min = self.min.min(other.min);
        self.count += other.count;
        self.sum += other.sum;
    }
}

impl From<i64> for StationData {
    fn from(value: i64) -> Self {
        Self {
            min: value,
            max: value,
            sum: value,
            count: 1,
        }
    }
}

impl Debug for StationData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "min/max/avg: {:.1}/{:.1}/{:.1}",
            self.min as f64 / 10.0,
            self.max as f64 / 10.0,
            (self.sum as f64 / self.count as f64) / 10.0
        )
    }
}

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn get_station_name_and_temperature(line: &[u8]) -> Result<(&[u8], i64)> {
    let split_point = line.iter().rposition(|&b| b == b';').ok_or_else(|| {
        format!(
            "no ; found in {}",
            std::str::from_utf8(line).unwrap_or("invalid utf8")
        )
    })?;

    let mut temp: i64 = 0;
    let sign_multiplier = if line[split_point + 1] == b'-' { -1 } else { 1 };
    let offset = if sign_multiplier == 1 { 1 } else { 2 };

    for b in line[split_point + offset..].iter().filter(|&b| *b != b'.') {
        temp = temp * 10 + (b - b'0') as i64;
    }

    Ok((&line[..split_point], temp * sign_multiplier))
}

fn read_file(buffer: &mut Vec<u8>) -> Result<()> {
    print!("reading file...");
    io::stdout().flush()?;
    let mut f = File::open("measurements.txt")?;
    f.read_to_end(buffer)?;
    println!("finished!");
    buffer.shrink_to_fit();
    Ok(())
}

fn main() -> Result<()> {
    let prog_start = Instant::now();

    println!("start");
    let core_count: usize = thread::available_parallelism()?.into();
    println!("Cores: {core_count}");

    //let buffer = get_mmap()?;

    let mut buffer = Vec::with_capacity(15 * 1024usize.pow(3));
    read_file(&mut buffer)?;

    let time = Instant::now();

    let buffer_size = buffer.len();

    let chunk_size = buffer_size / core_count;

    let starting_points: Vec<usize> = iter::once(0usize)
        .chain({
            (1..core_count)
                .map(|core| {
                    let temp_start = core * chunk_size;
                    let buffer = &buffer[temp_start..];

                    temp_start + buffer.iter().position(|&b| b == b'\n').unwrap_or(0) + 1
                })
                .collect::<Vec<usize>>()
        })
        .collect();

    let end_points = starting_points[1..]
        .iter()
        .copied()
        .chain(iter::once(buffer.len()))
        .collect::<Vec<usize>>();

    thread::scope(|s| {
        let mut handles = Vec::with_capacity(core_count);
        for (sp, ep) in iter::zip(starting_points, end_points) {
            let thread_buffer = &buffer[sp..ep];

            handles.push(s.spawn(move || {
                let mut map: FxHashMap<_, StationData> =
                    FxHashMap::with_capacity_and_hasher(10000, fxhash::FxBuildHasher::default());

                thread_buffer
                    .split(|&b| b == b'\n')
                    .filter(|&l| !l.is_empty())
                    .filter_map(|line| get_station_name_and_temperature(line).ok())
                    .for_each(|(name, temp)| {
                        map.entry(name)
                            .and_modify(|entry| entry.add(temp))
                            .or_insert(temp.into());
                    });

                map
            }));
        }

        let mut map: FxHashMap<_, StationData> =
            FxHashMap::with_capacity_and_hasher(10000, fxhash::FxBuildHasher::default());

        for handle in handles {
            for (key, entry) in handle.join().unwrap_or_else(|_| panic!("bla")) {
                map.entry(key)
                    .and_modify(|data| data.merge(&entry))
                    .or_insert(entry);
            }
        }

        println!(
            "{:?} in {:?} (whole run took {:?})",
            FxHashMap::from_iter(map.into_iter().map(|(key, data)| {
                (
                    std::str::from_utf8(key).unwrap_or_else(|e| panic!("{}", e.to_string())),
                    data,
                )
            })),
            time.elapsed(),
            prog_start.elapsed(),
        );
    });

    Ok(())
}
