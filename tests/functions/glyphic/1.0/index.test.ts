import { compileComponent } from "../../../compile-component.ts";
const { test } = Deno;
import { expect } from "@std/expect";

const wasmPath = "./functions/glyphic/1.0/glyphic.wasm";

const {
  glyphicApi: {
    testPing,
    getCalls,
    getCallById,
    getCallMediaById,
    getCallSnippetsById,
  },
} = await compileComponent(wasmPath);

// Get API key from environment variable
const apiKey = Deno.env.get("GLYPHIC_API_KEY") || "test_api_key";

test("test ping works", () => {
  const value = testPing(apiKey);
  expect(value).toBeTruthy();
});

test("get calls works", () => {
  // Test with empty query params
  const value = getCalls(apiKey, "{}");
  expect(value).toBeTruthy();
});

test("get calls with filters works", () => {
  // Test with query parameters
  const queryParams = JSON.stringify({
    limit: 10,
    direction: "next",
  });
  const value = getCalls(apiKey, queryParams);
  expect(value).toBeTruthy();
});

test("get call by id works", () => {
  const testCallId = "5eb7cf5a86d9755df3a6c593";
  const value = getCallById(apiKey, testCallId);
  expect(value).toBeTruthy();
});

test("get call media by id works", () => {
  const testCallId = "5eb7cf5a86d9755df3a6c593";
  const value = getCallMediaById(apiKey, testCallId);
  expect(value).toBeTruthy();
});

test("get call snippets by id works", () => {
  const testCallId = "5eb7cf5a86d9755df3a6c593";
  const value = getCallSnippetsById(apiKey, testCallId);
  expect(value).toBeTruthy();
});
