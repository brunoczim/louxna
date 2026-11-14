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

Example import:

```lua
-- External, just package name
local tempo = require("moontoon::")

-- External, package name and module
local tempo = require("moontoon::music.tempo")

-- External, with namespace
local tempo = require("brunoczim/moontoon::")

-- External, with everything
local tempo = require("brunoczim/moontoon::music.tempo")

-- Internal
local tempo = require("music.tempo")
```

## Example Package Configuration

```toml
name = "moontoon"
namespace = "brunoczim"
version = "1.0.0"
dependencies = [
    {
        namespace = "brunoczim",
        name = "rand",
        version = "0.1.0",
    },
    {
        namespace = "foobar",
        name = "someutil",
        version = "0.9.3",
    },
]
```
