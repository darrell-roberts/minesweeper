import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { WinHistory, Win } from "../../common/types";
import classes from "./Wins.module.css";

type WinsProps = {
	close: () => void;
};

function Wins({ close }: WinsProps) {
	const [wins, setWins] = useState<WinHistory>();
	const [closing, setClosing] = useState(false);
	const containerDiv = useRef<HTMLDivElement>(null);
	const [containerClasses, setContainerClasses] = useState(classes["modal"]);
	const [height, setHeight] = useState("0px");

	useEffect(() => {
		invoke<WinHistory>("get_win_history")
			.then(setWins)
			.catch((err) => console.error("failed to get win history", err));
		return () => {
			invoke("resume").catch((err) =>
				console.error("Failed to resume clock", err),
			);
		};
	}, []);

	useLayoutEffect(() => {
		if (containerDiv.current) {
			const { height } = containerDiv.current.getBoundingClientRect();
			setHeight(`-${height}px`);
			setContainerClasses(`${classes["modal"]} ${classes["modalReady"]}`);
		}
	}, [containerDiv]);

	useEffect(() => {
		if (closing) {
			setContainerClasses(`${classes["modal"]} ${classes["closing"]}`);
		}
	}, [closing]);

	const closeDialog = () => {
		setClosing(true);
	};

	return (
		<div
			className={containerClasses}
			onAnimationEnd={() => {
				if (closing) {
					close();
				}
			}}
			ref={containerDiv}
			style={{ "--wins-height": height } as React.CSSProperties}
		>
			<div className={classes["container"]}>
				<div className={classes["closeButton"]} onClick={closeDialog}>
					X
				</div>
				<div className={classes["title"]}>Top 10 Wins</div>
				{wins?.wins &&
					wins.wins.map((win, index) => (
						<WinComponent key={win.date} win={win} rank={index + 1} />
					))}
				{!wins && <span className={classes["noWins"]}>No wins yet.</span>}
			</div>
		</div>
	);
}

type WinComponentProps = {
	win: Win;
	rank: number;
};

const WinComponent = ({ win, rank }: WinComponentProps) => (
	<div className={classes["winComponent"]}>
		<div className={classes["rank"]}>
			<span>{rank}.</span>
		</div>

		<div className={classes["win"]}>
			<div className={classes["duration"]}>{win.duration}</div>
			<div className={classes["date"]}>{win.date}</div>
		</div>
	</div>
);

export default Wins;
