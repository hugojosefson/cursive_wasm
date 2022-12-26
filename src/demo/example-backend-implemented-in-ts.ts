// @deno-types="../../pkg/cursive_wasm.d.ts"
import {
  Color,
  ColorPair,
  Cursive,
  CursiveBackend,
  Effect,
  Event,
  Vec2,
} from "../../pkg/cursive_wasm.js";

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

  pollEvent(): Event {
    console.log(`CursiveBackendImpl: pollEvent()`);
    return Event.Exit;
  }

  screenSize(): Vec2 {
    return new Vec2(88, 240);
  }

  setTitle(title: string): void {
    console.log(`CursiveBackendImpl: setTitle(${title})`);
  }

  refresh(): void {
    console.log(`CursiveBackendImpl: refresh()`);
  }

  printAt(pos: Vec2, text: string): void {
    console.log(`CursiveBackendImpl: printAt(${pos.x}, ${pos.y}, ${text})`);
  }

  clear(color: Color): void {
    console.log(
      `CursiveBackendImpl: clear(${color.r}, ${color.g}, ${color.b})`,
    );
    console.log(`CursiveBackendImpl: clear(${color})`);
  }

  setColor(color: ColorPair): ColorPair {
    console.log(
      `CursiveBackendImpl: setColor(${color.front.r}, ${color.front.g}, ${color.front.b}, ${color.back.r}, ${color.back.g}, ${color.back.b})`,
    );
    console.log(`CursiveBackendImpl: setColor(${color})`);
    return color;
  }

  setEffect(effect: Effect): void {
    console.log(`CursiveBackendImpl: setEffect(${Effect[effect]})`);
  }

  unsetEffect(effect: Effect): void {
    console.log(`CursiveBackendImpl: unsetEffect(${Effect[effect]})`);
  }
}

const backend: CursiveBackend = new CursiveBackendImpl();
const wasmCursive: Cursive = Cursive.letsGo(backend);

wasmCursive.something();
