import { Universe } from "game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

// On each iteration, draws the current universe to the pre and then calls Universe::tick
const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();

    requestAnimationFrame(renderLoop);
}

// Make initial call for the first frame
requestAnimationFrame(renderLoop)
