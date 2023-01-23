export type Position = {
  index: number,
  pos: {
    x: number,
    y: number,
  }
  cell: Cell,
}


export type Cell = {
  adjacentMines: number,
  state: State
}

export type State =
  { type: "Closed", content: { flagged: boolean, mined: boolean } } |
  { type: "Open" } |
  { type: "ExposedMine" }