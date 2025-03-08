
import wasmBuffer from "@trust0/ridb-core/pkg/ridb_core_bg.wasm";

const module = await import("@trust0/ridb-core");
const wasmInstance = module.initSync(wasmBuffer);
await module.default(wasmInstance);

export default module;