# minesweeper-gui
A Relm4/Gtk4 UI for my rust minesweeper game.

![image](https://user-images.githubusercontent.com/33698065/210185175-def02c98-a8f9-4cf1-a2bc-6ecbe0645afe.png)
![image](https://user-images.githubusercontent.com/33698065/210185236-8b5c0c4b-2092-4222-a7c9-1391e4fe683f.png)

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
