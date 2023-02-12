import { useEffect, useState } from "react";
import { WebviewWindow } from "@tauri-apps/api/window";
import { TimeEvent } from "../../common/types";

/**
 * A Duration counter that renders time elapsed.
 */
function DurationCounter() {
    const [duration, setDuration] = useState("0 seconds");

    useEffect(() => {
        new WebviewWindow("main")
            .listen<TimeEvent>("time-event", event => setDuration(event.payload.duration))
            .then(handle => console.info("registered listener"))
            .catch(err => console.error("Failed to listen to time-event", err));

    }, []);

    return (
        <>
            <span>Duration: {duration}</span>
        </>
    )
}

export default DurationCounter;