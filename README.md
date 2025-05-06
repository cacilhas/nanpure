# Kodumaro Nanpūrë

[COPYING]: https://github.com/cacilhas/nanpure/blob/master/COPYING
[email]: mailto:montegasppa@cacilhas.info
[Michael Kennett’s Sudoku]: https://github.com/cinemast/sudoku
[The 3-Clause BSD License]: https://opensource.org/license/bsd-3-clause/
[Sudoku]: https://en.wikipedia.org/wiki/Sudoku
[Screenshot]: ./assets/screenshot.png

## Nanpūrë

Yet another (colourful) [Sudoku][] playing game.

### Installation

```sh
cargo install nanpure
```

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

- 1: <span style="color: white; background-color: red;">red</span>
- 2: <span style="color: black; background-color: orange;">orange</span>
- 3: <span style="color: black; background-color: yellow;">yellow</span>
- 4: <span style="color: white; background-color: green;">green</span>
- 5: <span style="color: black; background-color: skyblue;">blue</span>
- 6: <span style="color: white; background-color: indigo;">indigo</span>
- 7: <span style="color: black; background-color: violet;">violet</span>
- 8: <span style="color: white; background-color: magenta;">magenta</span>
- 9: <span style="color: black: background-color: darkgray;">gray</span>

### Generator

It’s require to have [Michael Kennett’s Sudoku][] installed. It’s used as board
generator.

### License

- Copyright 2023-2025 [Rodrigo Cacilhας \<montegasppa@cacilhas.info\>][email]
- [The 3-Clause BSD License][]
- [COPYING][]

License: BSD-3-Clause

![Screenshot][]
