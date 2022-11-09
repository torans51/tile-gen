# Wang Tile Generator

Command line to to generate wang tiles

## Quick start

To generate a tile grid in the terminal with 10 rows and 10 cols run the following command
```
cargo run
```

To see the command line options available execute
```
cargo run -- -help
```

Example of custom output
```
cargo run -- \
    --rows 5 \
    --cols 5 \
    --symmetric \
    --output-format svg \
    --output-filename myfile 
```
