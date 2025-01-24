[![Cargo](https://img.shields.io/crates/v/esix.svg)](https://crates.io/crates/esix)

# Esix
Esix is a simple wrapper for the e621 API in Rust.

## Installation
```toml
[dependencies]
esix = "0.1.1"
```

## Example
```rust
use esix::{error::Error, Esix};

fn main() -> Result<(), Error> {
    let mut client = Esix::new(
        "API_KEY",
        "USERNAME",
        "project_name".to_string(),
        "project_version".to_string(),
    );

    let posts = client.list("rating:safe", 1)?;

    for post in posts {
        println!("{:?}", post);
    }

    Ok(())
}

```

This crate is still in development and is not yet ready for production use.

It may include many bugs and missing features.

## License
This project is licensed under the MIT license.

## Contributing
Please feel free to contribute to this project by opening a pull request.

**You are responsible for any possible form of abuse with the e621 API.**