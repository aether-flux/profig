# Changelog

## [0.1.0] - Initial Release
### Added
- Procedural macros: `#[derive(Profig)]` and `#[profig(...)]`
- Schema definition and validation (`min`, `max`, `regex`, `default`, etc.)
- Enfore strict parsing rules by specifying only select formats: `#[profig(formats="toml, json")]`
- Auto-detection of file format while loading
- Sample and Documentation generation
- Different formats supported: `TOML`, `JSON`, `YAML` (feature-flagged)

## Changed
- NA

## Removed
- NA

