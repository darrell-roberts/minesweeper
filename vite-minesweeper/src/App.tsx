import { useEffect, useState } from 'react'
import './App.css'
import { invoke } from '@tauri-apps/api'
import { GameState, OpenResult, Position } from "./common/types";
import CellComp from "./components/Cell/Cell";
import { message } from '@tauri-apps/api/dialog';

function App() {
    const [board, setBoard] = useState<Position[]>([]);
    const [gameState, setGameState] = useState<GameState>("New");

    useEffect(() => {
        invoke<Position[]>("get_positions")
            .then(setBoard)
            .catch(err => console.error("failed to get positions", err));
    }, []);

    useEffect(() => {
        if (gameState === "Win" || gameState === "Loss") {
            message(gameState, "Game Status")
                .catch((err) => console.error("Failed to open dialog", err));
        }
    }, [gameState]);

    async function openCell(position: Position) {
        if (position.cell.state.type === "Closed") {
            console.log("opening position", position);
            const result = await invoke<OpenResult>("open", { position });
            setGameState(result.gameState);
            console.log("opened result", result);

            const updatedBoard = [...board];
            for (const opened of result.openedCells) {
                updatedBoard[opened.index].cell = opened.cell
            }
            setBoard(updatedBoard);
        }
    }

    function newGame() {
        invoke<Position[]>("new_game")
            .then(setBoard)
            .catch(err => console.error("Failed to start game", err));
    }

    return (
        <div className="App">
            {board.length > 0 &&
                <div className={`board ${gameState === "Loss" ? ".gameOver" : ""}`}>
                    {board.map(cell => <CellComp position={cell} open={openCell} />)}
                </div>
            }
            <button className="newGame" onClick={() => newGame()}>
                New Game
            </button>
        </div>
    )
}

export default App
