# Static Rock

A minimal Ubuntu OCI rock designed to serve as a lightweight parent image for running statically compiled binaries.

## Overview

The Static Rock provides a chiseled Ubuntu base optimized for running statically linked binaries from languages like C, C++, Go, and Rust. By including only the bare essentials—timezone data, certificate authorities, and system files—it keeps image size and attack surface to a minimum.

This rock is intended to be used as a **parent image only** and provides no functionality on its own beyond the Pebble entrypoint.

## Features

- **Minimal footprint**: Includes only dependencies necessary for static binaries to function
- **Security-focused**: Chiseled Ubuntu base with reduced attack surface
- **Pebble entrypoint**: Service and process management via Pebble
- **Multi-language support**: Ready for statically compiled binaries from Go, Rust, C/C++, and other languages
- **Non-root user**: Runs as `_daemon_` user by default for improved security

## Quick Start

### Prerequisites

- [Rockcraft](https://ubuntu.com/containers/rockcraft/docs/latest/) installed
- [LXD](https://documentation.ubuntu.com/lxd/en/latest/) running
- Linux system (Ubuntu 20.04 or later recommended)

### Building the Rock

```bash
# Build the rock
rockcraft pack
```

### Running Tests

```bash
rockcraft test
```


## Documentation

- [Rockcraft Documentation](https://ubuntu.com/containers/rockcraft/docs/)
- [Ubuntu Chiselled Images](https://ubuntu.com/chisel/docs/)
- [Pebble Reference](https://documentation.ubuntu.com/pebble/)
