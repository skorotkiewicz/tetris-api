import type { FC } from "react";

export const Controls: FC = () => {
	return (
		<div className="glass rounded-xl p-4">
			<div className="text-xs text-[var(--text-secondary)] uppercase tracking-widest mb-3">
				Controls
			</div>
			<div className="grid grid-cols-2 gap-2 text-xs text-[var(--text-secondary)]">
				<div className="flex items-center gap-2">
					<kbd className="px-2 py-1 bg-[var(--bg-tertiary)] rounded text-[var(--text-primary)]">
						←
					</kbd>
					<span>Left</span>
				</div>
				<div className="flex items-center gap-2">
					<kbd className="px-2 py-1 bg-[var(--bg-tertiary)] rounded text-[var(--text-primary)]">
						→
					</kbd>
					<span>Right</span>
				</div>
				<div className="flex items-center gap-2">
					<kbd className="px-2 py-1 bg-[var(--bg-tertiary)] rounded text-[var(--text-primary)]">
						↓
					</kbd>
					<span>Soft Drop</span>
				</div>
				<div className="flex items-center gap-2">
					<kbd className="px-2 py-1 bg-[var(--bg-tertiary)] rounded text-[var(--text-primary)]">
						↑
					</kbd>
					<span>Rotate</span>
				</div>
				<div className="flex items-center gap-2 col-span-2">
					<kbd className="px-3 py-1 bg-[var(--bg-tertiary)] rounded text-[var(--text-primary)]">
						Space
					</kbd>
					<span>Hard Drop</span>
				</div>
			</div>
		</div>
	);
};
