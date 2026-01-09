import type { FC } from "react";

type PieceType = "I" | "O" | "T" | "S" | "Z" | "J" | "L";

interface NextPieceProps {
	piece: PieceType;
}

const PIECE_PREVIEWS: Record<PieceType, number[][]> = {
	I: [[1, 1, 1, 1]],
	O: [
		[1, 1],
		[1, 1],
	],
	T: [
		[1, 1, 1],
		[0, 1, 0],
	],
	S: [
		[0, 1, 1],
		[1, 1, 0],
	],
	Z: [
		[1, 1, 0],
		[0, 1, 1],
	],
	J: [
		[1, 1, 1],
		[0, 0, 1],
	],
	L: [
		[1, 1, 1],
		[1, 0, 0],
	],
};

const PIECE_CLASSES: Record<PieceType, string> = {
	I: "piece-I",
	O: "piece-O",
	T: "piece-T",
	S: "piece-S",
	Z: "piece-Z",
	J: "piece-J",
	L: "piece-L",
};

export const NextPiece: FC<NextPieceProps> = ({ piece }) => {
	const preview = PIECE_PREVIEWS[piece];
	const pieceClass = PIECE_CLASSES[piece];

	return (
		<div className="glass rounded-xl p-4">
			<div className="text-xs text-[var(--text-secondary)] uppercase tracking-widest mb-3">
				Next
			</div>
			<div className="flex items-center justify-center min-h-[48px]">
				<div
					className="grid gap-[2px]"
					style={{ gridTemplateColumns: `repeat(${preview[0].length}, 1fr)` }}
				>
					{preview.map((row, y) =>
						row.map((cell, x) => (
							<div
								key={`${y}-${x}`}
								className={`w-4 h-4 rounded-sm ${cell ? pieceClass : "bg-transparent"}`}
							/>
						)),
					)}
				</div>
			</div>
		</div>
	);
};
