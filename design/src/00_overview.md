# Overview

Louxna (pronounced /ˈloʊksna/) is a [LuaJIT](https://luajit.org/) runtime
and package manager.

Planned features:

- Decentralization of package registries
- Public key-based namespaces
- Hot reloading
- Hot swapping

## Example File Structure

```text
(Project root)
|- pkg.lx.toml
|- src/
    |- mod.lx.toml
    |- init.lua
    |- plane
        |- mod.lx.toml
        |- init.lua
        |- axis.lua
        |- vector.lua
```
