import { useCallback, useEffect, useRef, useState } from "react";

const API_BASE = "http://localhost:3000/api/v1";

type PieceType = "I" | "O" | "T" | "S" | "Z" | "J" | "L";
type GameStatus = "playing" | "paused" | "game_over";
type Action = "left" | "right" | "down" | "rotate" | "drop";

interface Piece {
	type: PieceType;
	x: number;
	y: number;
	rotation: number;
}

interface Board {
	cells: number[][];
}

interface GameState {
	board: Board;
	current_piece: Piece;
	next_piece: PieceType;
	score: number;
	level: number;
	lines: number;
	status: GameStatus;
}

interface GameSession {
	sessionId: string;
	token: string;
	state: GameState;
}

// NES gravity speeds (ms per drop)
const GRAVITY_MS = [
	800, 717, 633, 550, 467, 383, 300, 217, 133, 100, 83, 83, 83, 67, 67, 67, 50,
	50, 50, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 17,
];

export function useTetris() {
	const [session, setSession] = useState<GameSession | null>(null);
	const [state, setState] = useState<GameState | null>(null);
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const tickRef = useRef<number | null>(null);

	const clearTick = useCallback(() => {
		if (tickRef.current) {
			clearInterval(tickRef.current);
			tickRef.current = null;
		}
	}, []);

	const startTick = useCallback(
		(level: number, token: string, sessionId: string) => {
			clearTick();
			const ms = GRAVITY_MS[Math.min(level, 29)];

			tickRef.current = window.setInterval(async () => {
				try {
					const res = await fetch(`${API_BASE}/games/${sessionId}/tick`, {
						method: "POST",
						headers: { Authorization: `Bearer ${token}` },
					});

					if (res.ok) {
						const data = await res.json();
						setState(data.state);

						if (data.state.status === "game_over") {
							clearTick();
						} else if (data.state.level !== level) {
							// Level changed, update tick speed
							startTick(data.state.level, token, sessionId);
						}
					}
				} catch {
					// Ignore tick errors
				}
			}, ms);
		},
		[clearTick],
	);

	const startGame = useCallback(
		async (startLevel = 0) => {
			setLoading(true);
			setError(null);
			clearTick();

			try {
				const res = await fetch(`${API_BASE}/games`, {
					method: "POST",
					headers: { "Content-Type": "application/json" },
					body: JSON.stringify({ start_level: startLevel }),
				});

				if (!res.ok) throw new Error("Failed to create game");

				const data = await res.json();
				const newSession = {
					sessionId: data.session_id,
					token: data.token,
					state: data.state,
				};

				setSession(newSession);
				setState(data.state);
				startTick(data.state.level, data.token, data.session_id);
			} catch (e) {
				setError(e instanceof Error ? e.message : "Unknown error");
			} finally {
				setLoading(false);
			}
		},
		[clearTick, startTick],
	);

	const performAction = useCallback(
		async (action: Action) => {
			if (!session || !state || state.status !== "playing") return;

			try {
				const res = await fetch(
					`${API_BASE}/games/${session.sessionId}/action`,
					{
						method: "POST",
						headers: {
							Authorization: `Bearer ${session.token}`,
							"Content-Type": "application/json",
						},
						body: JSON.stringify({ action }),
					},
				);

				if (res.ok) {
					const data = await res.json();
					setState(data.state);

					if (data.state.status === "game_over") {
						clearTick();
					}
				}
			} catch {
				// Ignore action errors
			}
		},
		[session, state, clearTick],
	);

	const endGame = useCallback(async () => {
		if (!session) return;
		clearTick();

		try {
			await fetch(`${API_BASE}/games/${session.sessionId}`, {
				method: "DELETE",
				headers: { Authorization: `Bearer ${session.token}` },
			});
		} catch {
			// Ignore
		}

		setSession(null);
		setState(null);
	}, [session, clearTick]);

	// Keyboard controls
	useEffect(() => {
		if (!state || state.status !== "playing") return;

		const handleKeyDown = (e: KeyboardEvent) => {
			switch (e.key) {
				case "ArrowLeft":
				case "a":
					e.preventDefault();
					performAction("left");
					break;
				case "ArrowRight":
				case "d":
					e.preventDefault();
					performAction("right");
					break;
				case "ArrowDown":
				case "s":
					e.preventDefault();
					performAction("down");
					break;
				case "ArrowUp":
				case "w":
					e.preventDefault();
					performAction("rotate");
					break;
				case " ":
					e.preventDefault();
					performAction("drop");
					break;
			}
		};

		window.addEventListener("keydown", handleKeyDown);
		return () => window.removeEventListener("keydown", handleKeyDown);
	}, [state, performAction]);

	// Cleanup on unmount
	useEffect(() => {
		return () => clearTick();
	}, [clearTick]);

	return {
		state,
		session,
		loading,
		error,
		startGame,
		endGame,
		performAction,
		isPlaying: state?.status === "playing",
		isGameOver: state?.status === "game_over",
	};
}
