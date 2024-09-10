# Please Installer

[Please Installer][] is a unified interface package manager for many OSes
inspired by [UPT][]. Even though it has started as a fork from UPT, Please
Installer was completely rewritten from scratch.

Please Installer is a beta release, not much is properly tested.

Such as UPT, Please Installer relies on the platform’s package manager to
perform its tasks.

## Installation

First install `rustup` using your system package manager, and then run:

```sh
rustup toolchain install nightly-2024-09-01
cargo +nightly-2024-09-01 install please-install
```

## Usage

```
❯ please help
A unified interface package manager for many OSes

Usage: please [OPTIONS] <COMMAND>

Commands:
  install       install package(s)
  remove        remove package(s)
  upgrade       upgrade package(s)
  search        search for package(s)
  info          get info for a package
  update        update database
  list          list installed packages
  list-vendors  list available vendors
  help          Print this message or the help of the given subcommand(s)

Options:
  -x, --skip-settings    skip settings
  -c, --config <CONFIG>  configuration file
  -d, --dry-run          dry run (do not actually execute commands)
  -y, --yes              assume yes for all prompts
  -s, --su               run as root (user must be sudoer)
  -v, --vendor <VENDOR>  set the installer command
  -h, --help             Print help
  -V, --version          Print version
```

You can also call `help` on subcommand:

```
❯ please help install
install package(s)

Usage: please install [OPTIONS] [PACKAGE]...

Arguments:
  [PACKAGE]...  package(s) to be installed

Options:
  -x, --skip-settings    skip settings
  -c, --config <CONFIG>  configuration file
  -d, --dry-run          dry run (do not actually execute commands)
  -y, --yes              assume yes for all prompts
  -s, --su               run as root (user must be sudoer)
  -v, --vendor <VENDOR>  set the installer command
  -h, --help             Print help
```

Supported vendors (backend package managers) are basically the same supported by
UPT:

| OS                                                   | Tools                |
|------------------------------------------------------|----------------------|
| windows                                              | scoop, choco, winget |
| macos                                                | brew, port           |
| ubuntu, debian, linuxmint, pop, deepin, elementary kali, raspbian, aosc, zorin, antix, devuan, bodhi, lxle, sparky | apt, snap, flatpak   |
| fedora, redhat, rhel, amzn, ol, almalinux, rocky, oubes, centos, qubes, eurolinux | dnf, yum, flatpak |
| arch, manjaro, endeavouros, arcolinux, garuda, antergos, kaos | pacman, yay, flatpak |
| alpine, postmarket                                   | apk                  |
| opensuse, opensuse-leap, opensuse-tumbleweed         | zypper, flatpak      |
| nixos                                                | nix-env, flatpak     |
| gentoo, funtoo                                       | emerge, flatpak      |
| void                                                 | xbps, flatpak        |
| mageia                                               | urpm, flatpak        |
| slackware                                            | slackpkg, flatpak    |
| solus                                                | eopkg, flatpak       |
| openwrt                                              | opkg, flatpak        |
| nutyx                                                | cards, flatpak       |
| crux                                                 | prt-get, flatpak     |
| freebsd, ghostbsd                                    | pkg                  |
| android                                              | termux               |
| haiku                                                | pkgman               |

## Settings

Please Installer can read settings from a configuration file in [TOML][] format
supplied by the option `--config=<CONFIG>`, by default
`$XDG_CONFIG_HOME/please.toml`.

Valid options and default values are:

```
assume-yes = false  # assume yes for all prompts
su = false          # run as root (user must be sudoer)
vendor = ""         # use a specific package manager from the available list
pager = ""          # use a specific pager to paginate search and list; in search, $args is replaced by the query
```

The default pager is given by `PAGER` environment variable; if it’s supplied in
the configuration file, Please Installer will assume `--paginate`.

You also may supply sessions to add per-subcommand settings. Valid
sessions are `[install]`, `[remove]`, `[update]`, `[upgrade]`, `[search]`, and
`[list]`.

Suggested configuration file:

```toml
assume-yes = true

[install]
su = true

[list]
pager = "bat --file-name='installed packages'"

[remove]
su = true

[search]
pager = "bat --file-name='search $args'"

[upgrade]
su = true

[update]
su = true
```

## Known bugs

Check [open issues][] for known bugs and feature requests.

## License

This software is licensed under the [BSD-3-Clause License][].

- Copyright 2024 Rodrigo Cacilhas &lt;montegasppa@cacilhas.info&gt;
- [COPYING][]

[BSD-3-Clause License]: https://opensource.org/licenses/BSD-3-Clause
[COPYING]: https://github.com/cacilhas/please?tab=License-1-ov-file
[open issues]: https://github.com/cacilhas/please/issues
[Please Installer]: https://crates.io/crates/please-install
[TOML]: https://toml.io/en/
[UPT]: https://crates.io/crates/upt
