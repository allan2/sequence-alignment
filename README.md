# Dirty implementation of global sequence alignment

My implementation of global alignment of two sequences.
I did not consult any literature until I completed my solution. It appears similar to Needleman-Wunsch.

It is far from optimal. Modern solutions will perform much better.

## Features

- builds the scoring matrix
- finds the optimal score
- finds all optimal paths

## Methodology

1. Construct the scoring matrix of size _M x N_.
2. Record candidate paths during matrix construction.
3. Employ reverse traversal to find optimal paths.

When collecting candidates, I eliminated paths using position only.
For example, an incomplete path (0,0) becomes outdated when you have traversed to (2, 1) using row-major order.

### Potential Improvements

Lots of ways ;)

## Setup

1. [Install Rust](https://www.rust-lang.org/tools/install).
2. To get started quickly:

```
cargo run
```

The release build is faster than the debug build. To build it:

```
cargo build --release
```

You can then run the executable that was just created at _*target/release/seq_align*_.
