# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Deprecated

### Removed

### Fixed

## [0.5.0] - 2023-12-20

### Changed
- Updated to SOFA release 2023-10-11 ([PR 3](https://github.com/duncaneddy/rsofa/pull/3), Credit: @strizel)
- Updated `build.rs` to use glob to grab all c-files to remove need to manually add files to bindings if more SOFA routines are added in the future
- Remove hard-coded unit tests from `lib.rs` for specific routines. Coverage should be automated and complete, or not present to highlight testing gap.

## [0.4.5] - 2021-12-28

### Fixed
- Fixed bad documentation link

## [0.4.4] - 2021-12-28

### Added
- Updated crate `Cargo.toml` with link to documentation and adding crate keywords. 

## [0.4.3] - 2021-12-28

### Added
- Added `CHANGELOG.md` to track release changes

### Removed
- Removed build dependency on `rustfmt` which may be failing doc build

### Fixed
- Get module-wide allowance of improper ctypes to fix improper module usage.