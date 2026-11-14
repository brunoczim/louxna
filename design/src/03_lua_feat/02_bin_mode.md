# Binary Mode

The Louxna has the binary mode for the main package of the project.
In Lua, it reflects as the following:

- The root module (`src/init.lua`) is the "executable"
- The module is executed with the variable `arguments`
    of the module's environment set to the CLI arguments, an array of strings
- The value exported by the module is the status code,
    either an integer or `nil`:
    - If returning an integer, it is the process's exit code
    - If returning `nil`, the process's exit code defaults to `0`
