# export-cef-dir

Export files from the prebuilt [Chromium Embedded Framework](https://github.com/chromiumembedded/cef)
archive on any supported platform. The structure of the exported directory matches the way that
the `cef-dll-sys` crate expects to see them.

To use the target directory when building, set the `CEF_PATH` environment variable to the path of the
exported directory, e.g., `~/.local/share/cef`.

To use the DLLs in this directory at runtime, the library loader path varies by platform:

- Linux

```sh
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$CEF_PATH"
```

- macOS

```sh
export DYLD_FALLBACK_LIBRARY_PATH="$DYLD_FALLBACK_LIBRARY_PATH:$CEF_PATH"
```

- Windows (using PowerShell)

```pwsh
$env:PATH = "$env:PATH;$env:CEF_PATH"
```
