# licg

licg - License Generator. Generate open source license in one single command.

## Install

You can build it from source.

```console
$ cargo build
```

## Usage

```console
Generate a MIT License with default holder name and file path.
$ licg mit
```

The default license file path is `./LICENSE`.

The default holder name is the git user name in local machine query by
`git config user.name`. If the query fail for some reasons like git is not installed, the `-n`/`--name` flag is required.

The year in the generated license is the current year, which is filled in automatically.

```console
Generate the MIT License in LICENSE.txt with holder name foo
$ licg mit -p ./LICENSE.txt -n foo
```

If the target file already exists, `licg` fails instead of overwriting it. Use `-f`/`--force` to overwrite:

```console
$ licg mit -f
```

## Supported Licenses

Supported Licenses:
- Apache2:   Apache License 2.0
- AGPL3:     GNU Affero General Public License v3.0
- BSD2:      BSD 2-Clause "Simplified" License
- BSD3:      BSD 3-Clause “New” or “Revised” License
- BSL:       Boost Software License 1.0
- EPL2:      Eclipse Public License 2.0
- GPL2:      GNU General Public License v2.0
- GPL3:      GNU General Public License v3.0
- LGPL2:     GNU Lesser General Public License v2.1
- LGPL3:     GNU Lesser General Public License v3.0
- MIT:       MIT License
- MPL2:      Mozilla Public License 2.0
- Unlicense: The Unlicense

You can also use `-l`/`--list` flag to see the supported licenses.

```console
$ licg -l
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
