import { describe, expect, it } from "vitest";
import { AuditInputError, evaluateAudit, type AuditInput } from "./audit";

const base: AuditInput = {
  ownerUid: "1000",
  ownerGid: "1000",
  remoteUid: "1000",
  remoteGid: "1000",
  mode: "0755",
  runtime: "docker",
  userns: "default",
  readOnly: false
};

describe("browser audit model", () => {
  it("passes a direct owner mapping", () => {
    expect(evaluateAudit(base).verdict).toBe("pass");
  });

  it("catches the default rootless Podman subuid mismatch", () => {
    const result = evaluateAudit({ ...base, runtime: "podman" });
    expect(result.verdict).toBe("fail");
    expect(result.mappedIdentity).toContain("100999:100999");
    expect(result.remedy).toContain("keep-id");
  });

  it("passes the same Podman identity with keep-id", () => {
    expect(evaluateAudit({ ...base, runtime: "podman", userns: "keep-id" }).verdict).toBe("pass");
  });

  it("fails an explicitly read-only mount", () => {
    expect(evaluateAudit({ ...base, readOnly: true }).summary).toContain("read-only");
  });

  it("rejects invalid octal modes", () => {
    expect(() => evaluateAudit({ ...base, mode: "0899" })).toThrow(AuditInputError);
  });

  it("rejects rootless mappings that overflow the Linux ID range", () => {
    expect(() => evaluateAudit({
      ...base,
      ownerUid: "4294967295",
      ownerGid: "4294967295",
      remoteUid: "4294967295",
      remoteGid: "4294967295",
      mode: "0777",
      runtime: "podman",
      userns: "default"
    })).toThrow(/Mapped UID is outside the Linux ID range/);
  });
});
