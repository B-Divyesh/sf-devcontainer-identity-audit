import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

interface Claim {
  id: string;
  claim: string;
  where: string;
  test: string;
  sandbox: string;
}

describe("claim registry", () => {
  it("maps every registered claim to exactly one tagged sandbox test", () => {
    const registryPath = resolve(import.meta.dirname, "../../.factory/claims.json");
    const specPath = resolve(import.meta.dirname, "../e2e/claims.spec.ts");
    const claims = JSON.parse(readFileSync(registryPath, "utf8")) as Claim[];
    const spec = readFileSync(specPath, "utf8");
    const ids = claims.map((claim) => claim.id);

    expect(new Set(ids).size).toBe(ids.length);
    expect(claims.length).toBeGreaterThan(0);
    for (const claim of claims) {
      expect(claim.claim).not.toBe("");
      expect(claim.where).not.toBe("");
      expect(claim.sandbox).not.toBe("");
      expect(claim.test).toBe(`npm run test:claims -- --grep @claim:${claim.id}`);
      expect(spec.split(`@claim:${claim.id}`).length - 1).toBe(1);
    }
  });
});
