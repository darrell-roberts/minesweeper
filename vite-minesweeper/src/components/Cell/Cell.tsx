import { Position } from "../../common/types"
import classes from "./Cell.module.css";
import { MouseEvent, useEffect, useState } from "react";

type CellProps = {
    position: Position,
    open: (position: Position) => Promise<void>,
    // flag: (flagged: boolean) => void,
    flag: (position: Position) => Promise<Position | undefined>,
    gameActive: boolean,
}

export default function CellComp({ position, open, gameActive, flag }: CellProps) {
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

    async function handleClick(event: MouseEvent) {
        if (event.altKey) {
            try {
                const pos = await flag(position);
                if (pos) {
                    setLocalPos(pos);
                }
            } catch (err) {
                console.error("failed to flag cell", err)
            }
        } else {
            if (localPos.cell.state.type == "Closed") {
                if (!localPos.cell.state.content.flagged) {
                    try {
                        await open(position);
                        setLocalPos(position);
                    } catch (err) {
                        console.error("failed to open cell", err);
                    }
                }
            }
        }
    }

    return (
        <button
            className={`${classes.container} ${getClassName()}`}
            onClick={handleClick}
            disabled={!gameActive}>
            {renderCell()}
        </button>
    )
}
