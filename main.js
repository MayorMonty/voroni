import "./main.css"


import("./pkg")
  .then((wasm) => {

    wasm.initialize();

    const canvas = document.getElementById("canvas");
    const context = canvas.getContext("2d");

    const regenerate = document.getElementById("regenerate");
    const points = document.getElementById("points");

    regenerate.addEventListener("click", () => {
      context.clearRect(0, 0, canvas.width, canvas.height);
      wasm.generate(context, +points.value);
    });

    points.addEventListener("input", () => {
      context.clearRect(0, 0, canvas.width, canvas.height);
      wasm.generate(context, +points.value);
    })

    wasm.generate(context, +points.value);
  })
  .catch(console.error);
