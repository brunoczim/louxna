# Iteration 1

## Goal

Introduce the `pkg.lx.toml` file.

## Changes

- Now `<root_dir>` requires the file `pkg.lx.toml` file to exist
    (except for `init`)
- The `init` command initializes a directory without `pkg.lx.toml`
- The `pkg.lx.toml` is minimal, only requiring a `name`
- The lua script must have its module name prefixed in the chunk

## Status

Done
