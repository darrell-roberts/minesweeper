import { useEffect, useState } from "react";
import { WebviewWindow } from "@tauri-apps/api/window";
import { GameState, TimeEvent } from "../../common/types";

/**
 * A Duration counter that renders time elapsed.
 */
function DurationCounter({ gameState }: { gameState: GameState }) {
    const [duration, setDuration] = useState("0 seconds");

    useEffect(() => {
        const unListen = new WebviewWindow("main").listen<TimeEvent>("time-event", event => {
            setDuration(event.payload.duration);
        })
        return () => {
            unListen.then(f => f());
        }
    }, []);

    useEffect(() => {
        if (gameState === "New") {
            setDuration("0 seconds");
        }
    }, [gameState])

    return (
        <>
            <span>Duration: {duration}</span>
        </>
    )
}

export default DurationCounter;