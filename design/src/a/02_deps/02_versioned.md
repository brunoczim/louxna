# Step 2

## Goal

Introduces basic versioning for packages.

### Changes

- Add a `version = "<semver>"` field to `pkg.lx.toml`
- Only core versions of semver are supported:
    - `<core> ::= <major>.<minor>.<patch>` is supported
    - Pre-release will be supported soon
    - Build tags are not supported
- Dependencies require a `version = "<semver-spec>"` field now
- Dependencies' versions can be specified fully or omitting trailing components

## Status

Pending
