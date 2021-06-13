# Triax Rust Template

This repository serves as a template for creating any other Rust projects for Triax.

## Creating a New Project

1. Create the new repository [from this template][using-templates].
2. If the project will only be a library:
    1. Delete `src/main.rs`
    2. Delete `.github/workflows/release.yml`
3. Change the project name in:
    1. `README.md`
    2. `Cargo.toml`
    3. (Optional) `.github/workflows/release.yml`
4. Update `README.md` with information for the new project.

[using-templates]: https://docs.github.com/en/github/creating-cloning-and-archiving-repositories/creating-a-repository-from-a-template

## Release Checklist

1. Create a pull request to `main`.
2. Increase version number according to [SemVer][semver]

[semver]: https://doc.rust-lang.org/cargo/reference/semver.html
