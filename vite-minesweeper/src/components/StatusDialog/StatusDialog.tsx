import { useState } from "react";
import classes from "./StatusDialog.module.css";

type StatusDialogProps = {
    close: () => void;
    message: string;
    emoji: string;
};

function StatusDialog({ close, message, emoji }: StatusDialogProps) {
    const [closing, setClosing] = useState(false);

    const closeDialog = () => {
        setClosing(true);
        setTimeout(() => close(), 250);
    }

    return (
        <div className={closing ? `${classes["modal"]} ${classes["closing"]}` : classes["modal"]}>
            <div className={classes["container"]}>
                <div className={classes["closeButton"]} onClick={closeDialog}>X</div>
                <div className={classes["content"]}>
                    <div className={classes["message"]}>
                        <div className={classes["emoji"]}>{emoji}</div>
                        <div className={classes["text"]}>{message}</div>
                    </div>
                </div>
            </div>
        </div>
    )
}

export default StatusDialog;