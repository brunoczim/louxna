# Specifying Dependecies

Dependencies are specified in the `dependencies` array of the `pkg.lx.toml`
file.
Each element of the list is in the format:

```toml
{
    name = "<package name>",
    namespace = "<package namespace>",
    version = "<package version>",
}
```

Example `pkg.lx.toml`:

```toml
name = "my_game"
namespace = "my_namespace"
version = "0.1.0"
dependencies = [
    {
        name = "geometry",
        namespace = "some_engine",
        version = "1.2.3",
    },
    {
        name = "graphics",
        namespace = "some_engine",
        version = "1.2.4",
    },
]
```

## Version Requirements

The version of a dependency can be specified in multiple ways.
For instance, you may specify only `major`, only `major.minor`,
or full `major.minor.patch`.
Besides, even if you specify an exact/full version,
the actual version might be another version,
compatible with the type of version requirement.

### Default Version Requirement Type

The default version requirement type,
which is specified without any special annotation,
allows Louxna to change the version to a semver-compatible version.
E.g. when you specify `1.2.3`, valid replacements are `1.2.2`, `1.2.4`,
`1.3.0`, `1.59.333`, etc; but never `1.1.0` or `2.0.0`.

### Tilde Version Requirement

When a version requirement is prefixed with a tilde (e.g. `~1.2.3`),
then Louxna is only allowed to change the patch component of the version.
You need to specify at least `~major.minor` when using tilde requirements.
E.g. when you specify `~1.2.3`, valid replacements are `1.2.2`, `1.2.4`, etc;
but never `1.3.0`, `1.59.333`, `1.1.0` or `2.0.0`.

### Exact Version Requirement

When a version requirement is prefixed with an equal sign (e.g. `=1.2.3`),
then Louxna is never allowed to change the version.
You need to specify full `=major.minor.patch` version when using exact
requirements.
E.g. when you specify `=1.2.3`, no replacement is valid.

## Pre-releases

A package version may be specified as pre-release,
either `alpha`, `beta` or `rc`,
e.g. `1.2.3-alpha`, `1.2.3-alpha.9`, `1.2.3-beta`, `1.2.3-beta.5`,
`1.2.3-rc`, `1.2.3-rc.123`.
A pre-release version must always match exactly,
and it is usually not recommended.
If you specify the version of the dependency as pre-release,
then you must specify the version completely,
and modifiers such as tilde (`~`) or exact (`=`) do not apply,
it is always treated as an exact version.
If you specify a pre-release, that exact version is picked.
A pre-release version is never chosen by Louxna,
unless you explicitly specified it.
