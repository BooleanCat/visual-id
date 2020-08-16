# visual-id (BotsAndUs tech test)

**This is a tech test submission for BotsAndUs' interview.**

The tools's usage is described below in the building section, however more
detailed usage information is available by running `visual-id --help` after
building.

PNG files are output to a local directory named "output" which must be created
by you before running the tool. If I spent more time on this I would have the
tool create this if it didn't exist and make the output directory
confirgurable.

Given that producing a few more test PNGs by hand would be time consuming, I
skipped this but would do so for production code and would have some
acceptance tests (probably using Lua's busted BDD framework) to run the
compiled binary as part of running tests. I chose not to for this submission
to save my time and your hassle installing more dependencies.

## Requirments

- Rust
- Cargo

## Testing

```bash
$ cargo test
```

# Building

```bash
$ cargo build --release
$ ./target/release/visual-id ids.txt  # produce visual ids for test data
```

# Notes

The supplied visual-id for 1337.png is not (as per the spec) using a one-bit
colour depth but an eight-bit colour depth. For this reason I was unable to use
the suppled png for testing image.rs and had to produce my own (the test
was failing because headers were different).