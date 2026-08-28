import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

test("loads without console errors and exposes the page structure", async ({ page }) => {
  const errors: string[] = [];
  page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  await page.goto("/");
  await expect(page).toHaveTitle(/Mount Identity Audit/);
  await expect(page.locator("h1")).toHaveCount(1);
  await expect(page.locator("main")).toHaveCount(1);
  await expect(page.locator("img")).toHaveAttribute("alt", /host computer and container workspace/i);
  await expect(page.getByRole("button", { name: "Run preflight" })).toBeVisible();
  expect(errors).toEqual([]);
});

test("predicts a mismatch then a safe keep-id mapping", async ({ page }) => {
  await page.goto("/#demo");
  await page.getByRole("button", { name: "Run preflight" }).click();
  await expect(page.locator("#status-stamp")).toHaveText("fail");
  await expect(page.locator("#mapped-id")).toContainText("100999:100999");
  await page.getByRole("button", { name: "Load safe example" }).click();
  await expect(page.locator("#status-stamp")).toHaveText("pass");
  await expect(page.locator("#result-title")).toHaveText("Workspace is writable");
});

test("announces validation errors", async ({ page }) => {
  await page.goto("/#demo");
  await page.getByLabel("Directory mode").fill("0899");
  await page.getByRole("button", { name: "Run preflight" }).click();
  await expect(page.getByRole("alert")).toContainText("octal digits");
});

test("has no serious or critical accessibility violations", async ({ page }) => {
  await page.goto("/");
  const results = await new AxeBuilder({ page }).analyze();
  const serious = results.violations.filter((violation) => ["serious", "critical"].includes(violation.impact ?? ""));
  expect(serious).toEqual([]);
});

test("fits the viewport and keeps controls reachable", async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== "mobile", "mobile-only layout check");
  await page.goto("/");
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  expect(overflow).toBeLessThanOrEqual(1);
  await page.getByRole("link", { name: "Try the audit" }).click();
  await expect(page.getByRole("heading", { name: "Test an identity mapping" })).toBeInViewport();
  await page.getByRole("button", { name: "Run preflight" }).scrollIntoViewIfNeeded();
  await expect(page.getByRole("button", { name: "Run preflight" })).toBeVisible();
});

test("shows the offline fallback and legal pages", async ({ page, context }) => {
  await page.goto("/");
  await page.evaluate(() => navigator.serviceWorker.ready);
  await context.setOffline(true);
  await page.evaluate(() => window.dispatchEvent(new Event("offline")));
  await expect(page.locator("#offline-banner")).toBeVisible();
  await page.reload();
  await expect(page.getByRole("heading", { level: 1 })).toContainText("Know who owns the mount");
  await context.setOffline(false);
  await page.goto("/privacy/");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Privacy");
  await page.goto("/terms/");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Terms");
});
