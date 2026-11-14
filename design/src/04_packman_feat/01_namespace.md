# Namespaces

Packages always exist within a namespace.
When only one package name exists as a dependency,
the dependent can import it without qualifying the namespace,
e.g. import `foobar::` instead of `mynamespace/foobar::`.
If there is conflict, however, one dependency can be declared as the "main"
package that has the given package name and can still be imported
without qualifying the namespace,
but the other packages need namespace qualification.

The namespace of a package is declared within its `pkg.lx.toml`, example:

```toml
name = "geometry"
namespace = "my_cool_engine"
version = "1.2.3"
```

It is planned that each namespace will have a unique public key used to
verify the signatures of its packages.
