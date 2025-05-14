# Kodumaro Numplës

[COPYING]: https://codeberg.org/cacilhas/numples/src/branch/master/COPYING
[email]: mailto:montegasppa@cacilhas.cc
[issues]: https://codeberg.org/cacilhas/numples/issues
[Michael Kennett’s Sudoku]: https://github.com/cinemast/sudoku
[Sudoku]: https://en.wikipedia.org/wiki/Sudoku
[Screenshot]: ./assets/screenshot.png
[The 3-Clause BSD License]: https://opensource.org/license/bsd-3-clause/
[UPX]: https://upx.github.io/

## Numplës (ナンプレツ)

Yet another (colourful) [Sudoku][] playing game. **Numplës** is pronounced as
“num-place” and stands for “number place”. It’s derivative from the Japanese
ナンプレ (*nanpŭrē*, Sudoku).

### Installation

```sh
cargo install numples
```

You’re probably going to want to run [UPX][] on the binary:

```sh
upx --best --lzma ~/.cargo/bin/numples
```

> Author note:
>
> Here in my case, release 1.0.1, UPX reduced the binary size from 27 MiB to
> 5.8 MiB (about 22% of the original size).

### Controls

- Cursor keys: select cell
- Numbers: set cell value
- 0: clean cell value up
- Control + number: toggle cell candidate
- U: undo
- Escape: back
- Pause: pause / unpause
- Control + Q: quit

You can use the mouse to select cells as well.

### Colours

- 1: <span style="color: black; background-color: #ff0000;">red</span>
- 2: <span style="color: black; background-color: #ff8000;">orange</span>
- 3: <span style="color: black; background-color: #ffff00;">yellow</span>
- 4: <span style="color: black; background-color: #00ff00;">green</span>
- 5: <span style="color: black; background-color: #00ffff;">blue</span>
- 6: <span style="color: white; background-color: #0000ff;">indigo</span>
- 7: <span style="color: black; background-color: #cd33ff;">violet</span>
- 8: <span style="color: black; background-color: #ff94ff;">pink</span>
- 9: <span style="color: white; background-color: #808080;">gray</span>

### Requirements

It’s require that [Michael Kennett’s Sudoku][] be installed: it’s used as board
generating back-end.

### Known bugs

Found bugs can be reported [here][issues].

### License

- Copyright 2023-2025 [Rodrigo Cacilhας \<montegasppa@cacilhas.cc\>][email]
- [The 3-Clause BSD License][]
- [COPYING][]

License: BSD-3-Clause

![Screenshot][]
