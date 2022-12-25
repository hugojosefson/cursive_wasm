// @deno-types="../../pkg/cursive_wasm.d.ts"
import {
  CursiveBackend,
  doSomethingWithMyBackend,
} from "../../pkg/cursive_wasm.js";

class CursiveBackendImpl implements CursiveBackend {
  print(s: string): void {
    console.log(`CursiveBackendImpl: print(${s})`);
  }
  setRaw(raw: boolean): void {
    console.log(`CursiveBackendImpl: setRaw(${raw})`);
  }
}

const backend: CursiveBackend = new CursiveBackendImpl();
doSomethingWithMyBackend(backend);
