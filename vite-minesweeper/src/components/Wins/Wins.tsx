import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api'
import { WinHistory, Win } from "../../common/types";
import classes from "./Wins.module.css";

function Wins() {
  const [wins, setWins] = useState<WinHistory>();

  useEffect(() => {
    invoke<WinHistory>("get_win_history")
      .then(setWins)
      .catch(err => console.error("failed to get win history", err));
  }, []);

  console.info("wins", wins);

  return (
    <div>
      {wins?.wins &&
        wins.wins.map((win, index) => <WinComponent win={win} rank={index + 1} />)
      }
      {
        !wins && <span className={classes.noWins}>No wins yet.</span>
      }
    </div>
  )
}

type WinComponentProps = {
  win: Win,
  rank: number
}

const WinComponent = ({ win, rank }: WinComponentProps) => (
  <div className={classes.container}>
    <span className={classes.rank}>{rank}.</span>
    <div className={classes.win}>
      <span>{win.date}</span>
      <span>{win.duration}</span>
    </div>
  </div>
);

export default Wins;