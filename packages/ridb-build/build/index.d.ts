import * as tsup from 'tsup';
import { Format } from 'tsup';
import * as esbuild from 'esbuild';

declare const wasmPlugin: {
    name: string;
    setup(build: any): void;
};
declare const plugins: esbuild.Plugin[];
declare function createConfig({ format, entry, banner }: {
    format: Format | Format[] | undefined;
    entry: string[] | undefined;
    banner?: {
        js: string;
    };
}): tsup.Options | tsup.Options[] | ((overrideOptions: tsup.Options) => tsup.Options | tsup.Options[] | Promise<tsup.Options | tsup.Options[]>);

export { createConfig as default, plugins, wasmPlugin };
