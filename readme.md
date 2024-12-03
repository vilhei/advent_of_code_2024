# Advent of code rust template

## Usage

- Implement `Task` trait which has 2 functions `task_part_one` and `task_part_two`.
- Build and run the cargo project with following command giving the day as argument to the built binary.

### Running tasks

```bash
cargo run -- <day>
```
If `Task` for given day has been implemented the results are printed out on the console. 

### Running benchmarks

Run all benches
```bash
cargo bench -- --verbose
```

Run single day both parts
```bash
cargo bench -- day_<day> --verbose
cargo bench -- day_4 --verbose #example
```

Rune part n for single day
```bash
 cargo bench -- day_<day>__part<n> --verbose
 cargo bench -- day_4__part2 --verbose
```



