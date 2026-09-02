import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const repositoryRoot = resolve(import.meta.dirname, "..");
const packageMetadata = JSON.parse(readFileSync(resolve(repositoryRoot, "package.json"), "utf8")) as {
  version: string;
};

export const BUILD_IDENTITY_MARKER = "<!-- __BUILD_IDENTITY__ -->";
export const PACKAGE_VERSION = packageMetadata.version;

function normalizeBuildId(value: string): string {
  const trimmed = value.trim();
  if (!/^[A-Za-z0-9][A-Za-z0-9._-]{0,79}$/.test(trimmed)) {
    throw new Error("Build ID must contain 1–80 letters, numbers, dots, underscores, or hyphens.");
  }
  return /^[0-9a-f]{40,64}$/i.test(trimmed) ? trimmed.slice(0, 12) : trimmed;
}

export function resolveBuildId(environment: NodeJS.ProcessEnv = process.env): string {
  for (const name of ["FACTORY_BUILD_ID", "BUILD_COMMIT", "GITHUB_SHA", "SOURCE_VERSION", "BUILD_SOURCEVERSION"]) {
    const value = environment[name];
    if (value?.trim()) return normalizeBuildId(value);
  }

  try {
    return normalizeBuildId(execFileSync("git", ["rev-parse", "--short=12", "HEAD"], {
      cwd: repositoryRoot,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"]
    }));
  } catch {
    throw new Error("Set FACTORY_BUILD_ID or build from a Git checkout so the release footer is traceable.");
  }
}

export const BUILD_ID = resolveBuildId();
export const BUILD_LABEL = `v${PACKAGE_VERSION} · ${BUILD_ID}`;

export function injectBuildIdentity(html: string): string {
  const markers = html.split(BUILD_IDENTITY_MARKER).length - 1;
  if (markers !== 1) {
    throw new Error(`Each HTML entry must contain one build identity marker; found ${markers}.`);
  }
  return html.replace(BUILD_IDENTITY_MARKER, BUILD_LABEL);
}
