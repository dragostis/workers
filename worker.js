importScripts("pkg/workers.js");

const { Counter } = wasm_bindgen;

onmessage = async (event) => {
  // Instantiate the module with the same WebAssembly.Memory as in the main
  // Worker.
  await wasm_bindgen(...event.data[0]);

  // Initialize the Counter from the pointer.
  const counter = Counter.from_raw(event.data[1]);

  console.log(counter.incr());
};
