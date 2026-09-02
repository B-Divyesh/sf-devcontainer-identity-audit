import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import config from "../public/staticwebapp.config.json";

describe("static deployment response policy", () => {
  it("ships the required security headers and preload-eligible HSTS", () => {
    expect(config.globalHeaders["Content-Security-Policy"]).toContain("default-src 'self'");
    expect(config.globalHeaders["Permissions-Policy"]).toBe("camera=(), microphone=(), geolocation=()");
    expect(config.globalHeaders["Strict-Transport-Security"]).toContain("max-age=31536000");
    expect(config.globalHeaders["Strict-Transport-Security"]).toContain("preload");
  });

  it("marks content-hashed assets and the hashed hero immutable for one year", () => {
    const immutable = "public, max-age=31536000, immutable";
    expect(config.routes).toContainEqual({ route: "/assets/*", headers: { "Cache-Control": immutable } });
    expect(config.routes).toContainEqual({ route: "/mount-ledger-6b7fee8c.webp", headers: { "Cache-Control": immutable } });
  });

  it("rewrites unknown routes to a designed 404 document", () => {
    expect("navigationFallback" in config).toBe(false);
    expect(config.responseOverrides).toEqual({ 404: { rewrite: "/404.html" } });
  });

  it("versions the offline cache and reloads the shell during updates", () => {
    const worker = readFileSync(resolve(import.meta.dirname, "../public/sw.js"), "utf8");
    expect(worker).toMatch(/const CACHE = "mia-site-v\d+";/);
    expect(worker).toContain('fetch("/", { cache: "reload" })');
    expect(worker).toContain("keys.filter((key) => key !== CACHE)");
  });
});
