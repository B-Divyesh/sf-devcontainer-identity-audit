import { describe, expect, it } from "vitest";
import config from "../public/staticwebapp.config.json";

describe("static deployment response policy", () => {
  it("ships the required security headers and preload-eligible HSTS", () => {
    expect(config.globalHeaders["Content-Security-Policy"]).toContain("default-src 'self'");
    expect(config.globalHeaders["Permissions-Policy"]).toBe("camera=(), microphone=(), geolocation=()");
    expect(config.globalHeaders["Strict-Transport-Security"]).toContain("max-age=31536000");
    expect(config.globalHeaders["Strict-Transport-Security"]).toContain("preload");
  });

  it("marks only content-hashed assets immutable for one year", () => {
    const immutable = "public, max-age=31536000, immutable";
    expect(config.routes).toContainEqual({ route: "/assets/*", headers: { "Cache-Control": immutable } });
    expect(config.routes).not.toContainEqual({ route: "/mount-ledger.webp", headers: { "Cache-Control": immutable } });
  });

  it("rewrites unknown routes to a designed 404 document", () => {
    expect(config.responseOverrides).toEqual({ 404: { rewrite: "/404.html" } });
  });
});
