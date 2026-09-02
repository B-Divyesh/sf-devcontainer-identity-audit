import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

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
    expect(html).toContain("v0.1.0 · polish-3");
    expect(html).toContain('id="route-status" aria-live="polite"');
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
