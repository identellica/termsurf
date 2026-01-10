# TermSurf TODO

## Issues

1. [ ] Be able to open an html file in the current directory with
       `web open [filename]` or maybe `web file [filename]`.
2. [ ] You should be able to press cmd+c to copy the current url when in control
       mode.
3. [ ] Basic copy/paste commands cmd+c/x/v should work in insert mode (currently
       they do not, although shift+right/left does work)

## TODO

### Profile Management

- [ ] List existing profiles
- [ ] Delete profile data (`WKWebsiteDataStore.remove(forIdentifier:)`)

### Developer Tools

- [ ] `termsurf devtools` command to open Safari Web Inspector

### Additional Features

- [x] User agent customization (basic: set to Safari UA in v0.1.5)
- [ ] Download handling
- [ ] Permission prompts (camera, microphone, location)

### Documentation

- [ ] Update ARCHITECTURE.md with browser pane details
- [ ] Document profile system
- [ ] Add usage examples to README

## Future Ideas

- **CEF Integration** - Chrome DevTools and Blink rendering engine. Deferred due
  to Swift-to-C marshalling issues. See `docs/cef.md` for details.
- **Firefox Integration**.
- **Ladybird Integration**.
- **RSS Feeds** - Built-in feed reader functionality
- Support Linux
- Support Windows
