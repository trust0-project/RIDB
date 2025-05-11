// @ts-ignore @ignore
import wasmBuffer from "@trust0/ridb-core/wasm";

let loaded : typeof import("@trust0/ridb-core") | undefined;

export async function WasmInternal() {
    if (!loaded) {
        const module = await import("@trust0/ridb-core");
        const wasmInstance = module.initSync(wasmBuffer);
        await module.default(wasmInstance);
        loaded = module;
    }
    return loaded;
};
