# CEF Integration: Incremental Plan

## Overview

**Two WezTerm.app locations:**

- `assets/macos/WezTerm.app/` → Template (checked into repo, read-only)
- `target/release/WezTerm.app/` → Built bundle (created during build)

---

## Step 1: Add CEF Dependency (Compile Only)

**Goal:** Verify CEF links correctly with wezterm-gui.

**Changes:**

1. Edit `wezterm-gui/Cargo.toml` - add feature and dependency:

```toml
[features]
cef = ["dep:cef"]

[target.'cfg(target_os = "macos")'.dependencies]
cef = { path = "../../cef-rs/cef", optional = true }
```

2. Edit `wezterm-gui/src/main.rs` - add minimal CEF reference:

```rust
#[cfg(all(target_os = "macos", feature = "cef"))]
fn cef_compiled() {
    let _ = cef::api_hash;
}
```

**Test:**

```bash
cargo build -p wezterm-gui --features cef
```

**Success criteria:**

- Build completes with no errors
- Binary exists at `target/debug/wezterm-gui`

---

## Step 2: Create Helper Binary (Compile Only)

**Goal:** Verify helper binary compiles.

**Changes:**

1. Create `wezterm-gui/src/bin/wezterm-cef-helper.rs`:

```rust
use cef::{args::Args, execute_process, library_loader, App};

fn main() {
    let args = Args::new();

    #[cfg(target_os = "macos")]
    let _loader = {
        let loader = library_loader::LibraryLoader::new(
            &std::env::current_exe().unwrap(),
            true,
        );
        assert!(loader.load());
        loader
    };

    execute_process(
        Some(args.as_main_args()),
        None::<&mut App>,
        std::ptr::null_mut(),
    );
}
```

2. Edit `wezterm-gui/Cargo.toml` - add bin target:

```toml
[[bin]]
name = "wezterm-cef-helper"
path = "src/bin/wezterm-cef-helper.rs"
required-features = ["cef"]
```

**Test:**

```bash
cargo build -p wezterm-gui --features cef
```

**Success criteria:**

- Both binaries exist:
  - `target/debug/wezterm-gui`
  - `target/debug/wezterm-cef-helper`

---

## Step 3: Manually Create Bundle

**Goal:** Create `target/release/WezTerm.app/` by copying from the template and
cef-osr.

**Prerequisites:**

- cef-osr bundle must exist at `/Users/ryan/dev/termsurf/cef-rs/cef-osr.app/`
- If not, build it first: `cd /Users/ryan/dev/termsurf/cef-rs && cargo build -p cef-osr && cargo run -p bundle-cef-app -- cef-osr -o cef-osr.app`

**Actions:**

```bash
# 1. Build release binaries
cargo build -p wezterm-gui --features cef --release

# 2. Copy template to target
cp -R assets/macos/WezTerm.app target/release/WezTerm.app

# 3. Create missing directories
mkdir -p target/release/WezTerm.app/Contents/MacOS
mkdir -p target/release/WezTerm.app/Contents/Frameworks

# 4. Copy main executable
cp target/release/wezterm-gui target/release/WezTerm.app/Contents/MacOS/

# 5. Copy CEF framework from cef-osr (known working)
cp -R "/Users/ryan/dev/termsurf/cef-rs/cef-osr.app/Contents/Frameworks/Chromium Embedded Framework.framework" target/release/WezTerm.app/Contents/Frameworks/

# 6. Create helper bundles by copying from cef-osr and modifying
CEF_OSR_FRAMEWORKS="/Users/ryan/dev/termsurf/cef-rs/cef-osr.app/Contents/Frameworks"
for suffix in "Helper" "Helper (GPU)" "Helper (Renderer)" "Helper (Plugin)" "Helper (Alerts)"; do
    SRC_BUNDLE="${CEF_OSR_FRAMEWORKS}/cef-osr ${suffix}.app"
    DEST_BUNDLE="target/release/WezTerm.app/Contents/Frameworks/WezTerm ${suffix}.app"

    # Copy entire helper bundle structure from cef-osr
    cp -R "${SRC_BUNDLE}" "${DEST_BUNDLE}"

    # Rename the executable
    mv "${DEST_BUNDLE}/Contents/MacOS/cef-osr ${suffix}" "${DEST_BUNDLE}/Contents/MacOS/WezTerm ${suffix}"

    # Replace with our helper binary
    cp target/release/wezterm-cef-helper "${DEST_BUNDLE}/Contents/MacOS/WezTerm ${suffix}"

    # Update Info.plist: replace "cef-osr" with "WezTerm" and update bundle identifier
    sed -i '' 's/cef-osr/WezTerm/g' "${DEST_BUNDLE}/Contents/Info.plist"
    sed -i '' 's/apps.tauri.cef-rs.WezTerm/com.github.wez.wezterm.helper/g' "${DEST_BUNDLE}/Contents/Info.plist"
done

# 7. Add MallocNanoZone to main app Info.plist (required for CEF on macOS)
# Insert after the opening <dict> tag
sed -i '' 's/<dict>/<dict>\
	<key>LSEnvironment<\/key>\
	<dict>\
		<key>MallocNanoZone<\/key>\
		<string>0<\/string>\
	<\/dict>/' target/release/WezTerm.app/Contents/Info.plist
```

