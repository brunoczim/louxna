# Module Isolation

A package can request isolation of the global environment.
When this is turned on (off by default),
changes to the global environment are not actually performed on the global
environment.

You can enable it in `pkg.lx.toml`:

```toml
# ...
isolated-modules = true
```

Note that the isolation is shallow, if an entry of the global table is also a
table,
it will be the same object and it can be mutated even if this feature is on.

## Security Notice

Do not rely on this feature for security purposes.
This feature is realized through metatables and can be bypassed.
