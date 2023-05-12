# sqlite-bench-rs
Port of https://github.com/benbjohnson/sqlite-bench to Rust

## Examples
```
$ cargo run --release -- --help
Usage: sqlite-bench-rs [OPTIONS] <DB_FILE>

Arguments:
  <DB_FILE>  

Options:
      --journal-mode <JOURNAL_MODE>  [default: delete]
      --synchronous <SYNCHRONOUS>    [default: full]
      --batch-size <BATCH_SIZE>      [default: 1000]
      --batch-count <BATCH_COUNT>    [default: 1000]
      --row-size <ROW_SIZE>          [default: 100]
  -h, --help                         Print help
```
```
$ cargo run --release /tmp/test.db
Inserts:   1000000 rows
Elapsed:   2.017s
Rate:      495827.070 insert/sec
File size: 110993408 bytes
```
```
$ cargo run --release -- --journal-mode wal --batch-size 10000 --batch-count 100 /tmp/test.db
Inserts:   1000000 rows
Elapsed:   1.926s
Rate:      519343.061 insert/sec
File size: 110993408 bytes
```
