// @ts-ignore @ignore
import wasmBuffer from "@trust0/ridb-core/wasm";
import * as ridbCore from "@trust0/ridb-core";

let loaded: typeof import("@trust0/ridb-core") | undefined;

export async function WasmInternal() {
  if (!loaded) {
    ridbCore.initSync(wasmBuffer);
    loaded = ridbCore;
  }
  return loaded;
}
