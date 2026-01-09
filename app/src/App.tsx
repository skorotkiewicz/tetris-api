import { useState } from "react";
import { Board } from "./components/Board";
import { Controls } from "./components/Controls";
import { NextPiece } from "./components/NextPiece";
import { Stats } from "./components/Stats";
import { useTetris } from "./hooks/useTetris";

function App() {
	const { state, loading, error, startGame, endGame, isPlaying, isGameOver } =
		useTetris();
	const [startLevel, setStartLevel] = useState(0);

	return (
		<div className="min-h-screen flex flex-col items-center justify-center p-8">
			{/* Title */}
			<h1 className="text-4xl font-bold mb-8 tracking-tight">
				<span className="text-[var(--accent)]">Tetris</span>
				<span className="text-[var(--text-secondary)] text-lg ml-2 font-normal">
					API
				</span>
			</h1>

			{!state ? (
				// Start Screen
				<div className="glass rounded-2xl p-8 animate-fade-in text-center max-w-sm">
					<div className="mb-6">
						<label className="block text-sm text-[var(--text-secondary)] mb-2">
							Starting Level
						</label>
						<div className="flex items-center justify-center gap-4">
							<button
								onClick={() => setStartLevel((l) => Math.max(0, l - 1))}
								className="w-10 h-10 rounded-lg bg-[var(--bg-tertiary)] hover:bg-[var(--bg-secondary)] transition-colors text-xl"
							>
								−
							</button>
							<span className="text-3xl font-bold w-12 text-center tabular-nums">
								{startLevel}
							</span>
							<button
								onClick={() => setStartLevel((l) => Math.min(19, l + 1))}
								className="w-10 h-10 rounded-lg bg-[var(--bg-tertiary)] hover:bg-[var(--bg-secondary)] transition-colors text-xl"
							>
								+
							</button>
						</div>
					</div>

					<button
						onClick={() => startGame(startLevel)}
						disabled={loading}
						className="w-full py-3 px-6 bg-[var(--accent)] hover:opacity-90 disabled:opacity-50 rounded-xl font-medium transition-all shadow-lg shadow-[var(--accent-glow)]"
					>
						{loading ? "Starting..." : "Start Game"}
					</button>

					{error && (
						<p className="mt-4 text-sm text-red-400">
							{error}. Is the API running on port 3000?
						</p>
					)}

					<p className="mt-6 text-xs text-[var(--text-secondary)]">
						Classic NES Tetris · REST API
					</p>
				</div>
			) : (
				// Game Screen
				<div className="flex gap-6 animate-fade-in">
					{/* Board */}
					<Board board={state.board} currentPiece={state.current_piece} />

					{/* Sidebar */}
					<div className="flex flex-col gap-4 w-40">
						<NextPiece piece={state.next_piece} />
						<Stats
							score={state.score}
							level={state.level}
							lines={state.lines}
						/>
						<Controls />

						{isGameOver && (
							<div className="glass rounded-xl p-4 text-center">
								<div className="text-lg font-bold text-red-400 mb-2">
									Game Over
								</div>
								<button
									onClick={() => {
										endGame();
										startGame(startLevel);
									}}
									className="w-full py-2 px-4 bg-[var(--accent)] hover:opacity-90 rounded-lg text-sm font-medium transition-all"
								>
									Play Again
								</button>
							</div>
						)}

						{isPlaying && (
							<button
								onClick={endGame}
								className="py-2 px-4 bg-[var(--bg-tertiary)] hover:bg-[var(--bg-secondary)] rounded-lg text-sm text-[var(--text-secondary)] transition-colors"
							>
								Quit
							</button>
						)}
					</div>
				</div>
			)}
		</div>
	);
}

export default App;
