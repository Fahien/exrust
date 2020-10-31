import * as webgl from "webgl";

var ctx = webgl.Context.new();

var canvas = document.getElementById("area");
canvas.onclick = (event) => {
    const x = (event.clientX / canvas.clientWidth) * 2.0 - 1.0;
    const y = - ((event.clientY / canvas.clientHeight) * 2.0 - 1.0);
    console.log(x + " " + y);
    ctx.draw_point(x, y);
}

ctx.draw_point(0.0, 0.0);
