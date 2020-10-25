// Import Rust structures
import { Universe, Cell } from "game-of-life";

// Import the WebAssebly memory
import { memory } from "game-of-life/game_of_life_bg";

const CELL_SIZE = 9; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.get_width();
const height = universe.get_height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

// Draws the grid: a set of equally-spaced horizontal and vertical lines
const draw_grid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const get_index = (row, column) => {
    return row * width + column;
};

const draw_cells = () => {
    // Get a pointer to the universe's cells
    const cells_ptr = universe.get_cells_ptr();

    // Construct a Uint8Array overlaying the cells buffer
    const cells = new Uint8Array(memory.buffer, cells_ptr, width * height);

    ctx.beginPath();

    // Iterate over each cell
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = get_index(row, col);

            // Draw a write or black rectangle depending on the cell state
            ctx.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOR
                : ALIVE_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

// On each iteration, draws the current universe to the pre and then calls Universe::tick
const renderLoop = () => {
    universe.tick();

    draw_grid();
    draw_cells();

    requestAnimationFrame(renderLoop);
}

// Make initial call for the first frame
requestAnimationFrame(renderLoop)
