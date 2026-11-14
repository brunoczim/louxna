# Modules

Each Louxna package has a set of hierarchical Lua modules.
Within a single package, module resolution works as in standard Lua.
You import it with a dot-separated path:
`""`, `"foo"`, `"foo.bar"`, `"foo.bar.bazz"`, ...
The main difference is that Louxna accept an empty string `""` as the root
module of the package.

The file name for the top-level module of a directory is named `init.lua`.
Thus `src/init.lua` is imported as `""`,
while `src/foo/bar/init.lua` is imported as `"foo.bar"`.
The file paths `src/foo/bar.lua` and `src/foo/bar/init.lua` are equivalent,
both are imported as `"foo.bar"`,
but you can only define one of the two,
defining both results in error.

## Dependency Imports

To import a dependency, use a path prefixed with `<package>::<modpath>`.
For instance, to import the `prng.xorshift` module of the `rand` dependency,
import it with `"rand::prng.xorshift"`.
To import the root module, just use `"rand::"` (note the trailing two colons).
In case you have multiple `rand` packages of multiple namespaces,
you can prefix the namespace as `"mynamespace/rand::prng.xorshift"`.

## Custom Require

Each module gets its own `require(modid)` function.
Therefore, one package can only import its own modules
or the modules of its own dependencies.
A package cannot import a dependency it does not declare.

## Module Caching and Require Arguments

The `require()` function accepts a second argument,
either `nil` or a table.
When a table is given,
the module is invoked with the table stored in the `arguments` variable of the
imported module's environment.

Usually, only the first import of a module actually executes the module.
After the first call, the exported value is cached and further imports
simply return this cached value.
However, if the `require()` function is called with a table as the second
argument,
then the module is re-executed with a new `arguments` variable in the
environment of the imported module.
The new exported value is cached.

Example of calling `require()` with module arguments:

```lua
local util = require("my_module.util", { log = true })
```

## Circular Imports

For calls to `require()` that happen at the top-level chunk of a module,
it is not possible to make circular imports.
That would result in an error such as stack overflow.
To prevent that, what you can do is calling `require()` inside of a function,
for instance, in case circular imports are necessary.

## High-order Require

While `require()` is per-package, it is just a function,
and functions in Lua are first-class citizens.
Given that, it is possible for one package to pass its `require()` to another
e.g. as a parameter of a function.
Those are high-order imports.
This enables a limited macro-like behavior.

For instance, a package can export a function that accepts as one of its
arguments the `require()` of the caller, which must be passed explicitly.
Since this is in the control of the caller, a custom function can also be given.
Then, the callee can "pretend" it is the caller or some other package,
and compute imports in behalf of.

Example of such a utility:

```lua
-- under package foobar, module util

local function normalize_init(req)
    return function(modid, arguments)
        local pos = modid:find("(\\.|^)init$")
        if pos ~= nil then
            modid = modid:sub(1, pos - 1)
        end
        return req(modid, arguments)
    end
end

return {
    normalize_init = normalize_init,
}
```

Example of usage of the utility:

```lua
-- note it is being imported from another package
local util = require("foobar::util")

local patched_require = util.normalize_init(require)

-- note that the module paths here are internal to the current package
assert(patched_require("init") == require(""))
```
