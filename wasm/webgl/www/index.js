import * as webgl from "webgl";

var ctx = webgl.Context.new();

const tick = () => {
    ctx.draw_triangle();
    requestAnimationFrame(tick);
}

requestAnimationFrame(tick);
