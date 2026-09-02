import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import {
  BUILD_ID,
  BUILD_IDENTITY_MARKER,
  BUILD_LABEL,
  PACKAGE_VERSION,
  injectBuildIdentity,
  resolveBuildId
} from "../build-identity";

const site = resolve(import.meta.dirname, "..");
const pages = ["index.html", "demo/index.html", "privacy/index.html", "terms/index.html", "404.html"];

describe("public page structure", () => {
  it.each(pages)("ships complete metadata and the shared shell on %s", (name) => {
    const html = readFileSync(resolve(site, name), "utf8");
    expect(html).toMatch(/<html lang="en">/);
    expect(html.match(/<h1(?:\s|>)/g)).toHaveLength(1);
    expect(html).toContain("<main");
    expect(html).toContain("<header class=\"site-header\">");
    expect(html).toContain("<footer>");
    expect(html).toMatch(/<link rel="canonical"/);
    expect(html).toMatch(/<meta property="og:image"/);
    expect(html).toMatch(/<meta name="twitter:card"/);
    expect(html).toMatch(/<link rel="apple-touch-icon"/);
    expect(html.split(BUILD_IDENTITY_MARKER)).toHaveLength(2);
    expect(html).not.toContain("polish-3");
    const builtHtml = injectBuildIdentity(html);
    expect(builtHtml).toContain(`data-build-identity>${BUILD_LABEL}</span>`);
    expect(builtHtml).not.toContain(BUILD_IDENTITY_MARKER);
    expect(html).toContain('id="route-status" aria-live="polite"');
  });

  it("uses the current package version and a traceable build identifier", () => {
    const packageJson = JSON.parse(readFileSync(resolve(site, "../package.json"), "utf8")) as { version: string };
    expect(PACKAGE_VERSION).toBe(packageJson.version);
    expect(BUILD_LABEL).toBe(`v${packageJson.version} · ${BUILD_ID}`);
    expect(BUILD_ID).toMatch(/^[A-Za-z0-9][A-Za-z0-9._-]{0,79}$/);
    expect(BUILD_ID).not.toBe("polish-3");
  });

  it("prefers a factory build ID and normalizes full commit hashes", () => {
    expect(resolveBuildId({ FACTORY_BUILD_ID: "devcontainer-identity-audit-polish-5" })).toBe(
      "devcontainer-identity-audit-polish-5"
    );
    expect(resolveBuildId({ GITHUB_SHA: "1234567890abcdef1234567890abcdef12345678" })).toBe("1234567890ab");
    expect(() => resolveBuildId({ FACTORY_BUILD_ID: "unsafe build id" })).toThrow(/Build ID/);
  });

  it("ships the social, touch, recording, demo, and audit documents", () => {
    for (const path of [
      "public/social-card.webp",
      "public/apple-touch-icon.png",
      "public/demo-terminal.svg",
      "demo/index.html",
      "404.html"
    ]) {
      expect(existsSync(resolve(site, path))).toBe(true);
    }
    expect(existsSync(resolve(site, "../.factory/copy-audit.md"))).toBe(true);
    expect(existsSync(resolve(site, "../.factory/demo.md"))).toBe(true);
  });
});
