# rust-github-actions

Some simple GitHub action workflows for checking, testing/building, and releasing rust crates.

## Files

- Build debug: [build.yml](.github/workflows/build.yml)
- Checks: [checks.yml](.github/workflows/checks.yml)
- Build release and create release: [release.yml](.github/workflows/release.yml)

## Downloads

- build.yml
  ```shell
  mkdir -p .github/workflows && curl -fsSL https://github.com/harmless-tech/rust-github-actions/raw/main/.github/workflows/build.yml > .github/workflows/build.yml
  ```
  or
  ```shell
  wget -P .github/workflows https://github.com/harmless-tech/rust-github-actions/raw/main/.github/workflows/build.yml
  ```

- checks.yml
  ```shell
  mkdir -p .github/workflows && curl -fsSL https://github.com/harmless-tech/rust-github-actions/raw/main/.github/workflows/checks.yml > .github/workflows/checks.yml
  ```
  or
  ```shell
  wget -P .github/workflows https://github.com/harmless-tech/rust-github-actions/raw/main/.github/workflows/checks.yml
  ```

- release.yml
  ```shell
  mkdir -p .github/workflows && curl -fsSL https://github.com/harmless-tech/rust-github-actions/raw/main/.github/workflows/release.yml > .github/workflows/release.yml
  ```
  or
  ```shell
  wget -P .github/workflows https://github.com/harmless-tech/rust-github-actions/raw/main/.github/workflows/release.yml
  ```

### Examples

- [cargo-prebuilt](https://github.com/crow-rest/cargo-prebuilt)
