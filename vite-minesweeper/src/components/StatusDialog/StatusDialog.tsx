import classes from "./StatusDialog.module.css";

type StatusDialogProps = {
    close: () => void;
    message: string;
};

function StatusDialog({ close, message }: StatusDialogProps) {
    return (
        <div className={classes.modal}>
            <div className={classes.container}>
                <div className={classes.closeButton} onClick={close}>X</div>
                <div className={classes.content}>
                    <div className={classes.message}>
                        {message}
                    </div>
                </div>
            </div>
        </div>
    )
}

export default StatusDialog;