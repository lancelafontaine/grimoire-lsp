# Contribution guidelines

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Reporting Issues

Before reporting an issue on the
[issue tracker](https://github.com/lancelafontaine/grimoire-lsp/issues),
please check that it has not already been reported by searching for some related
keywords.

## Getting Started

### Building & Running

```shell
cargo build --release && cargo run --release
```

### Testing

```shell
cargo test --all-features --workspace
```

### Linting

```shell
cargo clippy --all-targets --all-features --workspace
  ```

### Formatting

```shell
cargo fmt --all
```

## Updating the Changelog

Once you have a PR to submit, also update the changes you have made in
[CHANGELOG](https://github.com/lancelafontaine/grimoire-lsp/blob/main/CHANGELOG.md)
file under the **Unreleased** section.

Your PR will likely fall under one of the following types as defined by
[Keep a changelog](https://keepachangelog.com/en/1.0.0/):

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.
