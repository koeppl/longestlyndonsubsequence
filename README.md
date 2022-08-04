# Computing Longest Lyndon Subsequences

This tool computes the longest Lyndon subsequence found in a given text file.

## build
You can easily compile the tool with a recent version of `cargo`.

```console
cargo build --release
./target/release/longestlyndonsubseq -f <FILENAME> [-p prefixlength]
```

For benchmarking, you can use the file `run.sh` after you have specified the datasets (hard-coded) to use.

## Caveats
The program is quite slow, so do not expect to obtain immediate results for string lengths > 300.

## References
- Hideo Bannai, Tomohiro I, Tomasz Kociumaka, Dominik Köppl, Simon J. Puglisi: 
[Computing Longest (Common) Lyndon Subsequences](https://doi.org/10.1007/978-3-031-06678-8_10). IWOCA 2022: 128-142
