# A minesweeper game with different Rust UI Frameworks.

# minesweeper-relm4

A Relm4/Gtk4 UI for the minesweeper game.

https://user-images.githubusercontent.com/33698065/227719432-6a6ccaf7-b81f-47e0-b0c7-55e4527ae06e.mov

https://user-images.githubusercontent.com/33698065/227719270-cd3130cb-56d1-4922-90d5-236acf9c9d69.mov

# minesweeper-tauri

A Tauri UI for the minesweeper game.

https://user-images.githubusercontent.com/33698065/227748843-4da95c55-5bdf-4042-8dc8-2a617085d5bc.mov

# minesweeper-iced

An Iced UI for the minesweeper game.

<img width="712" alt="image" src="https://github.com/darrell-roberts/minesweeper/assets/33698065/81581b72-4370-4328-8ba7-9fd25d618621">

## Build from source

### Clone Repo

```
git clone https://github.com/darrell-roberts/minesweeper.git
```

### Install npm dependencies

```
cd minesweeper-tauri/vite-minesweeper
npm install
```

### Build Tauri App

```
cd ../minesweeper
cargo tauri build
```

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
