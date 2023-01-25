import { useEffect, useState } from 'react'
import './App.css'
import { invoke } from '@tauri-apps/api'
import { GameState, OpenResult, Position } from "./common/types";
import CellComp from "./components/Cell/Cell";
import { message } from '@tauri-apps/api/dialog';

type GameAppState = {
    board: Position[],
    state: GameState,
}

function App() {
    const [gameState, setGameState] = useState<GameAppState>({ board: [], state: "New" });
    const gameActive = gameState.state === "Active" || gameState.state === "New";

    useEffect(() => {
        newGame()
    }, []);

    useEffect(() => {
        if (!gameActive) {
            message(gameState.state, "Game Status")
                .catch((err) => console.error("Failed to open dialog", err));
        }
    }, [gameState.state]);

    async function openCell(position: Position) {
        if (position.cell.state.type === "Closed") {
            const result = await invoke<OpenResult>("open", { position });

            const updatedBoard = [...gameState.board];
            for (const opened of result.openedCells) {
                updatedBoard[opened.index].cell = opened.cell
            }
            setGameState({
                board: updatedBoard,
                state: result.gameState
            });
        }
    }

    function newGame() {
        invoke<Position[]>("new_game")
            .then(board => setGameState({ board, state: "New" }))
            .catch(err => console.error("Failed to start game", err));
    }

    return (
        <div className="App">
            {gameState.board.length > 0 &&
                <div className={`board ${!gameActive ? "gameOver" : ""}`}>
                    {gameState.board.map(cell => <CellComp position={cell} open={openCell} gameActive={gameActive} />)}
                </div>
            }
            <button className="newGame" onClick={() => newGame()} >
                New Game
            </button>
        </div>
    )
}

export default App
