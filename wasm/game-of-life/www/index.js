// Import Rust structures
import { Universe, Cell } from "game-of-life";

// Import the WebAssebly memory
import { memory } from "game-of-life/game_of_life_bg";

// FPS timer useful to investigate rendering
const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.prev = performance.now();
    }

    render() {
        const now = performance.now();
        const delta = now - this.prev;
        this.prev = now;
        const fps = (1 / delta) * 1000;

        this.frames.push(fps);
        if (this.frames.length > 100) {
            // Remove the first element
            this.frames.shift();
        }

        // Find min, max, mean
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        const mean = sum / this.frames.length;

        this.fps.textContent = `fps = ${Math.round(fps)}
avg = ${Math.round(mean)}
min = ${Math.round(min)}
max = ${Math.round(max)}`;
    }
};

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

// Keep track of the latest animation frame id
// That could be used in case we want to cancel it
let frame_id = null;

const is_paused = () => {
    return frame_id === null;
};

// On each iteration, draws the current universe to the pre and then calls Universe::tick
const render_loop = () => {
    fps.render();

    universe.tick();

    draw_grid();
    draw_cells();

    frame_id = requestAnimationFrame(render_loop);
};

const play_pause_button = document.getElementById("play-pause");

const play = () => {
    play_pause_button.textContent = "⏸️";
    render_loop();
};

const pause = () => {
    play_pause_button.textContent = "▶️";
    cancelAnimationFrame(frame_id);
    frame_id = null;
};

play_pause_button.addEventListener("click", event => {
    if (is_paused()) {
        play();
    } else {
        pause();
    }
});

canvas.addEventListener("click", event => {
    // Translate click coordinates into a row and a col
    const boundings = canvas.getBoundingClientRect();

    const scale_x = canvas.width / boundings.width;
    const scale_y = canvas.height / boundings.height;

    const left = (event.clientX - boundings.left) * scale_x;
    const top = (event.clientY - boundings.top) * scale_y;

    const row = Math.min(Math.floor(top / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(left / (CELL_SIZE + 1)), width - 1);

    console.log("Clicking at " + row + ", " + col);
    universe.toggle_cell(row, col);

    draw_grid();
    draw_cells();
});

// Triggers first animation frame
play();
