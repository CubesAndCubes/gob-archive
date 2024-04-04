# gob-archive

Rust-written CLI archiver and un-archiver for the LucasArts GOB file format.

## Usage

### Syntax

```sh
gob_archive [mode] [source] [destination?]
```

- `mode`
    - `extract` | `x` => Archive extraction mode.
    - `create` | `c` => Archive creation mode.
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

## Specification

### Data Structure

GOB files are encoded in the little-endian format.

The file structure can be abstracted as follows:

```rs
Gob {
    header: Header,
    body: Body,
}

Header {
    signature: 4 bytes, // must be "GOB "
    version: 4 bytes, // must be 0x14 -> 20
    body_offset: 4 bytes, // usually 0xC -> 12; byte address where body starts
}

Body {
    file_count: 4 bytes, // amount of files in archive
    files: [File; file_count], // file definitions
    ...file_data, // data of files; makes up remainder, thus size is variable
}

File {
    offset: 4 bytes, // byte address where file data starts
    size: 4 bytes, // size of file data in bytes
    filepath: 128 bytes, // path of file within archive
}
```