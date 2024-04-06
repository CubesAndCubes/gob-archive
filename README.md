# gob-archive

Rust-written CLI archiver and un-archiver for the LucasArts GOB archive format.

This program depends on the [gob-rs](https://github.com/CubesAndCubes/gob-rs) GOB parsing library.

It's tested to be able to unpack GOB files of (but is likely not limited to):

- Indiana Jones and the Infernal Machine
- Star Wars Jedi Knight: Dark Forces II

## Installation

See [releases](https://github.com/CubesAndCubes/gob-archive/releases) or build the program yourself via cargo:

```sh
cargo install gob_archive
```

## Usage

### Syntax

```sh
gob_archive [mode] [source] [destination?]
```

- `mode`
    - `x` => Archive extraction mode.
    - `extract` => Archive extraction mode (verbose status output).
    - `c` => Archive creation mode.
    - `create` => Archive creation mode (verbose status output).
- `source` Source archive or directory for extraction or creation respectively.
- `destination` (optional) Destination directory or file for extraction or creation respectively.

### Extraction

Will extract contents of `CD1.GOB` to `/path/to/CD1` (directory will be created if non-existent):

```sh
gob_archive x /path/to/CD1.GOB
```

Will extract contents of `CD1.GOB` to `/path/to/destination/` (directory will be created if non-existent):

```sh
gob_archive x /path/to/CD1.GOB /path/to/destination/
```

### Creation

Will archive and write contents of `CD1` to `path/to/CD1.GOB` (file will be created if non-existent):

```sh
gob_archive c /path/to/CD1
```

Will archive and write contents of `CD1` to `path/to/destination.GOB` (file will be created if non-existent):

```sh
gob_archive c /path/to/CD1 /path/to/destination.GOB
```

## License

This software is dual-licensed under the [MIT license](LICENSE-MIT) and [Apache License, Version 2.0](LICENSE-APACHE).