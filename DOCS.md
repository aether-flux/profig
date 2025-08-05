# Profig - Documentation

**Profig** is a declarative configuration framework for Rust. It lets you define config schemas using `#[derive(Profig)]`, adds validation with attribute macros, and supports sample generation and docs creation out of the box.

---

## Getting Started

### Add to `Cargo.toml`
```toml
[dependencies]
profig = { version = "0.1", features = ["toml", "json", "yaml"] }
```
Enable only the formats you plan to use.

### Define your schema (struct)
```rust
use profig::Profig;

#[derive(Profig)]
#[profig(format = "toml,json")]
struct MyConfig {
    #[profig(doc = "Number of threads", min = 1, default = "4")]
    threads: usize,

    #[profig(doc = "Output path")]
    path: String,

    #[profig(doc = "Enable verbose output", default = "false")]
    verbose: bool,
}
```

### Load the config
```rust
fn main() -> Result<(), Box<dyn std::error::Error> {
    let config = MyConfig::load("config.toml")?;
    Ok(())
}
```
**Profig** will auto-detect the format based on the file extension.

## Supported File Formats
You can read and generate formats in:
- TOML
- YAML
- JSON

> You must enable the corresponding feature (eg "toml" for TOML) to use the file formats with **profig**.

## Field-Level Attributes
### `#[profig(...)]` options
| Attribute | Description |
| --------- | ----------- |
| `doc="..."` | Description for the field (used for generating docs) |
| `default="..."` | Default value if the field is missing |
| `min="N"` | Minimum numeric value |
| `max="N"` | Maximum numeric value |
| `regex="..."` | Regex pattern (for strings) |

> Optional fields using `Option<T>` are skipped if not present, and replaced with `default` value if provided.

## Validation
### Built-in Validation
- Numeric `min/max` constraints
- String pattern matching with `regex`
- Defaults applied when needed

### Custom Validation
You can add your own validation by implementing a method:
```rust
impl MyConfig {
    pub fn validate(&self) -> Result<(), profig::ProfigError> {
        if self.threads > 100 {
            return Err(profig::ProfigError::Validation("Too many threads".into()));
        }
        Ok(())
    }
}
```

## Sample Generation
Generate sample config files from your metadata:
```rust
MyConfig::sample_config("sample.toml");
// Creates a file `sample.toml` with default and sample values
```
> You can generate samples in any supported format, regardless of the `format="..."` restriction. All you need is the corresponding `feature` enabled.

## Docs Generation
Generate documentation for your config struct:
```rust
MyConfig::generate_docs("myConfig.toml");
// Generates a file `config.md` with basic docs
```
> Output is a `markdown` file with headings and descriptions.

## How It Works?
- Uses a custom derive macro: `#[derive(Profig)]`
- Parses your struct and the `#[profig(...)]` attributes
- Generates loading logic, deserialization, validation, sample and docs
- Format is auto-detected using file extension
- Powered by `serde` internally

# Questions?
Open an issue on GitHub or reach out!

# License
MIT

