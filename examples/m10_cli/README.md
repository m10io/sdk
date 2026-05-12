## Build CLI

The CLI can be built for internal staff, or for customers, where certain menu items have been hidden. The build for internal staff produces the `m10` binary; the build for customers produces the `drm` binary.

Internal (produces `m10`):
```bash
cd services/ledger-api/m10_cli && cargo build --release --features internal --bin m10
cd ../target/release && cp m10 $HOME/.local/bin/m10 # or similar location in your path
```

Customers (produces `drm`):
```bash
cd services/ledger-api/m10_cli && cargo build --release --features customers --bin drm
cd ../target/release && cp drm $HOME/.local/bin/drm # or similar location in your path
```

`--features` is specified in Cargo.toml.

To enable a menu item to be toggled at build time, use 

```rust  
#[cfg_attr(feature = "customers", command(alias = "<short_name>", hide = true))]
#[cfg_attr(feature = "internal", command(alias = "<short_name>"))]
```
Example:
```rust
    /// Issue a token
    #[cfg_attr(feature = "customers", command(alias = "it", hide = true))]
    #[cfg_attr(feature = "internal", command(alias = "it"))]
    Issue {
        #[command(subcommand)]
        cmd: token::Issue,
    },
```