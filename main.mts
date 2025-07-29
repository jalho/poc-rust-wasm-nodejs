import * as lib from "./ffi-wasm/pkg/ffi_wasm.js";

async function main() {
  const value = lib.get_one();
  console.log("DEBUG", value);
}
main();
