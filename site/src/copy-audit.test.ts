import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

const repo = resolve(import.meta.dirname, "../..");

describe("generated copy audit", () => {
  it("matches current landing and README copy with reproducible counts", () => {
    const check = spawnSync(process.execPath, ["scripts/generate-copy-audit.mjs", "--check"], {
      cwd: repo,
      encoding: "utf8"
    });
    expect(check.status, check.stderr).toBe(0);
  });

  it("counts the em-dash tokenizer regression as nine words", () => {
    const sentence = "Use numeric identities—the CLI refuses to guess named users.";
    const count = sentence.split(/\s+/u).filter((token) => /[\p{L}\p{N}]/u.test(token)).length;
    expect(count).toBe(9);
  });

  it("preserves reader-visible Markdown code, links, and angle placeholders", () => {
    const audit = readFileSync(resolve(repo, ".factory/copy-audit.md"), "utf8");
    expect(audit).toContain("| 7 | The CLI discovers .devcontainer/devcontainer.json, .devcontainer.json, or devcontainer.json. |");
    expect(audit).toContain("| 7 | Compose services.<name>.user, image, build, volumes, and read_only; |");
    expect(audit).toContain("| 7 | Every public promise is listed in .factory/claims.json. |");
  });
});
