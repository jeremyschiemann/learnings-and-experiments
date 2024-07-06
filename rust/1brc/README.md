# One billion row challenge 

see https://github.com/gunnarmorling/1brc for an explanation.
tl;dr process 1 billion rows as fast as possible.


## Disclaimer 
I did use 3rd party crates as I only did this for myself to learn some rust and not to actually participate in the challenge



## Sequentiell approach

`cargo run --bin sequentiell`

This a naive sequential take on the challenge without any thoughts wasted on optimization.


## Parallel approach

`cargo run --bin parallel`

This is my somewhat optimized take and is obviously much faster.

Some optimizations i did:
* split workload in chunks and run in 1 thread per core (hence the bin name)
* dont parse to floats, parse to integer and devide by 10 at the end
* use `&[u8]` instead of `&str` and only parse to `&str` at the end.
* use `FxHashMap` in threads and combine into `BtreeMap` to automatically sort it.
* use `rfind` instead of `find` for finding the index of `;` because the city names are most likely longer than the temperature part.
* inline functions
* activate `lto` and some other release improvements (see [cargo.toml](./cargo.toml))


## Result
runs in around 2.5s to 3s including the file read compared to originally 600s

---
### PC
* Ryzen 5900X
* 32gb 3600MHz ram
* Samsung 980 pro
* Windows 10