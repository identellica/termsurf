# Acceptance Testing

This directory contains an acceptance test suite for ghostty. This works
by running the terminal emulator within a windowing environment, capturing a
screenshot, and comparing results. We use this to visually verify that
all rendering conforms to what we expect.

This test suite can also execute alternate terminal emulators so that we
can easily compare outputs between them.

## Running a Single Test

To run a single test, use the `run-host.sh` script. This must be executed
from this directory. Example:

```shell-session
$ ./run-host.sh --exec xterm --case /src/cases/vttest/launch.sh
```

The `--case` flag uses `/src` as the root for this directory.

The `--update` flag can be used to update the screenshot in place. This
should be used to gather a new screenshot. If you want to compare to the old
screenshot, copy the old one or use git to revert.

## Running the Full Suite

**Warning:** This can take a long time and isn't recommended. The CI
environment automatically runs the full test suite and is the recommended
approach.

To run the full test suite against all terminal emulators, use the
`run-all.sh` script. This optionally takes an `--exec` parameter to run
the full test suite against only a single terminal emulator.

## Modifying the `ghostty` Binary

This test suite expects the `ghostty` binary to be in _this directory_.
You can manually copy it into place. Each time you modify the binary, you
must rebuild the Docker image.
