import { useEffect, useReducer, } from 'react'
import './App.css'
import { invoke } from '@tauri-apps/api'
import { FlagResult, GameState, OpenResult, Position, } from "./common/types";
import CellComp from "./components/Cell/Cell";
import { message } from '@tauri-apps/api/dialog';
import DurationCounter from './components/DurationCounter/DurationCounter';
import Wins from './components/Wins/Wins';

type GameAppState = {
    board: Position[],
    state: GameState,
    opened: number,
    mined: number,
    flagged: number,
    active: boolean,
    showWins: boolean,
}

type GameAction = { type: "open", result: OpenResult }
    | { type: "restart", board: Position[], }
    | { type: "flag", position: Position }
    | { type: "showWins" }
    ;

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
        case "flag": {
            const flagged = action.position.cell.state.type === "Closed" && action.position.cell.state.content.flagged;
            return {
                ...state,
                board: state.board.map(pos => pos.index === action.position.index ? action.position : pos),
                flagged: flagged
                    ? state.flagged + 1
                    : state.flagged - 1,
            }
        }
        case "showWins": return {
            ...state,
            showWins: !state.showWins
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
    showWins: false,
}

function App() {
    const [gameState, dispatch] = useReducer(gameReducer, INITIAL_STATE);

    useEffect(() => {
        newGame();
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
                dispatch({ type: "flag", position: result.position })
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
                <DurationCounter gameState={gameState.state} />
                <span>Opened: {gameState.opened}</span>
                <span>Flagged: {gameState.flagged}</span>
                <span>Mined: {gameState.mined}</span>
            </div>
            <div className='boardContainer'>
                {
                    gameState.showWins && <Wins />
                }
                {gameState.board.length > 0 && !gameState.showWins &&
                    <div className={`board ${!gameState.active ? "gameOver" : ""}`}>
                        {gameState.board.map(cell =>
                            <CellComp
                                key={cell.index}
                                position={cell}
                                open={openCell}
                                gameActive={gameState.active}
                                flag={flagCell}
                            />)
                        }
                    </div>
                }
            </div>

            <div className="buttonBar">
                <button className="buttons" onClick={() => dispatch({ type: "showWins" })}>
                    {gameState.showWins ? "Hide Top Scores" : "Show Top Scores"}
                </button>
                <button className="buttons" onClick={() => newGame()} >
                    New Game
                </button>
            </div>

        </div>
    )
}

export default App
