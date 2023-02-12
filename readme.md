# minesweeper-gui

A Relm4/Gtk4 UI for the minesweeper game.

<img src="https://user-images.githubusercontent.com/33698065/216826865-5495416b-1ebf-482e-b748-e52c64da3a36.png" />

# minesweeper-tauri

A Tauri UI for the minesweeper game.
<img width="732" alt="image" src="https://user-images.githubusercontent.com/33698065/218323494-4f6bc94b-ef65-4bf1-96ff-168aa8da3b8e.png">

# minesweeper

A minesweeper game library and text binary interface.

```text
minesweeper

USAGE:
    minesweeper [OPTIONS]

OPTIONS:
    -c <COLUMNS>        Number of columns [default: 10]
    -h, --help          Print help information
    -r <ROWS>           Number of rows [default: 10]
```

Ex:

```text
$ minesweeper
board: 100, mines: 10
   1  2  3  4  5  6  7  8  9  10
 1 .  .  .  .  .  .  .  .  .  .
 2 .  .  .  .  .  .  .  .  .  .
 3 .  .  .  .  .  .  .  .  .  .
 4 .  .  .  .  .  .  .  .  .  .
 5 .  .  .  .  .  .  .  .  .  .
 6 .  .  .  .  .  .  .  .  .  .
 7 .  .  .  .  .  .  .  .  .  .
 8 .  .  .  .  .  .  .  .  .  .
 9 .  .  .  .  .  .  .  .  .  .
10 .  .  .  .  .  .  .  .  .  .

(o, f, q): o 1 1
board: 100, mines: 10
   1  2  3  4  5  6  7  8  9  10
 1    1  .  .  .  .  .  .  .  .
 2 1  2  .  .  .  .  .  .  .  .
 3 .  .  .  .  .  .  .  .  .  .
 4 .  .  .  .  .  .  .  .  .  .
 5 .  .  .  .  .  .  .  .  .  .
 6 .  .  .  .  .  .  .  .  .  .
 7 .  .  .  .  .  .  .  .  .  .
 8 .  .  .  .  .  .  .  .  .  .
 9 .  .  .  .  .  .  .  .  .  .
10 .  .  .  .  .  .  .  .  .  .

(o, f, q): o 10 1
board: 100, mines: 10
   1  2  3  4  5  6  7  8  9  10
 1    1  .  .  .  .  1
 2 1  2  .  .  .  .  1
 3 .  .  .  .  .  .  2
 4 .  2  1  1  .  .  1
 5 1  1     1  2  2  1
 6
 7 1  1           1  1  1
 8 .  2  1        1  .  1
 9 .  .  1        1  1  1
10 .  .  1

(o, f, q):
```
