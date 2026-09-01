import { describe, expect, it } from "vitest";
import { AuditInputError, evaluateAudit, type AuditInput } from "./audit";

const base: AuditInput = {
  ownerUid: "1000",
  ownerGid: "1000",
  hostUid: "1000",
  hostGid: "1000",
  subuidStart: "100000",
  subgidStart: "100000",
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

  it("maps a non-kept identity through the keep-id subordinate range", () => {
    const result = evaluateAudit({
      ...base,
      remoteUid: "2000",
      remoteGid: "2000",
      runtime: "podman",
      userns: "keep-id"
    });
    expect(result.verdict).toBe("fail");
    expect(result.mappedIdentity).toBe("102000:102000 · keep-id mapping");
  });

  it("does not confuse the workspace owner with the host caller", () => {
    const result = evaluateAudit({
      ...base,
      ownerUid: "1000",
      ownerGid: "1000",
      hostUid: "1500",
      hostGid: "1600",
      remoteUid: "2000",
      remoteGid: "2100",
      runtime: "podman",
      userns: "keep-id"
    });
    expect(result.verdict).toBe("fail");
    expect(result.mappedIdentity).toBe("102000:102100 · keep-id mapping");
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
    })).toThrow(/Owner UID must be below Linux's reserved 4294967295 value/);
  });

  it.each(["ownerUid", "ownerGid", "remoteUid", "remoteGid"] as const)(
    "rejects reserved Linux identity in %s for direct Docker",
    (field) => {
      expect(() => evaluateAudit({ ...base, [field]: "4294967295" })).toThrow(
        /reserved 4294967295/
      );
    }
  );

  it("rejects a rootless mapping that reaches the reserved Linux identity", () => {
    expect(() => evaluateAudit({
      ...base,
      remoteUid: "4294867296",
      runtime: "podman",
      userns: "default"
    })).toThrow(/Mapped UID must be below Linux's reserved 4294967295 value/);
  });
});
