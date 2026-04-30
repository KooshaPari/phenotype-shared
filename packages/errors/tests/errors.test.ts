import { describe, it, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { HeliosAppError, ErrorCode, ERROR_CODES } from "../src/index";

const contractRoot = join(import.meta.dir, "../../../contracts/errors");

describe("HeliosAppError", () => {
  it("should create an error with code and message", () => {
    const error = new HeliosAppError(ErrorCode.INTERNAL_ERROR, "test message");
    expect(error.code).toBe("INTERNAL_ERROR");
    expect(error.message).toBe("test message");
  });

  it("should include details and fatal flag", () => {
    const error = new HeliosAppError(ErrorCode.INVALID_ARGUMENT, "bad arg", {
      details: { field: "name" },
      fatal: true,
    });
    expect(error.details?.field).toBe("name");
    expect(error.fatal).toBe(true);
  });

  it("should serialize to JSON correctly", () => {
    const error = new HeliosAppError(ErrorCode.NOT_FOUND, "not found", {
      details: { id: "123" },
    });
    const json = error.toJSON();
    expect(json.code).toBe("NOT_FOUND");
    expect(json.message).toBe("not found");
    expect(json.details?.id).toBe("123");
  });

  it("should default fatal to false", () => {
    const error = new HeliosAppError(ErrorCode.TIMEOUT, "timed out");
    expect(error.fatal).toBe(false);
  });

  it("should expose the same code order as the shared contract", () => {
    const codes = JSON.parse(readFileSync(join(contractRoot, "error-codes.json"), "utf8"));
    expect(ERROR_CODES).toEqual(codes);
  });

  it("should serialize to the shared validation fixture", () => {
    const error = new HeliosAppError(ErrorCode.VALIDATION_ERROR, "Invalid lane name", {
      details: { field: "lane.name" },
      retryable: false,
    });
    const fixture = JSON.parse(
      readFileSync(join(contractRoot, "fixtures/validation-error.json"), "utf8"),
    );
    expect(error.toJSON()).toEqual({
      ...fixture,
      fatal: false,
    });
  });

  it("should serialize to the shared not-found fixture", () => {
    const error = new HeliosAppError(ErrorCode.NOT_FOUND, "project 42 not found", {
      details: { resource: "project", id: "42" },
      retryable: false,
    });
    const fixture = JSON.parse(
      readFileSync(join(contractRoot, "fixtures/not-found.json"), "utf8"),
    );
    expect(error.toJSON()).toEqual({
      ...fixture,
      fatal: false,
    });
  });
});
