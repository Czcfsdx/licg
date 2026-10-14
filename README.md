# licg

licg - License Generator. Generate open source license in one single command.

## Usage

Generate a BSD 2-Clause "Simplified" License with default holder name and file path.

```console
$ licg bsd2
```

The default license file path is `./LICENSE`.

The default holder name is the git user name in local machine query by
`git config user.name`. If the query fail for some reasons like git is not installed, the `-n`/`--name` flag is required.

Generate MIT License in `./LICENSE.txt` with holder name `foo`

```console
$ licg mit -p ./LICENSE.txt -n foo
```

## Build

```console
$ cargo build
```

## Supported Licenses

Supported licenses:
- BSD2    : BSD 2-Clause "Simplified" License
- BSD3    : BSD 3-Clause “New” or “Revised” License
- MIT     : MIT License
- Apache2 : Apache License 2.0
- GPL2    : GNU General Public License v2.0
- GPL3    : GNU General Public License v3.0

You can also use `-l`/`--list` flag to see the supported licenses.

```console
$ licg -l
Supported Licenses:
    Apache2: Apache License 2.0
    BSD2: BSD 2-Clause "Simplified" License
    BSD3: BSD 3-Clause “New” or “Revised” License
    GPL2: GNU General Public License v2.0
    GPL3: GNU General Public License v3.0
    MIT: MIT License
```

The year in the generated license is the current year, which is filled in automatically.

If the target file already exists, `licg` fails instead of overwriting it. Use `-f`/`--force` to overwrite:

```console
$ licg MIT -f
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
