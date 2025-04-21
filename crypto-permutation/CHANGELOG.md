# Changelog

## 0.1.1 - 2025-04-21

### Added
- Added `must_use` annotations to `BufMut::reborrow()` and `BufMut::restrict()` to avoid bugs like
  the one below.

### Fixed
- Fixed `<BufMut as Writer>::skip()` not actually skipping over buffer space.

## 0.1.0 - 2023-07-10 [YANKED]

__Notice__: Yanked because of the issue described under the 0.1.1 version.

Initial release
