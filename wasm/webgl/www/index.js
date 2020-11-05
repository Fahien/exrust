import * as webgl from "webgl";

var ctx = webgl.Context.new();

const tick = () => {
    ctx.draw_primitive();
    requestAnimationFrame(tick);
}

requestAnimationFrame(tick);
