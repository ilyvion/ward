## 2.0.0
### Improvements 🙌
* When using the `else` branch mechanism, its block no longer automatically returns. This should've
been the default from day one, but didn't occur to me until [issue #1][i1] brought it up. Since
doing it this way break backwards compatibility with the macros' usage in earlier versions, a
major version bump was required by semver rules.

## 1.1.0
### New ✨
* Added support for other early return statements aside from the until now enforced `return`.

## 1.0.2
### Improvements 🙌
* Added extra metadata to `Cargo.toml`

## 1.0.1
### Improvements 🙌
* Added code and crate documentation

## 1.0.0
* Initial release

[i1]: https://github.com/alexschrod/ward/issues/1
