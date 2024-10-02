# Rust CLI: solana-list-epoch-boundaries

Just to list first and last slot in the epoch

## To run

```sh
cargo run -- -f 660 -l 670
```

# To SQL

```sh
cargo run -- -f 660 -l 670 | tee /tmp/blocks.txt

cat /tmp/blocks.txt | head -n -1 | sed 's/^\([0-9]\+\).*| \([0-9]\+\)  | \([0-9]\+\).*/-- epoch: \1\n--  and BLOCK_ID >= \2 and BLOCK_ID <= \3/'

cat /tmp/blocks.txt | head -n -1 | sed 's/^\([0-9]\+\).*| \([0-9]\+\)  | \([0-9]\+\).*/    WHEN BLOCK_ID >= \2 and BLOCK_ID <= \3 THEN \1/'
```
