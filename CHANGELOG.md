# Changelog

## v0.1.6 (2022-02-16)
### Added 
- feature `lax_comparison`: Lax comparison instread of std `PartialEq` on `Value` which tries some conversions before failing.  If enabled, `String` can be compared to `Bool` and `Numeric` as well as `Numeric` against `Bool`. As this
behaviour is unexpected for callers, this feature is not enabled by default.

### Fixed
- if a comparison sequence ended with a block that had a whitespace trailing, the parsing failed

## v0.1.5 (2022-02-15)
### Added
- feature `serde_de`: Serde Deserialize support for Sequence