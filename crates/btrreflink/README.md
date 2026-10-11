# `BeTteR REFLINK`

Cross-platform COW reflink copy of files

Based on <https://github.com/cargo-bins/reflink-copy>

(namesake BTRFS)

## Naming convention

for function name: `reflink_{src_kind}{dst_kind}`

with kinds:

- `d` - dir FD and path (equivalent to `*at` functions)
- `f` - file FD
- `p` - path
