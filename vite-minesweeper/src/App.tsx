import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import './App.css'
import { invoke } from '@tauri-apps/api'
import { Position } from "./common/types";
import CellComp from "./components/Cell";

function App() {
  const [board, setBoard] = useState<Position[]>([]);

  useEffect(() => {
    invoke<Position[]>("get_positions").then(setBoard).catch(err => console.error("failed to get positions", err));
  }, []);

  return (
    <div className="App">
      {board.length > 0 &&
        <div className="board">
          {board.map(cell => <CellComp position={cell} />)}
        </div>
      }
    </div>
  )
}

export default App
