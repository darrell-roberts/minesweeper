import { Position } from "../common/types"
import classes from "./Cell.module.css";
import { invoke } from '@tauri-apps/api'

type CellProps = {
  position: Position
}

export default function CellComp({ position }: CellProps) {

  function renderCell() {
    switch (position.cell.state.type) {
      case "Closed": return position.cell.state.content.flagged
        ? (<div className={classes.flagged}>🚩</div>)
        : (<div className={classes.closed}>&nbsp;</div>)
      case "ExposedMine": return (
        <div className={classes.exposed}>💣</div>
      )
      case "Open": return (
        <div className={classes.open}>
          {position.cell.adjacentMines > 0
            ? position.cell.adjacentMines
            : ""
          }</div>
      )
    }
  }

  return (
    <div
      className={classes.container}
      onClick={() => invoke("open", position)}>
      {renderCell()}
    </div>
  )
}
