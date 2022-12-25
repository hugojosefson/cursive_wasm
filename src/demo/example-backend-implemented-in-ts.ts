// @deno-types="../../pkg/cursive_wasm.d.ts"
import { Cursive, CursiveBackend, Vec2 } from "../../pkg/cursive_wasm.js";

class CursiveBackendImpl implements CursiveBackend {
  print(s: string): void {
    console.log(`CursiveBackendImpl: print(${s})`);
  }

  setRaw(raw: boolean): void {
    console.log(`CursiveBackendImpl: setRaw(${raw})`);
  }

  hasColors(): boolean {
    console.log(`CursiveBackendImpl: hasColors()`);
    return true;
  }

  pollEvent(): string {
    console.log(`CursiveBackendImpl: pollEvent()`);
    return "SOME_EVENT";
  }

  screenSize(): Vec2 {
    return new Vec2(88, 240);
  }

  setTitle(title: string): void {
    console.log(`CursiveBackendImpl: setTitle(${title})`);
  }
}

const backend: CursiveBackend = new CursiveBackendImpl();
const wasmCursive: Cursive = new Cursive(backend);

wasmCursive.printSomethingInRawMode();

const haveColors = wasmCursive.checkMyColors();
console.log(`haveColors: ${haveColors}`);
const screenSize = wasmCursive.checkMyScreenSize();
console.log(`screenSize: ${screenSize.x}x${screenSize.y}`);
wasmCursive.callMe();
