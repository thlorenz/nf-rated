# nf_rated

## Steps

Insert data from provided CSV file:

```sh
cargo run --bin init_data
```

Get API key from [omdbapi](http://www.omdbapi.com/).

```sh
OMDB_KEY=<api key> cargo run --bin update_data
```

Run TUI app to find good shows :)

```sh
cargo run
```
