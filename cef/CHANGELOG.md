# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [141.0.0+141.0.5](https://github.com/tauri-apps/cef-rs/compare/cef-v140.3.6+140.1.14...cef-v141.0.0+141.0.5) - 2025-10-16

### Other

- *(release)* update CEF version to 141.0.5

## [140.3.6+140.1.14](https://github.com/tauri-apps/cef-rs/compare/cef-v140.3.5+140.1.14...cef-v140.3.6+140.1.14) - 2025-10-14

### Other

- release v140.3.6+140.1.14
- update bindings

## [140.3.5+140.1.14](https://github.com/tauri-apps/cef-rs/compare/cef-v140.3.4+140.1.14...cef-v140.3.5+140.1.14) - 2025-10-13

### Fixed

- add commas to fn new parameters ([#239](https://github.com/tauri-apps/cef-rs/issues/239))

### Other

- release v140.3.5+140.1.14
- update bindings

## [140.3.4+140.1.14](https://github.com/tauri-apps/cef-rs/compare/cef-v140.3.3+140.1.14...cef-v140.3.4+140.1.14) - 2025-10-13

### Other

- release v140.3.4+140.1.14
- update bindings
- update bindings
- update bindings

## [140.3.3+140.1.14](https://github.com/tauri-apps/cef-rs/compare/cef-v140.3.2+140.1.14...cef-v140.3.3+140.1.14) - 2025-10-11

### Fixed

- do not impl Default for structs with methods ([#225](https://github.com/tauri-apps/cef-rs/issues/225))

### Other

- release v140.3.3+140.1.14
- update bindings

## [140.3.2+140.1.14](https://github.com/tauri-apps/cef-rs/compare/cef-v140.3.1+140.1.14...cef-v140.3.2+140.1.14) - 2025-10-11

### Fixed

- windows build with wgpu@27
- macos build with wgpu@27
- macos osr texture handling now builds correctly
- macos metal texture fetching wrongly used macro and variables
- *(macos)* revert io_surface handle creation to original example, fix improper hal vs non-hal device usage
- (last attempt) macos accelerated rendering implementation
- (attempt) macos accelerated rendering implementation
- convert iostream implementation to previous one, since the original impl is broken
- attempt macos impl fix
- patch removed essential dependency in linux
- Windows accelerated paint now running
- modify osr_texture_import and example code for its new location

### Other

- release v140.3.2+140.1.14
- cleanup dependencies
- cargo fmt
- upgrade wgpu to ^26
- throw compile error if accelerated_osr requested on unsupported platform
- fix cargo fmt
- clean up import_via_metal code into closures
- remove unused objc2-metal dependency
- fix format
- attempt macos build error fix
- fix macos missing dependencies
- (attempt) fix windows build errors
- fix create proper docstring for mod.rs
- describe the accelerated_osr feature flag in Cargo.toml
- move osr_texture_import onto main cef crate

## [140.3.1+140.1.14](https://github.com/tauri-apps/cef-rs/compare/cef-v140.3.0+140.1.14...cef-v140.3.1+140.1.14) - 2025-10-03

### Fixed

- copy wrapped out-params back to pointers ([#224](https://github.com/tauri-apps/cef-rs/issues/224))

### Other

- release v140.3.1+140.1.14
- update bindings
- *(test)* test out-params

## [140.3.0+140.1.14](https://github.com/tauri-apps/cef-rs/compare/cef-v140.2.0+140.1.14...cef-v140.3.0+140.1.14) - 2025-09-23

### Other

- release

## [140.2.0+140.1.14](https://github.com/tauri-apps/cef-rs/compare/cef-v140.1.0+140.1.13...cef-v140.2.0+140.1.14) - 2025-09-21

### Other

- *(release)* update CEF version to 140.1.14

## [140.1.0+140.1.13](https://github.com/tauri-apps/cef-rs/compare/cef-v140.0.0+140.1.13...cef-v140.1.0+140.1.13) - 2025-09-19

### Other

- release

## [140.0.0+140.1.13](https://github.com/tauri-apps/cef-rs/compare/cef-v139.8.0+139.0.40...cef-v140.0.0+140.1.13) - 2025-09-19

### Other

- update bindings
- *(release)* update CEF version to 140.1.13

## [139.8.0+139.0.40](https://github.com/tauri-apps/cef-rs/compare/cef-v139.7.2+139.0.38...cef-v139.8.0+139.0.40) - 2025-09-12

### Other

- *(release)* update CEF version to 139.0.40

## [139.7.2+139.0.38](https://github.com/tauri-apps/cef-rs/compare/cef-v139.7.1+139.0.38...cef-v139.7.2+139.0.38) - 2025-09-08

### Fixed

- cleanup logic for copying back out-params
- handle out-params ([#173](https://github.com/tauri-apps/cef-rs/issues/173))

### Other

- release v139.7.2+139.0.38
- update bindings

## [139.7.1+139.0.38](https://github.com/tauri-apps/cef-rs/compare/cef-v139.7.0+139.0.38...cef-v139.7.1+139.0.38) - 2025-09-07

### Other

- release v139.7.1+139.0.38
- *(deps)* update rust crate windows-sys to 0.61

## [139.7.0+139.0.38](https://github.com/tauri-apps/cef-rs/compare/cef-v139.6.0+139.0.37...cef-v139.7.0+139.0.38) - 2025-08-31

### Other

- *(release)* update CEF version to 139.0.38

## [139.6.0+139.0.37](https://github.com/tauri-apps/cef-rs/compare/cef-v139.5.0+139.0.30...cef-v139.6.0+139.0.37) - 2025-08-29

### Other

- *(release)* update CEF version to 139.0.37

## [139.5.0+139.0.30](https://github.com/tauri-apps/cef-rs/compare/cef-v139.4.0+139.0.28...cef-v139.5.0+139.0.30) - 2025-08-28

### Other

- *(release)* update CEF version to 139.0.30

## [139.4.0+139.0.28](https://github.com/tauri-apps/cef-rs/compare/cef-v139.3.0+139.0.26...cef-v139.4.0+139.0.28) - 2025-08-23

### Other

- *(release)* update CEF version to 139.0.28

## [139.3.0+139.0.26](https://github.com/tauri-apps/cef-rs/compare/cef-v139.2.1+139.0.23...cef-v139.3.0+139.0.26) - 2025-08-22

### Other

- *(release)* update CEF version to 139.0.26

## [139.2.1+139.0.23](https://github.com/tauri-apps/cef-rs/compare/cef-v139.2.0+139.0.23...cef-v139.2.1+139.0.23) - 2025-08-16

### Fixed

- warnings about usize < 0 comparisons

### Other

- release

## [139.2.0+139.0.23](https://github.com/tauri-apps/cef-rs/compare/cef-v139.1.0+139.0.20...cef-v139.2.0+139.0.23) - 2025-08-16

### Other

- *(release)* update CEF version to 139.0.23

## [139.1.0+139.0.20](https://github.com/tauri-apps/cef-rs/compare/cef-v139.0.1+139.0.17...cef-v139.1.0+139.0.20) - 2025-08-15

### Other

- *(release)* update CEF version to 139.0.20

## [139.0.1+139.0.17](https://github.com/tauri-apps/cef-rs/compare/cef-v139.0.0+139.0.17...cef-v139.0.1+139.0.17) - 2025-08-08

### Other

- release v139.0.1+139.0.17

## [139.0.0+139.0.17](https://github.com/tauri-apps/cef-rs/compare/cef-v138.9.0+138.0.36...cef-v139.0.0+139.0.17) - 2025-08-08

### Other

- *(release)* update CEF version to 139.0.17

## [138.9.0+138.0.36](https://github.com/tauri-apps/cef-rs/compare/cef-v138.8.0+138.0.34...cef-v138.9.0+138.0.36) - 2025-08-07

### Other

- *(release)* update CEF version to 138.0.36

## [138.8.0+138.0.34](https://github.com/tauri-apps/cef-rs/compare/cef-v138.7.1+138.0.33...cef-v138.8.0+138.0.34) - 2025-08-02

### Fixed

- remove cef version from example dependencies

### Other

- *(release)* update CEF version to 138.0.34

## [138.7.1+138.0.33](https://github.com/tauri-apps/cef-rs/compare/cef-v138.7.0+138.0.33...cef-v138.7.1+138.0.33) - 2025-07-29

### Other

- release v138.7.1+138.0.33
- move examples into separate crates

## [138.7.0+138.0.33](https://github.com/tauri-apps/cef-rs/compare/cef-v138.6.1+138.0.27...cef-v138.7.0+138.0.33) - 2025-07-29

### Other

- *(release)* update CEF version to 138.0.33

## [138.6.1+138.0.27](https://github.com/tauri-apps/cef-rs/compare/cef-v138.6.0+138.0.27...cef-v138.6.1+138.0.27) - 2025-07-28

### Fixed

- embed git-cliff as a library in get-latest

### Other

- *(release)* bump version for get-latest updates

## [138.6.0+138.0.27](https://github.com/tauri-apps/cef-rs/compare/cef-v138.5.1+138.0.26...cef-v138.6.0+138.0.27) - 2025-07-28

### Added

- update CEF version to 138.0.27

### Fixed

- bump version for release

## [138.5.1+138.0.26](https://github.com/tauri-apps/cef-rs/compare/cef-v138.5.0+138.0.26...cef-v138.5.1+138.0.26) - 2025-07-22

### Other

- release
- *(doc)* regenerate CHANGELOG.md

## [138.5.0+138.0.26](https://github.com/tauri-apps/cef-rs/compare/cef-v138.4.0+138.0.25...cef-v138.5.0+138.0.26) - 2025-07-19

### Other

- update CEF version

## [138.4.0+138.0.25](https://github.com/tauri-apps/cef-rs/compare/cef-v138.3.0+138.0.23...cef-v138.4.0+138.0.25) - 2025-07-18

### Other

- update CEF version

## [138.3.0+138.0.23](https://github.com/tauri-apps/cef-rs/compare/cef-v138.2.2+138.0.21...cef-v138.3.0+138.0.23) - 2025-07-17

### Other

- update CEF version

## [138.2.2+138.0.21](https://github.com/tauri-apps/cef-rs/compare/cef-v138.2.1+138.0.21...cef-v138.2.2+138.0.21) - 2025-07-14

### Other

- release
- seed CHANGELOG.md files

## [138.2.1+138.0.21](https://github.com/tauri-apps/cef-rs/compare/cef-v138.2.0+138.0.21...cef-v138.2.1+138.0.21) - 2025-07-14

### Fixed

- bump major version of download-cef [#145](https://github.com/tauri-apps/cef-rs/issues/145)

