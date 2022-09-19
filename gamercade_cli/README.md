# Gamercade Cli

The **g**amer**c**ade **c**ommand **l**ine Interface, `gccl`.

Provides some useful tools for working with Gamercade for those who prefer a command line interface.

Learn more about [Gamercade](https://gamercade.io).

## How to Use

Run the cli with `gccl`. Invoke one of the following commands:

- `help` - Provides the help text.
- `bundle` - Bundle mode. You must provide code `-c`, an output file path `-o`, and optionally an asset provider `-a`.
- `console` - Console mode.

### Bundle Mode

This is how you can bundle and package game code and assets via command line. Usable by the `gccl bundle` command. Parameters:

- `--code` or `-c` - a code provider. Can be a `.wasm` file or an already bundled `.gcrom`.
- `--output` or `-o` - an output file. The path where you want to output the bundled `.gcrom`.
- `--assets` or `-a` - an asset provider. Can be a `.gce` file, or an already bundled `.gcrom`.

### Console Mode

This is how you can run the console, and optionally bundle files, via the command line. Usable by the `gccl console` command. Has a few different modes:

- `rom` mode. For quickly launching a single player game with the target rom. Must provide a path to the `.gcrom` file.
- `bundle` mode. For quickly bundling and launching a single player game with the target code and asset providers. Must provide code via `-c`, and optionally an assets source via `-a`.

## File Watching

You can also "watch" for file changes, and automatically run commands. This is done by adding the `-w` or `--watch` flag before entering the command. For example:

`gccl -w bundle -c [PATH TO CODE] -o my_game.gcrom`

This will use the default value for assets (since `-a` is not provided), and will bundle it with code provided to `-c`, into the output file `my_game.gcrom`

Alternatively, if you want to bundle and run the console for fast development, you can do:

`gccl -w console rom [PATH TO CODE]`

This will automatically launch and continue to re-launch the game should the rom file change.

Many other commands and modes are watchable, not only these two listed above.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
