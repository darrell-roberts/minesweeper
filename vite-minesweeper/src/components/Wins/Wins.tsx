import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api'
import { WinHistory, Win } from "../../common/types";
import classes from "./Wins.module.css";

type WinsProps = {
    close: () => void,
};

function Wins({ close }: WinsProps) {
    const [wins, setWins] = useState<WinHistory>();
    const [closing, setClosing] = useState(false);

    useEffect(() => {
        invoke<WinHistory>("get_win_history")
            .then(setWins)
            .catch(err => console.error("failed to get win history", err));
        return () => {
            invoke("resume").catch(err => console.error("Failed to resume clock", err));
        }
    }, []);

    const closeDialog = () => {
        setClosing(true);
    };

    return (
        <div className={closing ? `${classes.modal} ${classes.closing}` : classes.modal}
            onAnimationEnd={() => {
                if (closing) {
                    close();
                }
            }}>
            <div className={classes.container}>
                <div className={classes.closeButton} onClick={closeDialog}>X</div>
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
