# Dirty implementation of global sequence alignment

My implementation of global alignment of two sequences.
I did not consult any literature until I completed my solution. It appears similar to Needleman-Wunsch.

It is far from optimal. Modern solutions will perform much better.

## Example

Below is the print output of `problem_set::eddy().solve()`.
Complete paths refers to candidate paths that make it to (M, N) of the scoring matrix.
```

Total number of paths: 79022
Total number of complete paths: 34775

=== Solution ===

Optimal score: 11
One solution
(0, 0), (1, 1), (1, 2), (1, 3), (2, 4), (3, 5), (4, 6), (5, 7), (6, 8)
[[0, -6, -12, -18, -24, -30, -36, -42, -48],
 [-6, 5, -1, -7, -13, -19, -25, -31, -37],
 [-12, -1, 3, -3, -2, -8, -14, -20, -26],
 [-18, -7, -3, 8, 2, 3, -3, -9, -15],
 [-24, -13, -9, 2, 6, 0, 1, -5, -4],
 [-30, -19, -15, -4, 7, 4, -2, 6, 0],
 [-36, -25, -21, -10, 1, 5, 2, 0, 11]]
END
```

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
