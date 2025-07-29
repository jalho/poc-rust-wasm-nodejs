import * as lib from "./ffi-wasm/pkg/ffi_wasm.js";

async function main() {
  const num_1: number = lib.get_one();
  console.log("DEBUG num_1", num_1);

  const log_entry: lib.LogEntry = lib.make_log_entry();
  const timestamp: bigint = log_entry.as_timestamp();
  console.log("DEBUG timestamp", timestamp);

  log_entry.free();
}
main();
