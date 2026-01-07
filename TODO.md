# TermSurf TODO

## Issues

1. [x] cmd+r does not refresh the page. it does nothing. we probably need to
       handle cmd+r in a similar way as ctrl+c. it is not a menu item.
2. [x] Links that open in a new window don't work. For instance, all links
       inside a post on x.com do not work. We will most likely need to detect
       the "open this link in a new window" api call from the webview and then
       do something with that command, like run "web open ..." in a new tab.
3. [x] Be able to press cmd+c/v/x in insert mode in the control panel.
4. [x] Be able to press cmd+z/Z (undo, redo) in insert mode.
5. [ ] Be able to press cmd+c/v in control mode in the control panel.
6. [ ] Be able to open an html file in the current directory with
       `web open [filename]` or maybe `web file [filename]`.
7. [ ] You should be able to press cmd+c to copy the current url when in control
       mode.
8. [ ] Basic copy/paste commands cmd+c/x/v should work in insert mode (currently
       they do not, although shift+right/left does work)

## TODO

### Profile Management

- [ ] List existing profiles
- [ ] Delete profile data (`WKWebsiteDataStore.remove(forIdentifier:)`)

### Developer Tools

- [ ] `termsurf devtools` command to open Safari Web Inspector

### Additional Features

- [ ] User agent customization
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
