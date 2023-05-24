# BrickSet API

This crate provides utilities for working with the [BrickSet API](https://brickset.com/article/52666/brickset-web-services).
This includes:

- Low-level tools for building API requests, and parsing API responses.
- High-level client wrapper for [reqwest](https://docs.rs/reqwest/)

# Features

- `log` (default): Generate log messages using the [log](https://docs.rs/log/) crate.
- `reqwest` (default): High-level wrapper for [reqwest](https://docs.rs/reqwest/). If
  you aren't using reqwest, you should disable this feature.

# Examples

The examples in this crate require a couple of environment variables:

```sh
export BRICKSET_KEY="<your BrickSet API key>"
export BRICKSET_USERNAME="<your BrickSet username>"
```

## get_wanted_sets

`get_wanted_sets` uses the high-level reqwest-based API to retrieve a BrickSet user's
wantlist.

```
cargo run --example get_wanted_sets
```

## get_wanted_sets_low

`get_wanted_sets_low` does the same thing as `get_wanted_sets`, but doesn't use
the high-level wrapper.
