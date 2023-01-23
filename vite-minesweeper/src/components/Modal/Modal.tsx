import classes from "./Modal.module.css";

export type ModalProps = {
    message: string,
}

function Modal({ message }: ModalProps) {
    return (
        <div className={classes.modal}>
            <div className={classes.modalContent}>
                <span className={classes.close}>&times;</span>
            </div>
            <p>{message}</p>
        </div>
    )
}

export default Modal;