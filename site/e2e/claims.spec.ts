import { expect, test } from "@playwright/test";
import { spawnSync } from "node:child_process";
import {
  chmodSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
  writeFileSync
} from "node:fs";
import { tmpdir } from "node:os";
import { join, relative, resolve } from "node:path";

const repo = resolve(import.meta.dirname, "../..");
const releaseCli = join(repo, "target/release/mount-identity-audit");
const debugCli = join(repo, "target/debug/mount-identity-audit");

function cliPath(): string {
  const path = existsSync(releaseCli) ? releaseCli : debugCli;
  expect(existsSync(path), "build the CLI before running claim tests").toBe(true);
  return path;
}

function runCli(args: string[], env: NodeJS.ProcessEnv = {}) {
  return spawnSync(cliPath(), args, {
    cwd: repo,
    encoding: "utf8",
    env: { ...process.env, ...env }
  });
}

function packedCliPath(): { path: string; sandbox: string } {
  const packageRun = spawnSync("cargo", ["package", "--allow-dirty", "--no-verify"], {
    cwd: repo,
    encoding: "utf8"
  });
  expect(packageRun.status, packageRun.stderr).toBe(0);

  const crate = join(repo, "target/package/mount-identity-audit-0.1.0.crate");
  expect(existsSync(crate), "cargo package must produce the consumer artifact").toBe(true);
  const sandbox = mkdtempSync(join(tmpdir(), "mia-packed-claim-"));
  const sourceRoot = join(sandbox, "source");
  const installRoot = join(sandbox, "install");
  mkdirSync(sourceRoot, { recursive: true });
  const unpack = spawnSync("tar", ["-xzf", crate, "-C", sourceRoot], { encoding: "utf8" });
  expect(unpack.status, unpack.stderr).toBe(0);
  const install = spawnSync(
    "cargo",
    [
      "install",
      "--path", join(sourceRoot, "mount-identity-audit-0.1.0"),
      "--root", installRoot,
      "--target-dir", join(repo, "target"),
      "--locked",
      "--offline"
    ],
    { cwd: repo, encoding: "utf8" }
  );
  expect(install.status, install.stderr).toBe(0);
  const path = join(installRoot, "bin/mount-identity-audit");
  expect(existsSync(path), "the packed CLI must install for a clean consumer").toBe(true);
  return { path, sandbox };
}

function project(config: string): string {
  const root = mkdtempSync(join(tmpdir(), "mia-claim-"));
  mkdirSync(join(root, ".devcontainer"), { recursive: true });
  writeFileSync(join(root, ".devcontainer/devcontainer.json"), config);
  chmodSync(root, 0o755);
  return root;
}

function snapshot(root: string): string[] {
  const rows: string[] = [];
  const visit = (path: string): void => {
    const stat = statSync(path);
    rows.push(stat.isDirectory()
      ? `${relative(root, path)}:${stat.mode & 0o7777}:directory`
      : `${relative(root, path)}:${stat.mode & 0o7777}:${stat.size}:${stat.mtimeMs}`);
    if (stat.isDirectory()) {
      for (const name of readdirSync(path).sort()) visit(join(path, name));
    } else {
      rows.push(readFileSync(path, "utf8"));
    }
  };
  visit(root);
  return rows;
}

test("CLI demo uses an isolated bundled sample @claim:cli-demo", () => {
  const shippedPath = join(repo, "examples/mismatch/.devcontainer/devcontainer.json");
  const before = readFileSync(shippedPath, "utf8");
  const output = runCli(["--demo"]);
  expect(output.status).toBe(1);
  expect(output.stdout).toContain("DEMO — bundled sample data");
  expect(output.stdout).toContain("FAIL");
  const sample = output.stdout.match(/^Sample copy: (.+)$/m)?.[1];
  expect(sample).toBeTruthy();
  expect(sample!.startsWith(tmpdir())).toBe(true);
  expect(sample!.startsWith(repo)).toBe(false);
  expect(readFileSync(shippedPath, "utf8")).toBe(before);
  rmSync(sample!, { recursive: true, force: true });
});

