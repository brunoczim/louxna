# Desgin Principles

## Embedding-First

Louxna does have a `run` module, which executes the root module of the package
as an executable, providing CLI arguments as an array of strings to the
`arguments` variable,
using the returned integer as exit status of the process (defaulting to `0`).
This is the binary mode.

The binary mode, however, is a secondary feature.
The focus of Louxna is, instead, embedding Lua into Rust projects
with a package manager and an opinionated runtime.

## Sandboxing Best-effort

As embedding Lua is a priority for Louxna,
sandboxing is desired.
Louxna has best-effort sandboxing features,
such as re-implementing `require()` to prevent filesystem operations
and giving an option to isolate globals per-module.
There are also some measures that help on sandboxing that
Louxna adopts by default,
such as cutting access to I/O standard libraries
and overriding of `print()` (todo).

### Security Notice

It should be clear that these sandboxing features are best-effort.
You should not assume that you would be automatically secure
only because you use these features,
even though security improvements are expected.
Despite that,
you can still rely on these features for purposes of functional correctness.

## Low Friction for Internal Modules

If a package has no dependencies,
then using Louxna should have little to no friction.
Internal module imports should be able to be performed
without changes, in the format `abc.xyz.foobar`.

If you have an existing Lua project that has no dependencies,
it is pretty probable that it will work by just creating a directory,
opening it,
running `louxna init --name <> --namespace <>`,
and then copying all your modules to `src/`.

## No Dependency Hoisting

If a package has not declared a dependency,
even if another package declares it,
the first won't be able to import such dependency.
This implies that dependency resolution is isolated between modules.

## Promotion of Dependency Auditability

Louxna promotes principles we believe that make dependencies more auditable
in comparison to other package managers, such as Cargo and npm.
This includes:

- Decentralized registries (todo)
    - Promotes less dependencies
    - Promotes open source packages
    - Promotes being closer to source code of dependencies
    - Promotes vendoring of dependencies
- Namespaced packages with public key distribution (todo public key)
    - Promotes cohesion of multiple related packages
    - Promotes integrity key pairs scoped
        to a single namespace or a small set of namespaces
- Allowed overriding of any dependency by top-level package or host (todo)
    - Promotes freedom for the end-user software to patch any
        security vulnerabilities
    - Promotes freedom for the end-user software to force a trusted version