**Test:**

```bash
# Verify bundle structure
ls -la target/release/WezTerm.app/Contents/Frameworks/

# Verify MallocNanoZone is in main plist
grep -A3 MallocNanoZone target/release/WezTerm.app/Contents/Info.plist

# Verify helper plists have correct executable names
grep CFBundleExecutable target/release/WezTerm.app/Contents/Frameworks/*/Contents/Info.plist
```

**Success criteria:**

- `ls` output shows:
  - `Chromium Embedded Framework.framework/`
  - `WezTerm Helper.app/`
  - `WezTerm Helper (GPU).app/`
  - `WezTerm Helper (Renderer).app/`
  - `WezTerm Helper (Plugin).app/`
  - `WezTerm Helper (Alerts).app/`
- `grep MallocNanoZone` shows the key exists with value `0`
- `grep CFBundleExecutable` shows `WezTerm Helper`, `WezTerm Helper (GPU)`, etc.

---

## Step 4: Run Without CEF Init

**Goal:** Verify the bundle structure works before adding CEF code.

**Test:**

```bash
./target/release/WezTerm.app/Contents/MacOS/wezterm-gui
```

**Success criteria:**

- WezTerm launches normally
- Terminal works as expected

---

## Step 5: Add CEF Loading Code

**Goal:** Load and initialize CEF.

**Changes to `wezterm-gui/src/main.rs`:**

```rust
#[cfg(all(target_os = "macos", feature = "cef"))]
fn init_cef() -> Result<(), String> {
    use cef::{args::Args, execute_process, initialize, library_loader, Settings, App};

    let exe = std::env::current_exe().map_err(|e| format!("current_exe: {e}"))?;
    let loader = library_loader::LibraryLoader::new(&exe, false);
    if !loader.load() {
        return Err("Failed to load CEF framework".into());
    }
    log::info!("CEF framework loaded");

    let args = Args::new();
    let ret = execute_process(
        Some(args.as_main_args()),
        None::<&mut App>,
        std::ptr::null_mut(),
    );
    if ret >= 0 {
        std::process::exit(ret);
    }
    log::info!("CEF execute_process returned {ret}");

    let settings = Settings {
        windowless_rendering_enabled: 1,
        external_message_pump: 1,
        no_sandbox: 1,
        ..Default::default()
    };

    if initialize(Some(args.as_main_args()), Some(&settings), None::<&mut App>, std::ptr::null_mut()) != 1 {
        return Err("CEF initialize failed".into());
    }

    log::info!("CEF initialized successfully");
    Ok(())
}
```

Add to `main()` after `notify_on_panic()`:

```rust
#[cfg(all(target_os = "macos", feature = "cef"))]
match init_cef() {
    Ok(()) => {}
    Err(e) => log::error!("CEF init failed: {e}"),
}
```

**Test:**

```bash
cargo build -p wezterm-gui --features cef --release
cp target/release/wezterm-gui target/release/WezTerm.app/Contents/MacOS/
RUST_LOG=info ./target/release/WezTerm.app/Contents/MacOS/wezterm-gui 2>&1 | grep -i cef
```

**Success criteria:**

- Log shows: `CEF framework loaded`
- Log shows: `CEF initialized successfully`
- WezTerm launches normally

---

## Step 6: Add CEF Shutdown

**Goal:** Clean shutdown.

**Changes to `wezterm-gui/src/main.rs`** - at end of `main()`:

```rust
#[cfg(all(target_os = "macos", feature = "cef"))]
cef::shutdown();
```

**Test:**

- Run app, then Cmd+Q to quit
- Should exit cleanly with no crash

---

## Step 7: Automate Bundle Creation

**Goal:** Script the manual steps from Step 3.

Create `scripts/bundle-cef.sh` containing the commands from Step 3.

**Test:**

```bash
rm -rf target/release/WezTerm.app
./scripts/bundle-cef.sh
./target/release/WezTerm.app/Contents/MacOS/wezterm-gui
```

---

## Summary

| Step | What               | Test                         | Pass                |
| ---- | ------------------ | ---------------------------- | ------------------- |
| 1    | Add CEF dependency | `cargo build --features cef` | Compiles            |
| 2    | Add helper binary  | `cargo build --features cef` | Both binaries exist |
| 3    | Manual bundle      | `ls Frameworks/`             | 6 items present     |
| 4    | Run without CEF    | Launch app                   | WezTerm works       |
| 5    | Add CEF init       | Check logs                   | "CEF initialized"   |
| 6    | Add shutdown       | Quit app                     | Clean exit          |
| 7    | Automate           | Run script                   | Same as step 5      |