test("home opens the working browser sample in one click @claim:browser-demo", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("link", { name: "Try it with sample data" }).click();
  await expect(page).toHaveURL(/\/demo\/$/);
  await expect(page.getByText("Demo — sample data, nothing is saved")).toBeVisible();
  await expect(page.locator("#status-stamp")).toHaveText("fail");
  await expect(page.locator("#mapped-id")).toContainText("100999:100999");
  await expect(page.getByRole("button", { name: "Reset demo" })).toBeVisible();
  await expect(page.getByRole("link", { name: "Start for real" })).toBeVisible();
});

test("CLI verdicts keep stable process exit codes @claim:permission-verdicts", () => {
  const pass = project('{ remoteUser: "0:0" }');
  const fail = project('{ remoteUser: "424242:424242" }');
  const unknown = project('{ remoteUser: "4294967295:4294967295" }');
  try {
    const cases = [[pass, 0, "pass"], [fail, 1, "fail"], [unknown, 2, "unknown"]] as const;
    for (const [root, exit, verdict] of cases) {
      const output = runCli([root, "--runtime", "docker", "--no-runtime", "--json"]);
      expect(output.status).toBe(exit);
      expect(JSON.parse(output.stdout).verdict).toBe(verdict);
    }
  } finally {
    for (const root of [pass, fail, unknown]) rmSync(root, { recursive: true, force: true });
  }
});

test("audit is read-only and caps runtime inspection @claim:read-only-safety", () => {
  const root = project('{ remoteUser: "1000:1000" }');
  const logRoot = mkdtempSync(join(tmpdir(), "mia-call-log-"));
  const log = join(logRoot, "runtime-calls.log");
  const runtime = join(root, "podman-recorder");
  writeFileSync(runtime, `#!/bin/sh
printf '%s\\n' "$*" >> "$AUDIT_CALL_LOG"
case "$1" in
  info) printf '%s\\n' '{"version":{"Version":"5.2.2"},"host":{"security":{"rootless":true}}}' ;;
  unshare) printf '%s\\n' '0 1000 1' '1 100000 65536' ;;
  *) exit 9 ;;
esac
`);
  chmodSync(runtime, 0o755);
  const before = snapshot(root);
  const output = runCli([root, "--runtime", "podman", "--runtime-bin", runtime, "--json"], { AUDIT_CALL_LOG: log });
  const calls = readFileSync(log, "utf8").trim().split("\n");
  rmSync(log);
  const after = snapshot(root);
  expect([0, 1]).toContain(output.status);
  expect(calls).toEqual([
    "info --format json",
    "unshare cat /proc/self/uid_map",
    "unshare cat /proc/self/gid_map"
  ]);
  expect(calls.length).toBeLessThanOrEqual(3);
  expect(calls.join(" ")).not.toMatch(/\b(run|start|create|pull)\b/);
  expect(after).toEqual(before);
  rmSync(root, { recursive: true, force: true });
  rmSync(logRoot, { recursive: true, force: true });
});

test("CLI reads JSONC and selected Compose metadata @claim:config-support", () => {
  const root = project(`{
    // Compose remains the source for the selected service and bind.
    dockerComposeFile: "compose.yml",
    service: "app",
    workspaceFolder: "/work"
  }`);
  writeFileSync(join(root, ".devcontainer/compose.yml"), "services:\n  app:\n    user: '0:0'\n    volumes:\n      - ../:/work\n");
  const output = runCli([root, "--runtime", "docker", "--no-runtime", "--json"]);
  const report = JSON.parse(output.stdout);
  expect(output.status).toBe(0);
  expect(report.verdict).toBe("pass");
  expect(report.identity.source).toContain("Compose service app user");
  expect(report.workspace.path).toBe(root);
  const parserTests = spawnSync("cargo", ["test", "config::tests"], { cwd: repo, encoding: "utf8" });
  expect(parserTests.status, parserTests.stderr).toBe(0);
  rmSync(root, { recursive: true, force: true });
});

