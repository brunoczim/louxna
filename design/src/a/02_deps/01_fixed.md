# Step 1

## Goal

Introduce basic dependencies for Louxna projects.
In this step, all dependencies must be placed under a `deps/` directory.

## Changes

- Now `pkg.lx.toml` accepts a `dependencies` 
    - Dependencies are declared in the format
    `{ name = <name>, namespace = <namespace> }`
    in the array
- Packages directories are named in the format `deps/<namespace>/<name>/*`
- Each package will have a dedicated `require()` to find only dependencies

## Status

Pending
