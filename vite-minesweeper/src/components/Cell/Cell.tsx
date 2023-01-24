import { Position } from "../../common/types"
import classes from "./Cell.module.css";
import { invoke } from '@tauri-apps/api'
import { useEffect, useState } from "react";

type CellProps = {
    position: Position,
    open: (position: Position) => void,
    gameActive: boolean,
}

export default function CellComp({ position, open, gameActive }: CellProps) {
    const [localPos, setLocalPos] = useState(position);

    useEffect(() => {
        setLocalPos(position);
    }, [position])

    function renderCell() {
        switch (localPos.cell.state.type) {
            case "Closed": return localPos.cell.state.content.flagged
                ? "🚩"
                : ""
            case "ExposedMine": return "💣"
            case "Open": return localPos.cell.adjacentMines > 0
                ? localPos.cell.adjacentMines
                : ""

        }
    }

    function getClassName() {
        switch (localPos.cell.state.type) {
            case "Closed": return classes.closed;
            case "ExposedMine": return classes.exposed;
            case "Open": return classes.open;
        }
    }

    return (
        <button
            className={`${classes.container} ${getClassName()}`}
            onClick={event => {
                if (event.altKey) {
                    invoke<Position | undefined>("flag", { position }).then(pos => {
                        if (pos) {
                            setLocalPos(pos);
                        }
                    })
                } else {
                    open(position);
                }
            }}
            disabled={!gameActive}>
            {renderCell()}
        </button>
    )
}
