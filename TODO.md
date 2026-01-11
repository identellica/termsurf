# TermSurf TODO

## TODO

### Profile Management

- [ ] List existing profiles
- [ ] Delete profile data (`WKWebsiteDataStore.remove(forIdentifier:)`)

### Developer Tools

- [ ] `termsurf devtools` command to open Safari Web Inspector

### Additional Features

- [x] User agent customization (basic: set to Safari UA in v0.1.5)
- [x] JavaScript dialogs (alert, confirm, prompt)
- [x] File uploads
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
