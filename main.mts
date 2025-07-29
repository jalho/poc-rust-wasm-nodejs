import * as lib from "./ffi-wasm/pkg/ffi_wasm.js";

async function main() {
  const num_1: number = lib.get_one();
  console.log("DEBUG num_1", num_1);

  const log_entry: lib.LogEntry = lib.make_log_entry();
  console.log("DEBUG log_entry", log_entry);
  console.log("DEBUG log_entry instanceof lib.LogEntry", log_entry instanceof lib.LogEntry);

  const timestamp_millis: bigint = log_entry.as_timestamp_millis();
  console.log("DEBUG timestamp_millis", timestamp_millis);

  const timestamp_rfc3339: string = log_entry.as_timestamp_rfc3339();
  console.log("DEBUG timestamp_rfc3339", timestamp_rfc3339);

  log_entry.free();
}
main();
