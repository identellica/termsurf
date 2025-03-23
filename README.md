# cef-rs

Use CEF in Rust.

## Supported Targets

| Target | Linux | macOS | Windows |
| ------ | ----- | ----- | ------- |
| x86_64 | ✅    | ✅    | ✅      |
| ARM64  | ✅    | ✅    | ✅      |

## Usage

### Install Shared CEF Binaries

This step is optional, but it will make all other builds of the `cef` crate much faster. If you don't do this, the `cef-dll-sys` crate `build.rs` script will download and extract the same files under its `OUT_DIR` directory. You should repeat this step each time you upgrade to a new version of the `cef` crate.

#### Linux or macOS:

```sh
cargo run -p export-cef-dir -- --force $HOME/.local/share/cef
```

#### Windows (using PowerShell)

```pwsh
cargo run -p export-cef-dir -- --force $env:USERPROFILE/.local/share/cef
```

### Set Environment Variables

#### Linux

```sh
export CEF_PATH=$HOME/.local/share/cef
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$CEF_PATH
```

#### macOS

```sh
export CEF_PATH=$HOME/.local/share/cef
export DYLD_FALLBACK_LIBRARY_PATH=$DYLD_FALLBACK_LIBRARY_PATH:$CEF_PATH
```

#### Windows (using PowerShell)

```pwsh
$env:CEF_PATH="$env:USERPROFILE/.local/share/cef"
$env:PATH="$env:PATH;$env:CEF_PATH"
```

### Run the `cefsimple` Example

#### Linux

```sh
cargo run --example cefsimple
```

#### macOS

```sh
cargo run --example bundle_cefsimple
open target/debug/examples/cefsimple.app
```

#### Windows (using PowerShell)

```pwsh
cp ./cef/examples/cefsimple/win/cefsimple.exe.manifest ./target/debug/examples/
cargo run --example cefsimple
```

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.
