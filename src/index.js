const wasm = require('./main.rs')

wasm.initialize({noExitRuntime: true}).then(module => {
  // Create a Javascript wrapper around our Rust function
  const runExample = module.cwrap('run_example', 'number', [])

  console.log('Calling rust functions from javascript!')
  console.log(runExample());
})
