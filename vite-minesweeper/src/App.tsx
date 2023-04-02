import { useEffect, useLayoutEffect, useReducer, useRef, useState, } from 'react'
import './App.css'
import { invoke } from '@tauri-apps/api'
import { FlagResult, GameState, OpenResult, Position, } from "./common/types";
import CellComp from "./components/Cell/Cell";
import DurationCounter from './components/DurationCounter/DurationCounter';
import Wins from './components/Wins/Wins';
import StatusDialog from './components/StatusDialog/StatusDialog';
import { appWindow, LogicalSize } from '@tauri-apps/api/window';

type GameAppState = {
    board: Position[],
    state: GameState,
    opened: number,
    mined: number,
    flagged: number,
    active: boolean,
    showWins: boolean,
    statusDialog: boolean
}

type GameAction = { type: "open", result: OpenResult }
    | { type: "restart", board: Position[], }
    | { type: "flag", position: Position }
    | { type: "showWins" }
    | { type: "statusDialog" };

function gameReducer(state: GameAppState, action: GameAction): GameAppState {
    switch (action.type) {
        case "open": {
            const updatedBoard = [...state.board];
            for (const opened of action.result.openedCells) {
                updatedBoard[opened.index]!.cell = opened.cell;
            }
            return {
                ...state,
                board: updatedBoard,
                state: action.result.gameState,
                active: action.result.gameState == "Active",
                opened: state.opened + action.result.openedCells.length,
                mined: action.result.totalMines,
                statusDialog: action.result.gameState === "Loss"
                    || action.result.gameState === "Win",

            }
        };
        case "restart": return {
            ...INITIAL_STATE,
            board: action.board,
        };
        case "flag": {
            const flagged = action.position.cell.state.type === "Closed"
                && action.position.cell.state.content.flagged;
            return {
                ...state,
                board: state.board.map(pos =>
                    pos.index === action.position.index
                        ? action.position : pos),
                flagged: flagged
                    ? state.flagged + 1
                    : state.flagged - 1,
            }
        }
        case "showWins": return {
            ...state,
            showWins: !state.showWins
        }
        case "statusDialog": return {
            ...state,
            statusDialog: !state.statusDialog
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
    statusDialog: false,
}

function App() {
    const [gameState, dispatch] = useReducer(gameReducer, INITIAL_STATE);
    const [resized, setResized] = useState(false);
    const [platform, setPlatform] = useState<string>();
    const [dimensions, setDimensions] = useState<LogicalSize>();
    const ref = useRef<HTMLDivElement>(null);

    useEffect(() => {
        invoke<string>("platform").then(setPlatform);
    }, []);

    useEffect(() => {
        if (dimensions) {
            appWindow.setSize(dimensions)
                .catch((err) => console.error("failed to resize", err));
        }
    }, [dimensions]);

    useLayoutEffect(() => {
        if (ref.current && gameState.board.length > 0 && !resized && platform) {
            setResized(true);
            let { width, height } = ref.current.getBoundingClientRect();
            if (platform === "mac") {
                height += 25;
            }
            setDimensions(new LogicalSize(width, height));
        }
    }, [ref.current, gameState.board, resized, platform]);


    useEffect(() => {
        addEventListener("contextmenu", (event) => {
            event.preventDefault();
        });
        newGame();
    }, []);

    async function openCell(position: Position) {
        if (position.cell.state.type === "Closed") {
            const result = await invoke<OpenResult>("open", { position });
            dispatch({ type: "open", result });
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
        <div className="App" ref={ref}>
            <div className="header">
                <DurationCounter gameState={gameState.state} />
                <span>Opened: {gameState.opened}</span>
                <span>Flagged: {gameState.flagged}</span>
                <span>Mined: {gameState.mined}</span>
            </div>

            <div className='boardContainer'>
                {
                    gameState.showWins &&
                    <Wins close={() => dispatch({ type: "showWins" })} />
                }
                {
                    gameState.statusDialog &&
                    <StatusDialog
                        close={() => dispatch({ type: "statusDialog" })}
                        message={gameState.state === "Win" ? "You Won!" : "You Lose!"}
                        emoji={gameState.state === "Win" ? "😀" : "😞"}
                    />
                }
                {gameState.board.length > 0 &&
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
                <button
                    className="buttons"
                    onClick={() => dispatch({ type: "showWins" })}
                    disabled={gameState.statusDialog || gameState.showWins}
                >
                    Top Scores
                </button>
                <button
                    className="buttons"
                    onClick={() => newGame()}
                    disabled={gameState.statusDialog || gameState.showWins}
                >
                    New Game
                </button>
            </div>

        </div>
    )
}

export default App
