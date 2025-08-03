# Profig
**A Config Framework with schema validation, sample configs, and doc generation**
> Load your config files, validate them, document them, all with `#[derive(Profig)]`.

---

## Features
- **Easy config schema definition** with `#[derive(Profig)]`
- **Set strict formats** with `#[profig(formats="json,toml")]`
- **Field-level metadata** using `#[profig(doc = "...", default = "...")]`
- **Built-in validation** with `#[profig(min = 1, max = 10, regex = "...")]`
- **Multi format support**: TOML, JSON, YAML
- **Automatically load data** by automatically determining file type (ie `.json`/`.toml`/`.yaml`)
- **Automatic sample generation** with one `sample_config("filename.format")` (eg, sample_config("sample.json"))
- **Docs generation** using the provided `doc="..."` metadata fields

## Example Usage

```rust
use profig::Profig;

// Config Schema struct
#[derive(Profig)]
#[profig(formats="toml, json")]
struct AppConfig {
    #[profig(min="4", max="10", doc="Number of worker threads")]
    threads: usize,

    #[profig(doc="Path to output directory")]
    output_dir: String,

    #[profig(default="false", doc="Enable debug mode")]
    debug: Option<bool>,
}

// Main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load a JSON file
    let jsonConf = AppStruct::load("config.json")?;

    // Load a TOML file
    let tomlConf = AppStruct::load("config.toml")?;

    // Print fields from JSON-loaded object
    println!("Threads: {}", jsonConf.threads);

    // Print fields from TOML-loaded object
    println!("Output directory: {}", tomlConf.output_dir);
}
```

## Formats Supported
Currently, **profig** supports the following config file types:
- `JSON`
- `TOML`
- `YAML`

Each format is gated behind their individual feature flags. Enable the features in `Cargo.toml` as follows:
```toml
[dependencies.profig]
version = "0.1"
features = ["toml", "json", "yaml"]  # choose the features as per your requirements
```

## Full Documentation
See [DOCS.md](https://github.com/aether-flux/profig/blob/main/DOCS.md) for complete documentation and guide on usage of **profig**.

## Installation
Run the following command to install the crate:
```bash
cargo add profig
```

Or, for format features:
```bash
cargo add profig --features "toml,json"
```

Alternatively, you can directly add a `Cargo.toml` entry as follows:
```toml
[dependencies]
profig = { version = "0.1", features = ["toml", "yaml"] }
```

## Status
Profig is currently in **v0.1**, stable for usage but still in very early stages.

## Links
- [Crates.io](https://crates.io/crates/profig)
- [GitHub Repository](https://github.com/aether-flux/profig)

## License
MIT

