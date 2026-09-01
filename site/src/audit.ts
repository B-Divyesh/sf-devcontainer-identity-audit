export type Runtime = "docker" | "podman";
export type UserNamespace = "default" | "keep-id" | "host";

export interface AuditInput {
  ownerUid: string;
  ownerGid: string;
  hostUid: string;
  hostGid: string;
  subuidStart: string;
  subgidStart: string;
  remoteUid: string;
  remoteGid: string;
  mode: string;
  runtime: Runtime;
  userns: UserNamespace;
  readOnly: boolean;
}

export interface DemoAuditResult {
  verdict: "pass" | "fail";
  containerIdentity: string;
  mappedIdentity: string;
  workspaceIdentity: string;
  access: string;
  summary: string;
  remedy: string;
}

export class AuditInputError extends Error {}

const numericId = (value: string, label: string): number => {
  if (!/^\d+$/.test(value.trim())) throw new AuditInputError(`${label} must be a non-negative number.`);
  const parsed = Number(value);
  if (!Number.isSafeInteger(parsed) || parsed >= 4_294_967_295) throw new AuditInputError(`${label} must be below Linux's reserved 4294967295 value.`);
  return parsed;
};

const mappedId = (value: number, label: string): number => {
  if (!Number.isSafeInteger(value) || value >= 4_294_967_295) {
    throw new AuditInputError(`${label} must be below Linux's reserved 4294967295 value.`);
  }
  return value;
};

const octalMode = (value: string): number => {
  if (!/^[0-7]{3,4}$/.test(value.trim())) throw new AuditInputError("Directory mode must be three or four octal digits, such as 0755.");
  return Number.parseInt(value, 8) & 0o777;
};

export function evaluateAudit(input: AuditInput): DemoAuditResult {
  const ownerUid = numericId(input.ownerUid, "Owner UID");
  const ownerGid = numericId(input.ownerGid, "Owner GID");
  const remoteUid = numericId(input.remoteUid, "Remote UID");
  const remoteGid = numericId(input.remoteGid, "Remote GID");
  const mode = octalMode(input.mode);

  let mappedUid = remoteUid;
  let mappedGid = remoteGid;
  let mapping = "direct mapping";
  if (input.runtime === "podman" && input.userns !== "host") {
    const hostUid = numericId(input.hostUid, "Host caller UID");
    const hostGid = numericId(input.hostGid, "Host caller GID");
    const subuidStart = numericId(input.subuidStart, "Subordinate UID start");
    const subgidStart = numericId(input.subgidStart, "Subordinate GID start");

    if (input.userns === "default") {
      mappedUid = mappedId(remoteUid === 0 ? hostUid : subuidStart + remoteUid - 1, "Mapped UID");
      mappedGid = mappedId(remoteGid === 0 ? hostGid : subgidStart + remoteGid - 1, "Mapped GID");
      mapping = "rootless subuid map";
    } else {
      mappedUid = mappedId(remoteUid === hostUid ? hostUid : subuidStart + remoteUid, "Mapped UID");
      mappedGid = mappedId(remoteGid === hostGid ? hostGid : subgidStart + remoteGid, "Mapped GID");
      mapping = "keep-id mapping";
    }
  }

  const bits = mappedUid === 0
    ? 0o7
    : mappedUid === ownerUid
      ? (mode >> 6) & 0o7
      : mappedGid === ownerGid
        ? (mode >> 3) & 0o7
        : mode & 0o7;
  const readable = mappedUid === 0 || (bits & 0o5) === 0o5;
  const writableByMode = mappedUid === 0 || (bits & 0o3) === 0o3;
  const writable = writableByMode && !input.readOnly;
  const verdict = readable && writable ? "pass" : "fail";

  let summary = "The mapped identity can read, write, and traverse this workspace.";
  let remedy = "No ownership change is indicated. Confirm with the CLI against the real runtime map.";
  if (input.readOnly) {
    summary = "The mount is explicitly read-only, regardless of matching ownership.";
    remedy = "Remove the read-only mount flag only if workspace edits are intended.";
  } else if (!readable) {
    summary = "The mapped identity cannot read and traverse the workspace directory.";
    remedy = input.runtime === "podman" && input.userns === "default"
      ? "Use Podman keep-id so the developer identity maps back to the host owner."
      : "Choose a remote UID:GID that maps to the workspace owner or intended project group.";
  } else if (!writable) {
    summary = "The mapped identity can read this directory but cannot create or edit entries.";
    remedy = input.runtime === "podman" && input.userns === "default"
      ? "Use Podman keep-id so the developer identity maps back to the host owner."
      : "Match the workspace owner or deliberately grant group write access on the host.";
  }

  return {
    verdict,
    containerIdentity: `${remoteUid}:${remoteGid}`,
    mappedIdentity: `${mappedUid}:${mappedGid} · ${mapping}`,
    workspaceIdentity: `${ownerUid}:${ownerGid} · ${input.mode.padStart(4, "0")}`,
    access: `${readable ? "read" : "no read"} · ${writable ? "write" : "no write"} · traverse`,
    summary,
    remedy
  };
}
