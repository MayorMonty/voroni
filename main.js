import "./main.css"

function demo1(wasm) {
  const canvas = document.getElementById("demo-1");
  const context = canvas.getContext("2d");

  const regenerate = document.getElementById("demo-1-regenerate");
  const points = document.getElementById("demo-1-points");

  regenerate.addEventListener("click", () => {
    context.clearRect(0, 0, canvas.width, canvas.height);
    wasm.demo1(canvas, +points.value);
  });

  wasm.demo1(canvas, +points.value);
}

function demo2(wasm) {
  const canvas = document.getElementById("demo-2");
  const context = canvas.getContext("2d");

  const go = document.getElementById("demo-2-go");
  const points = document.getElementById("demo-2-points");
  const speed = document.getElementById("demo-2-speed");

  go.addEventListener("click", () => {
    context.clearRect(0, 0, canvas.width, canvas.height);
    wasm.demo2(canvas, +points.value, +speed.value);
  });


  wasm.demo2(canvas, +points.value, +speed.value);
}



import("./pkg")
  .then((wasm) => {

    // Set up good error reporting
    wasm.initialize();

    // Demo 1: Highlight the closest point
    demo1(wasm);

    // Demo 2: BFS
    demo2(wasm);

  })
  .catch(console.error);