test("Dev Container and Compose precedence holds in the packed CLI @claim:compose-user-precedence", () => {
  test.setTimeout(120_000);
  const packed = packedCliPath();
  const cases = [
    { property: "containerUser", devcontainerUser: "424242:424242", composeUser: "0:0", exit: 0, verdict: "pass", uid: 0, source: "Compose service app user" },
    { property: "containerUser", devcontainerUser: "0:0", composeUser: "424242:424242", exit: 1, verdict: "fail", uid: 424242, source: "Compose service app user" },
    { property: "remoteUser", devcontainerUser: "424242:424242", composeUser: "0:0", exit: 1, verdict: "fail", uid: 424242, source: "devcontainer remoteUser" },
    { property: "remoteUser", devcontainerUser: "0:0", composeUser: "424242:424242", exit: 0, verdict: "pass", uid: 0, source: "devcontainer remoteUser" }
  ] as const;
  const roots: string[] = [];
  try {
    for (const item of cases) {
      const root = project(`{
        dockerComposeFile: "compose.yml",
        service: "app",
        workspaceFolder: "/work",
        ${item.property}: "${item.devcontainerUser}"
      }`);
      roots.push(root);
      writeFileSync(
        join(root, ".devcontainer/compose.yml"),
        `services:\n  app:\n    user: "${item.composeUser}"\n    volumes:\n      - ../:/work\n`
      );
      const output = spawnSync(
        packed.path,
        [root, "--runtime", "docker", "--no-runtime", "--json"],
        { encoding: "utf8" }
      );
      expect(output.status).toBe(item.exit);
      const report = JSON.parse(output.stdout);
      expect(report.verdict).toBe(item.verdict);
      expect(report.identity.container_uid).toBe(item.uid);
      expect(report.identity.container_gid).toBe(item.uid);
      expect(report.identity.source).toBe(item.source);
    }
  } finally {
    for (const root of roots) rmSync(root, { recursive: true, force: true });
    rmSync(packed.sandbox, { recursive: true, force: true });
  }
});

test("share mode removes every supplied local path @claim:share-redaction", () => {
  const root = project('{ remoteUser: "0:0" }');
  const wrapper = join(root, "private/acme/runtime-wrapper");
  const output = runCli([root, "--runtime", "docker", "--runtime-bin", wrapper, "--share", "--json"]);
  expect(output.status).toBe(2);
  expect(output.stdout).not.toContain(root);
  expect(output.stdout).not.toContain(wrapper);
  expect(output.stdout).toContain("<runtime-bin>");
  expect(output.stdout).toContain("<devcontainer-config>");
  rmSync(root, { recursive: true, force: true });
});

test("JSON reports retain their versioned share-safe contract @claim:report-contract", () => {
  const root = project('{ remoteUser: "0:0" }');
  const output = runCli([root, "--runtime", "docker", "--no-runtime", "--json", "--share"]);
  const report = JSON.parse(output.stdout);
  expect(output.status).toBe(0);
  expect(report.schema_version).toBe(1);
  expect(report.config_path).toBe("<devcontainer-config>");
  expect(report.workspace.path).toBe("<workspace>");
  expect(output.stdout).not.toContain(root);
  rmSync(root, { recursive: true, force: true });
});

test("Docker and rootless Podman use distinct identity maps @claim:runtime-mapping", () => {
  const root = project('{ remoteUser: "1000:1000" }');
  const runtime = join(root, "podman-map");
  writeFileSync(runtime, `#!/bin/sh
case "$1" in
  info) printf '%s\\n' '{"version":{"Version":"5.2.2"},"host":{"security":{"rootless":true}}}' ;;
  unshare) printf '%s\\n' '0 1000 1' '1 100000 65536' ;;
  *) exit 9 ;;
esac
`);
  chmodSync(runtime, 0o755);
  const docker = runCli([root, "--runtime", "docker", "--no-runtime", "--json"]);
  const podman = runCli([root, "--runtime", "podman", "--runtime-bin", runtime, "--json"]);
  expect(JSON.parse(docker.stdout).identity.host_uid).toBe(1000);
  expect(JSON.parse(podman.stdout).identity.host_uid).toBe(100999);
  expect(JSON.parse(podman.stdout).runtime.rootless).toBe(true);
  rmSync(root, { recursive: true, force: true });
});

