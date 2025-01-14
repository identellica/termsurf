# cef-rs

Use CEF in Rust.

## Supported Targets

| Target | Linux | macOS | Windows |
| ------ | ----- | ----- | ------- |
| x86_64 | ✅    | ✅    | ✅      |
| ARM64  | ✅    | ✅    | ✅      |

## Usage

### Linux

#### Manual Install

- [Download](https://cef-builds.spotifycdn.com/index.html#linux64) Linux-64bit build.

- Copy files to `.local`:

```
cp -r Resources ~/.local/share/cef
cp -r Release ~/.local/share/cef
```

- Build and run the application with `LD_LIBRARY_PATH` (or you can also add rpath to your cargo config or build script):

```
LD_LIBRARY_PATH=~/.local/share/cef cargo r --example demo
```

#### Flatpak

- Install flatpak runtime & sdk:

```
flatpak install flathub dev.crabnebula.Platform
flatpak install flathub dev.crabnebula.Sdk
```

- Setup cargo project for flatpak. See [flatpak-builder-tools](https://github.com/flatpak/flatpak-builder-tools/blob/master/cargo/README.md) for more details. Here are files you will need to have at leaset:
  - flatpak-cargo-generator.py
  - flatpak manifest file (ie. app.example.demo.yml)

- Build the flatpak application and run:

```
cargo b --example demo
python3 ./flatpak-cargo-generator.py ./Cargo.lock -o cargo-sources.json
touch run.sh
flatpak-builder --user --install --force-clean target app.example.demo.yml
flatpak run app.example.demo
```

### macOS

- Download cef prebuilts with `update-bindings`, it should print out the extracted `CEF_PATH` on success.

```
cargo run -p update-bindings --bindgen --download
```

- Build and run the cefsimple application

```
export CEF_PATH=<path>

./cef/examples/cefsimple/bundle_script.rs

open target/debug/examples/cefsimple.app
```

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

