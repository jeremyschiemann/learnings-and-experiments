use std::fs::File;
use std::io::Read;
use std::sync::mpsc;
use std::time::Instant;
use std::{io, thread};

fn main() -> io::Result<()> {
    println!("start");
    let core_count: usize = thread::available_parallelism()?.into();
    println!("Cores: {core_count}");

    let mut f = File::open("measurements.txt").unwrap();
    let mut buf: Vec<u8> = Vec::with_capacity(15 * 1024 * 1024 * 1024);
    f.read_to_end(&mut buf)?;
    buf.shrink_to_fit();
    let time = Instant::now();

    println!("file read");

    let (tx, rx) = mpsc::channel();
    thread::scope(|s| {
        let scope_tx = tx;

        let chunk_size = buf.len() / core_count;

        for chunk in buf.chunks(chunk_size) {
            let thread_tx = scope_tx.clone();
            let buf_chunnk = chunk;

            s.spawn(move || {
                let thread_counter = bytecount::count(buf_chunnk, b'\n');
                thread_tx.send(thread_counter).expect("oopsie");
            });
        }
    });

    let mut counter = 0;

    for r in rx {
        println!("adding {r} to {counter}");
        counter += r;
    }

    let elapsed = time.elapsed();

    println!("counted {counter} in {elapsed:?}");

    Ok(())
}
