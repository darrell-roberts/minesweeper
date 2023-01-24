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
        newGame()
    }, []);

    useEffect(() => {
        if (!gameActive) {
            message(gameState, "Game Status")
                .catch((err) => console.error("Failed to open dialog", err));
        }
    }, [gameState]);

    async function openCell(position: Position) {
        if (position.cell.state.type === "Closed") {
            const result = await invoke<OpenResult>("open", { position });
            setGameState(result.gameState);

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
            .then(() => setGameState("New"))
            .catch(err => console.error("Failed to start game", err));
    }

    const gameActive = gameState === "Active" || gameState === "New";

    return (
        <div className="App">
            {board.length > 0 &&
                <div className={`board ${!gameActive ? "gameOver" : ""}`}>
                    {board.map(cell => <CellComp position={cell} open={openCell} gameActive={gameActive} />)}
                </div>
            }
            <button className="newGame" onClick={() => newGame()} >
                New Game
            </button>
        </div>
    )
}

export default App
