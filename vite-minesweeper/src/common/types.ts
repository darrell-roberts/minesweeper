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

export type ModifiedPosition = {
  pos: { x: number, y: number },
  cell: Cell
}

export type OpenResult = {
  openedCells: Position[],
  gameState: GameState,
  totalMines: number,
}

export type FlagResult = {
    position?: Position,
}

export type TimeEvent = {
    duration: string,
}

export type GameState = "New" | "Active" | "Win" | "Loss";
