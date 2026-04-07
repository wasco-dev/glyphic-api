import { compileComponent } from "../../../compile-component.ts";
const { test } = Deno;
import { expect } from "@std/expect";

const wasmPath = "./functions/glyphic/1.0/glyphic.wasm";

const {
  glyphic: { getCalls, getCallById, getCallMediaById, getCallSnippetsById },
} = await compileComponent(wasmPath);

test("get calls works", () => {
  const value = getCalls();

  expect(value).toBeTruthy();
});

test("get call by id works", () => {
  const value = getCallById();

  expect(value).toBeTruthy();
});

test("get call media by id works", () => {
  const value = getCallMediaById();

  expect(value).toBeTruthy();
});

test("get call snippets by id works", () => {
  const value = getCallSnippetsById();

  expect(value).toBeTruthy();
});
