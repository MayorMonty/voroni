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

  go.addEventListener("click", () => {
    context.clearRect(0, 0, canvas.width, canvas.height);
    wasm.demo2(canvas, +points.value);
  });


  wasm.demo2(canvas, +points.value);
}

function demo3(wasm) {
  const canvas = document.getElementById("demo-3");
  const context = canvas.getContext("2d");

  const go = document.getElementById("demo-3-go");
  const points = document.getElementById("demo-3-points");

  go.addEventListener("click", () => {
    context.clearRect(0, 0, canvas.width, canvas.height);
    wasm.demo3(canvas, +points.value);
  });


  wasm.demo3(canvas, +points.value);
}


import("./pkg")
  .then((wasm) => {

    // Set up good error reporting
    wasm.initialize();

    // Demo 1: Highlight the closest point
    demo1(wasm);

    // Demo 2: Finds the closest site and draws a line to it
    demo2(wasm);

    // Demo 3: Finds and draws all of the perpendicular bisectors
    demo3(wasm);
  

  })
  .catch(console.error);
