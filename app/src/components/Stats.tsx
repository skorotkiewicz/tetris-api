import type { FC } from "react";

interface StatsProps {
	score: number;
	level: number;
	lines: number;
}

export const Stats: FC<StatsProps> = ({ score, level, lines }) => {
	return (
		<div className="glass rounded-xl p-4 space-y-4">
			<div>
				<div className="text-xs text-[var(--text-secondary)] uppercase tracking-widest">
					Score
				</div>
				<div className="text-2xl font-bold tabular-nums text-[var(--accent)]">
					{score.toLocaleString()}
				</div>
			</div>

			<div className="flex gap-6">
				<div>
					<div className="text-xs text-[var(--text-secondary)] uppercase tracking-widest">
						Level
					</div>
					<div className="text-xl font-semibold tabular-nums">{level}</div>
				</div>

				<div>
					<div className="text-xs text-[var(--text-secondary)] uppercase tracking-widest">
						Lines
					</div>
					<div className="text-xl font-semibold tabular-nums">{lines}</div>
				</div>
			</div>
		</div>
	);
};
