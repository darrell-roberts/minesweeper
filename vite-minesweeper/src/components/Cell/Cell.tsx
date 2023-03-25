import { Position } from "../../common/types"
import classes from "./Cell.module.css";
import { MouseEvent, useEffect, useState } from "react";

type CellProps = {
    position: Position,
    open: (position: Position) => Promise<void>,
    flag: (position: Position) => Promise<Position | undefined>,
    gameActive: boolean,
}

function mineCountStyle(count: number): string {
    switch (count) {
        case 1: return classes.one;
        case 2: return classes.two;
        case 3: return classes.three;
        default: return classes.four;
    }
}

/**
 * A Cell component.
 */
export default function CellComp({ position, open, gameActive, flag }: CellProps) {
    const [localPos, setLocalPos] = useState(position);

    useEffect(() => {
        // addEventListener("contextmenu", async (event) => {
        //     try {
        //         const pos = await flag(position);
        //         if (pos) {
        //             setLocalPos(pos);
        //         }
        //     } catch (err) {
        //         console.error("failed to flag cell", err)
        //     }
        //     event.preventDefault();
        // });
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

    function getClassName(): string {
        switch (localPos.cell.state.type) {
            case "Closed": {
                return localPos.cell.state.content.flagged ? classes.flagged : classes.closed;
            };
            case "ExposedMine": return classes.exposed;
            case "Open": return `${classes.open} ${mineCountStyle(localPos.cell.adjacentMines)}`;
        }
    }

    async function handleClick(event: MouseEvent) {
        console.info(`button: ${event.button}`);
        if (event.altKey || event.button == 2) {
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
            onContextMenu={handleClick}
            disabled={!gameActive}>
            {renderCell()}
        </button>
    )
}
