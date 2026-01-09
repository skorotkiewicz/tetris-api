import type { FC } from "react";

interface Piece {
	type: "I" | "O" | "T" | "S" | "Z" | "J" | "L";
	x: number;
	y: number;
	rotation: number;
}

interface Board {
	cells: number[][];
}

interface BoardProps {
	board: Board;
	currentPiece?: Piece;
}

// Piece shapes for rendering current piece
const PIECE_SHAPES: Record<string, Record<number, number[][]>> = {
	I: {
		0: [
			[0, 0, 0, 0],
			[1, 1, 1, 1],
			[0, 0, 0, 0],
			[0, 0, 0, 0],
		],
		1: [
			[0, 0, 1, 0],
			[0, 0, 1, 0],
			[0, 0, 1, 0],
			[0, 0, 1, 0],
		],
	},
	O: {
		0: [
			[0, 0, 0, 0],
			[0, 1, 1, 0],
			[0, 1, 1, 0],
			[0, 0, 0, 0],
		],
	},
	T: {
		0: [
			[0, 0, 0, 0],
			[1, 1, 1, 0],
			[0, 1, 0, 0],
			[0, 0, 0, 0],
		],
		1: [
			[0, 1, 0, 0],
			[1, 1, 0, 0],
			[0, 1, 0, 0],
			[0, 0, 0, 0],
		],
		2: [
			[0, 1, 0, 0],
			[1, 1, 1, 0],
			[0, 0, 0, 0],
			[0, 0, 0, 0],
		],
		3: [
			[0, 1, 0, 0],
			[0, 1, 1, 0],
			[0, 1, 0, 0],
			[0, 0, 0, 0],
		],
	},
	S: {
		0: [
			[0, 0, 0, 0],
			[0, 1, 1, 0],
			[1, 1, 0, 0],
			[0, 0, 0, 0],
		],
		1: [
			[1, 0, 0, 0],
			[1, 1, 0, 0],
			[0, 1, 0, 0],
			[0, 0, 0, 0],
		],
	},
	Z: {
		0: [
			[0, 0, 0, 0],
			[1, 1, 0, 0],
			[0, 1, 1, 0],
			[0, 0, 0, 0],
		],
		1: [
			[0, 1, 0, 0],
			[1, 1, 0, 0],
			[1, 0, 0, 0],
			[0, 0, 0, 0],
		],
	},
	J: {
		0: [
			[0, 0, 0, 0],
			[1, 1, 1, 0],
			[0, 0, 1, 0],
			[0, 0, 0, 0],
		],
		1: [
			[0, 1, 0, 0],
			[0, 1, 0, 0],
			[1, 1, 0, 0],
			[0, 0, 0, 0],
		],
		2: [
			[1, 0, 0, 0],
			[1, 1, 1, 0],
			[0, 0, 0, 0],
			[0, 0, 0, 0],
		],
		3: [
			[0, 1, 1, 0],
			[0, 1, 0, 0],
			[0, 1, 0, 0],
			[0, 0, 0, 0],
		],
	},
	L: {
		0: [
			[0, 0, 0, 0],
			[1, 1, 1, 0],
			[1, 0, 0, 0],
			[0, 0, 0, 0],
		],
		1: [
			[1, 1, 0, 0],
			[0, 1, 0, 0],
			[0, 1, 0, 0],
			[0, 0, 0, 0],
		],
		2: [
			[0, 0, 1, 0],
			[1, 1, 1, 0],
			[0, 0, 0, 0],
			[0, 0, 0, 0],
		],
		3: [
			[0, 1, 0, 0],
			[0, 1, 0, 0],
			[0, 1, 1, 0],
			[0, 0, 0, 0],
		],
	},
};

const PIECE_COLORS: Record<number, string> = {
	1: "piece-I",
	2: "piece-O",
	3: "piece-T",
	4: "piece-S",
	5: "piece-Z",
	6: "piece-J",
	7: "piece-L",
};

const PIECE_TYPE_TO_NUM: Record<string, number> = {
	I: 1,
	O: 2,
	T: 3,
	S: 4,
	Z: 5,
	J: 6,
	L: 7,
};

export const Board: FC<BoardProps> = ({ board, currentPiece }) => {
	// Create a copy of the board with the current piece overlaid
	const displayCells = board.cells.map((row) => [...row]);

	if (currentPiece) {
		const shape = PIECE_SHAPES[currentPiece.type];
		const rotations = Object.keys(shape).length;
		const rotation = currentPiece.rotation % rotations;
		const pieceGrid = shape[rotation];
		const pieceNum = PIECE_TYPE_TO_NUM[currentPiece.type];

		for (let py = 0; py < 4; py++) {
			for (let px = 0; px < 4; px++) {
				if (pieceGrid[py][px]) {
					const boardY = currentPiece.y + py;
					const boardX = currentPiece.x + px;
					if (boardY >= 0 && boardY < 20 && boardX >= 0 && boardX < 10) {
						displayCells[boardY][boardX] = pieceNum;
					}
				}
			}
		}
	}

	return (
		<div className="glass rounded-xl p-3 shadow-2xl">
			<div
				className="grid gap-[1px] bg-[var(--cell-border)] rounded-lg overflow-hidden"
				style={{
					gridTemplateColumns: "repeat(10, 1fr)",
					width: "240px",
				}}
			>
				{displayCells.map((row, y) =>
					row.map((cell, x) => (
						<div
							key={`${y}-${x}`}
							className={`
                aspect-square w-6 transition-all duration-75
                ${cell ? PIECE_COLORS[cell] : "bg-[var(--cell-empty)]"}
              `}
						/>
					)),
				)}
			</div>
		</div>
	);
};
