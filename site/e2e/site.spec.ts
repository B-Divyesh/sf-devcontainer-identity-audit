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

test("rejects a rootless mapping that exceeds the Linux ID range", async ({ page }) => {
  await page.goto("/#demo");
  for (const label of ["Owner UID", "Owner GID", "Remote UID", "Remote GID"]) {
    await page.getByLabel(label).fill("4294967295");
  }
  await page.getByLabel("Directory mode").fill("0777");
  await page.getByRole("button", { name: "Run preflight" }).click();
  await expect(page.getByRole("alert")).toContainText("Mapped UID is outside the Linux ID range");
  await expect(page.locator("#status-stamp")).toHaveText("Ready");
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

test("reflows at 200% mobile text size without clipping", async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== "mobile", "mobile-only text resize check");
  await page.goto("/");
  await page.evaluate(() => { document.documentElement.style.fontSize = "32px"; });

  const clipped = await page.locator("h1, input, select, button, #result-title, #result-summary").evaluateAll((elements) =>
    elements
      .filter((element) => {
        const rect = element.getBoundingClientRect();
        return rect.width > 0 && rect.height > 0;
      })
      .filter((element) => {
        const rect = element.getBoundingClientRect();
        return rect.left < -1 || rect.right > document.documentElement.clientWidth + 1;
      })
      .map((element) => `${element.tagName.toLowerCase()}#${element.id || "(none)"}`)
  );
  expect(clipped).toEqual([]);
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  expect(overflow).toBeLessThanOrEqual(1);
  await page.getByRole("button", { name: "Run preflight" }).click();
  await expect(page.locator("#result-title")).toHaveText("Mount mismatch predicted");
  await page.getByRole("button", { name: "Load safe example" }).click();
  await expect(page.locator("#result-title")).toHaveText("Workspace is writable");
});

test("supports the primary flow with only the keyboard", async ({ page }) => {
  await page.goto("/");
  await page.keyboard.press("Tab");
  await expect(page.getByRole("link", { name: "Skip to main content" })).toBeFocused();
  await page.keyboard.press("Enter");

  for (let index = 0; index < 20; index += 1) {
    if (await page.getByRole("button", { name: "Run preflight" }).evaluate((element) => element === document.activeElement)) break;
    await page.keyboard.press("Tab");
  }
  await expect(page.getByRole("button", { name: "Run preflight" })).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(page.locator("#status-stamp")).toHaveText("fail");
  await page.keyboard.press("Tab");
  await expect(page.getByRole("button", { name: "Load safe example" })).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(page.locator("#status-stamp")).toHaveText("pass");
});

test("keeps the skip link hidden until focus with reduced motion", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto("/");
  const skip = page.getByRole("link", { name: "Skip to main content" });
  const beforeFocus = await skip.boundingBox();
  expect(beforeFocus).not.toBeNull();
  expect(beforeFocus!.y + beforeFocus!.height).toBeLessThanOrEqual(0);
  await page.keyboard.press("Tab");
  await expect(skip).toBeFocused();
  const afterFocus = await skip.boundingBox();
  expect(afterFocus).not.toBeNull();
  expect(afterFocus!.y).toBeGreaterThanOrEqual(0);
});

test("keeps demo data local and stores no user values", async ({ page, context }) => {
  const requests: string[] = [];
  page.on("request", (request) => requests.push(request.url()));
  await page.goto("/");
  await page.getByLabel("Owner UID").fill("1234");
  const beforeRun = requests.length;
  await page.getByRole("button", { name: "Run preflight" }).click();
  expect(requests).toHaveLength(beforeRun);
  expect(await context.cookies()).toEqual([]);
  const storage = await page.evaluate(async () => ({
    local: localStorage.length,
    session: sessionStorage.length,
    indexedDb: (await indexedDB.databases()).length
  }));
  expect(storage).toEqual({ local: 0, session: 0, indexedDb: 0 });
});

test("gives mobile links and buttons 44px hit areas", async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== "mobile", "mobile-only target-size check");
  await page.goto("/");
  const undersized = await page.locator("a, button").evaluateAll((elements) =>
    elements
      .filter((element) => {
        const rect = element.getBoundingClientRect();
        return rect.width > 0 && rect.height > 0;
      })
      .filter((element) => {
        const rect = element.getBoundingClientRect();
        return rect.width < 44 || rect.height < 44;
      })
      .map((element) => `${element.textContent?.trim()}: ${element.getBoundingClientRect().width}x${element.getBoundingClientRect().height}`)
  );
  expect(undersized).toEqual([]);
});

test("shows the offline fallback and legal pages", async ({ page, context }) => {
  await page.goto("/");
  await page.evaluate(async () => {
    const registration = await navigator.serviceWorker.ready;
    await registration.update();
  });
  await context.setOffline(true);
  await page.evaluate(() => window.dispatchEvent(new Event("offline")));
  await expect(page.locator("#offline-banner")).toBeVisible();
  await page.reload();
  await expect(page.getByRole("heading", { level: 1 })).toContainText("Know who owns the mount");
  expect(await page.evaluate(() => navigator.serviceWorker.controller !== null)).toBe(true);
  await context.setOffline(false);
  await page.goto("/privacy/");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Privacy");
  await page.goto("/terms/");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Terms");
});
