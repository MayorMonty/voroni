import "./main.css"

const verticalDemos = window.matchMedia("(max-aspect-ratio: 1/1)").matches;
const dimensions = verticalDemos ? [900, 1600] : [1600, 900];


function demo1(wasm) {
  const canvas = document.getElementById("demo-1");

  canvas.width = dimensions[0];
  canvas.height = dimensions[1];

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

  // Divide by 5 to ensure reasonable computation time
  canvas.width = dimensions[0] / 5;
  canvas.height = dimensions[1] / 5;

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

  canvas.width = dimensions[0];
  canvas.height = dimensions[1];

  const context = canvas.getContext("2d");

  const go = document.getElementById("demo-3-go");
  const points = document.getElementById("demo-3-points");

  go.addEventListener("click", () => {
    context.clearRect(0, 0, canvas.width, canvas.height);
    wasm.demo3(canvas, +points.value);
  });


  wasm.demo3(canvas, +points.value);
}

function demo4(wasm) {
  const canvas = document.getElementById("demo-4");

  canvas.width = dimensions[0];
  canvas.height = dimensions[1];

  const context = canvas.getContext("2d");

  const go = document.getElementById("demo-4-go");
  const points = document.getElementById("demo-4-points");

  go.addEventListener("click", () => {
    context.clearRect(0, 0, canvas.width, canvas.height);
    wasm.demo4(canvas, +points.value);
  });


  wasm.demo4(canvas, +points.value);
}

function demo5(wasm) {

  // For mobile screens, flip the canvas to portrait mode, and go full screen.


};


import("./pkg")
  .then((wasm) => {

    // Set up good error reporting
    wasm.initialize();

    // Demo 1: Highlight the closest point
    demo1(wasm);

    // Demo 2: Finds the closest site and colorize it
    demo2(wasm);

    // Demo 3: Finds and draws all of the perpendicular bisectors
    demo3(wasm);

    // Demo 3: Finds and draws all of the perpendicular bisectors, limiting their range appropriately
    demo4(wasm);

    demo5(wasm);
  

  })
  .catch(console.error);
