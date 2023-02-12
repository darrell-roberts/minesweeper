import { useEffect, useReducer, } from 'react'
import './App.css'
import { invoke } from '@tauri-apps/api'
import { FlagResult, GameState, OpenResult, Position, TimeEvent } from "./common/types";
import CellComp from "./components/Cell/Cell";
import { message } from '@tauri-apps/api/dialog';
import { WebviewWindow } from "@tauri-apps/api/window";

type GameAppState = {
    board: Position[],
    state: GameState,
    opened: number,
    mined: number,
    flagged: number,
    active: boolean,
    duration: string,
}

type GameAction = { type: "open", result: OpenResult }
    | { type: "restart", board: Position[], }
    | { type: "flag", data: FlagResult }
    | { type: "duration", event: TimeEvent };

function gameReducer(state: GameAppState, action: GameAction): GameAppState {
    switch (action.type) {
        case "open": {
            const updatedBoard = [...state.board];
            for (const opened of action.result.openedCells) {
                updatedBoard[opened.index].cell = opened.cell
            }
            return {
                ...state,
                board: updatedBoard,
                state: action.result.gameState,
                active: action.result.gameState == "Active",
                opened: state.opened + action.result.openedCells.length,
                mined: action.result.totalMines,
            }
        };
        case "restart": return {
            ...INITIAL_STATE,
            board: action.board,
        };
        case "flag": return {
            ...state,
            flagged: action.data.position
                ? state.flagged + 1
                : state.flagged - 1,

        }
        case "duration": return {
            ...state,
            duration: action.event.duration,
        }
        default: return state;
    }
}

const INITIAL_STATE: GameAppState = {
    board: [],
    state: "New",
    opened: 0,
    mined: 0,
    flagged: 0,
    active: true,
    duration: "0 Seconds"
}

function App() {
    const [gameState, dispatch] = useReducer(gameReducer, INITIAL_STATE);

    useEffect(() => {
        newGame();
        new WebviewWindow("main")
            .listen<TimeEvent>("time-event", event => dispatch({ type: "duration", event: event.payload }))
            .then(handle => console.info("registered listener"))
            .catch(err => console.error("Failed to listen to time-event", err));
    }, []);

    useEffect(() => {
        if (!gameState.active) {
            message(gameState.state, "Game Status")
                .catch((err) => console.error("Failed to open dialog", err));
        }
    }, [gameState.state]);

    async function openCell(position: Position) {
        if (position.cell.state.type === "Closed") {
            try {
                const result = await invoke<OpenResult>("open", { position });
                dispatch({ type: "open", result });
            } catch (e) {
                console.error("failed to open cell", e);
            }
        }
    }

    async function flagCell(position: Position): Promise<Position | undefined> {
        const result = await invoke<FlagResult>("flag", { position });
        if (result.position) {
            if (result.position.cell.state.type == "Closed") {
                dispatch({ type: "flag", data: result })
            }
        }
        return result.position;
    }

    function newGame() {
        invoke<Position[]>("new_game")
            .then(board => dispatch({ type: "restart", board, }))
            .catch(err => console.error("Failed to start game", err));
    }

    return (
        <div className="App">
            <div className="header">
                <span>Duration: {gameState.duration}</span>
                <span>Opened Cells: {gameState.opened}</span>
                <span>Flagged Cells: {gameState.flagged}</span>
                <span>Mined Cells: {gameState.mined}</span>
            </div>
            {gameState.board.length > 0 &&
                <div className={`board ${!gameState.active ? "gameOver" : ""}`}>
                    {gameState.board.map(cell =>
                        <CellComp
                            position={cell}
                            open={openCell}
                            gameActive={gameState.active}
                            flag={flagCell}
                        />)
                    }
                </div>
            }
            <button className="newGame" onClick={() => newGame()} >
                New Game
            </button>
        </div>
    )
}

export default App