test("unproven identities never receive a safe verdict @claim:conservative-identities", async ({ page }) => {
  const configs = [
    '{ remoteUser: "vscode" }',
    '{ remoteUser: "1000" }',
    '{ build: { dockerfile: "Dockerfile" } }',
    '{ remoteUser: "4294967295:4294967295" }'
  ];
  for (const config of configs) {
    const root = project(config);
    const output = runCli([root, "--runtime", "docker", "--no-runtime", "--json"]);
    expect(output.status).toBe(2);
    expect(JSON.parse(output.stdout).verdict).toBe("unknown");
    rmSync(root, { recursive: true, force: true });
  }
  await page.goto("/demo/");
  await page.getByLabel("Runtime").selectOption("docker");
  await page.getByLabel("Remote UID").fill("4294967295");
  await page.getByRole("button", { name: "Run preflight" }).click();
  await expect(page.getByRole("alert")).toContainText("reserved 4294967295");
});

test("browser sample sends and stores no entered project data @claim:browser-private", async ({ page, context }) => {
  const requests: string[] = [];
  page.on("request", (request) => requests.push(`${request.url()} ${request.postData() ?? ""}`));
  await page.goto("/demo/");
  await page.getByLabel("Owner UID").fill("3141592");
  const beforeRun = requests.length;
  await page.getByRole("button", { name: "Run preflight" }).click();
  expect(requests).toHaveLength(beforeRun);
  expect(requests.join("\n")).not.toContain("3141592");
  expect(requests.every((request) => new URL(request.split(" ")[0]).origin === new URL(page.url()).origin)).toBe(true);
  expect(await context.cookies()).toEqual([]);
  expect(await page.evaluate(async () => ({
    local: localStorage.length,
    session: sessionStorage.length,
    indexedDb: (await indexedDB.databases()).length
  }))).toEqual({ local: 0, session: 0, indexedDb: 0 });
});

test("CLI source has no account, network, or telemetry client @claim:cli-private", () => {
  const manifest = readFileSync(join(repo, "Cargo.toml"), "utf8");
  const sources = ["src/main.rs", "src/lib.rs", "src/config.rs", "src/runtime.rs"]
    .map((path) => readFileSync(join(repo, path), "utf8"))
    .join("\n");
  expect(manifest).not.toMatch(/reqwest|hyper|ureq|telemetry|analytics/i);
  expect(sources).not.toMatch(/std::net|TcpStream|UdpSocket|telemetry|analytics/i);
  const demo = runCli(["--demo", "--quiet"]);
  expect(demo.stdout).toContain("DEMO — bundled sample data");
  expect(demo.stderr).toBe("");
});

test("browser sample reloads offline after first visit @claim:offline-reload", async ({ browser }) => {
  const context = await browser.newContext();
  const page = await context.newPage();
  try {
    await page.goto("/demo/");
    await expect(page.locator("#status-stamp")).toHaveText("fail");
    await page.evaluate(async () => { await navigator.serviceWorker.ready; });
    await page.waitForFunction(() => navigator.serviceWorker.controller !== null);
    await context.setOffline(true);
    await page.reload();
    await expect(page.getByRole("heading", { level: 1 })).toHaveText("Inspect the sample mount mismatch");
    await expect(page.locator("#status-stamp")).toHaveText("fail");
    expect(await page.evaluate(() => navigator.serviceWorker.controller !== null)).toBe(true);
  } finally {
    await context.close();
  }
});

test("browser model covers mismatch and keep-id recovery @claim:browser-parity", async ({ page }) => {
  await page.goto("/demo/");
  await expect(page.locator("#mapped-id")).toContainText("100999:100999");
  await expect(page.locator("#status-stamp")).toHaveText("fail");
  await page.getByRole("button", { name: "Load safe example" }).click();
  await expect(page.locator("#mapped-id")).toContainText("1000:1000 · keep-id mapping");
  await expect(page.locator("#status-stamp")).toHaveText("pass");
});

test("MIT terms are shipped with the free product @claim:mit-license", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByText("Free under the MIT License.")).toBeVisible();
  const cargo = readFileSync(join(repo, "Cargo.toml"), "utf8");
  const license = readFileSync(join(repo, "LICENSE"), "utf8");
  expect(cargo).toContain('license = "MIT"');
  expect(license).toContain("Permission is hereby granted, free of charge");
});
