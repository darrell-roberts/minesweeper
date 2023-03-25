import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api'
import { WinHistory, Win } from "../../common/types";
import classes from "./Wins.module.css";

type WinsProps = {
    close: () => void,
};

function Wins({ close }: WinsProps) {
    const [wins, setWins] = useState<WinHistory>();

    useEffect(() => {
        invoke<WinHistory>("get_win_history")
            .then(setWins)
            .catch(err => console.error("failed to get win history", err));
        return () => {
            invoke("resume").catch(err => console.error("Failed to resume clock", err));
        }
    }, []);

    return (
        <div className={classes.modal}>
            <div className={classes.container}>
                <div className={classes.closeButton} onClick={close}>X</div>
                <div className={classes.title}>Top 10 Wins</div>
                {wins?.wins &&
                    wins.wins.map((win, index) =>
                        <WinComponent
                            key={win.date}
                            win={win}
                            rank={index + 1}
                        />
                    )
                }
                {
                    !wins && <span className={classes.noWins}>No wins yet.</span>
                }
            </div>
        </div>
    )
}

type WinComponentProps = {
    win: Win,
    rank: number,

};

const WinComponent = ({ win, rank }: WinComponentProps) => (
    <div className={classes.winComponent}>
        <div className={classes.rank}>
            <span >{rank}.</span>
        </div>

        <div className={classes.win}>
            <div className={classes.duration}>{win.duration}</div>
            <div className={classes.date}>{win.date}</div>
        </div>

    </div>
);

export default Wins;
