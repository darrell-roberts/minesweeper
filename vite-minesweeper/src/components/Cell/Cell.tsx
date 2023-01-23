import { Position } from "../../common/types"
import classes from "./Cell.module.css";

type CellProps = {
    position: Position,
    open: (position: Position) => void,
}

export default function CellComp({ position, open }: CellProps) {

    function renderCell() {
        switch (position.cell.state.type) {
            case "Closed": return position.cell.state.content.flagged
                ? "🚩"
                : ""
            case "ExposedMine": return "💣"
            case "Open": return position.cell.adjacentMines > 0
                ? position.cell.adjacentMines
                : ""

        }
    }

    function getClassName() {
        switch (position.cell.state.type) {
            case "Closed": return classes.closed;
            case "ExposedMine": return classes.exposed;
            case "Open": return classes.open;
        }
    }

    //   console.log("rendering CellComp");

    return (
        <button
            className={`${classes.container} ${getClassName()}`}
            onClick={() => open(position)}>
            {renderCell()}
        </button>
    )
}
