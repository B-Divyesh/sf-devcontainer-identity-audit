import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

test("loads without console errors and exposes the page structure", async ({ page }) => {
  const errors: string[] = [];
  page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  await page.goto("/");
  await expect(page).toHaveTitle(/Mount Identity Audit/);
  await expect(page.locator("h1")).toHaveCount(1);
  await expect(page.locator("main")).toHaveCount(1);
  await expect(page.getByRole("img", { name: /host computer and container workspace/i })).toBeVisible();
  await expect(page.getByRole("button", { name: "Check mount permissions" })).toBeVisible();
  expect(errors).toEqual([]);
});

test("keeps all three required facts in the cold first viewport", async ({ page }) => {
  await page.goto("/");
  const facts = page.locator(".plain-facts li");
  await expect(facts).toHaveCount(3);
  const viewport = page.viewportSize();
  expect(viewport).not.toBeNull();
  for (let index = 0; index < 3; index += 1) {
    const box = await facts.nth(index).boundingBox();
    expect(box, `fact ${index + 1} must be rendered`).not.toBeNull();
    expect(box!.y).toBeGreaterThanOrEqual(0);
    expect(box!.y + box!.height).toBeLessThanOrEqual(viewport!.height);
  }
});

test("query demo is isolated, labelled, and moves focus to the sample heading", async ({ page }) => {
  await page.goto("/?demo=1#demo");
  await expect(page.getByText("Demo — sample data, nothing is saved")).toBeVisible();
  await expect(page.locator("#status-stamp")).toHaveText("fail");
  await expect(page.getByRole("heading", { name: "Check numeric workspace access" })).toBeFocused();
  await page.getByRole("button", { name: "Load safe example" }).click();
  await expect(page.locator("#status-stamp")).toHaveText("pass");
  await page.getByRole("button", { name: "Reset demo" }).click();
  await expect(page.locator("#status-stamp")).toHaveText("fail");
  await page.getByRole("link", { name: "Open blank browser check" }).click();
  await expect(page).toHaveURL(/\/#demo$/);
  await expect(page.getByRole("heading", { name: "Check numeric workspace access" })).toBeFocused();
});

test("forward, back, and fragment navigation focus the destination heading", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("link", { name: "Try it with sample data" }).click();
  await expect(page.getByRole("heading", { name: "Check numeric workspace access" })).toBeFocused();
  await page.goBack();
  await expect(page.getByRole("heading", { name: "Check mount permissions before container startup" })).toBeFocused();
  await page.goto("/#how");
  await expect(page.getByRole("heading", { name: "Check the configuration, map, and workspace" })).toBeFocused();
});

test("predicts a mismatch then a safe keep-id mapping", async ({ page }) => {
  await page.goto("/#demo");
  await page.getByRole("button", { name: "Check mount permissions" }).click();
  await expect(page.locator("#status-stamp")).toHaveText("fail");
  await expect(page.locator("#mapped-id")).toContainText("100999:100999");
  await page.getByRole("button", { name: "Load safe example" }).click();
  await expect(page.locator("#status-stamp")).toHaveText("pass");
  await expect(page.locator("#result-title")).toHaveText("Workspace is writable");
});

test("loads every safe-example value after a validation error", async ({ page }) => {
  await page.goto("/demo/");
  await page.getByLabel("Directory mode").fill("0899");
  await page.getByRole("button", { name: "Check mount permissions" }).click();
  await expect(page.getByRole("alert")).toContainText("octal digits");
  await page.getByLabel("Owner UID").fill("3000");
  await page.getByLabel("Owner GID").fill("3001");
  await page.getByLabel("Remote UID").fill("4000");
  await page.getByLabel("Remote GID").fill("4001");
  await page.getByLabel("Host caller UID").fill("5000");
  await page.getByLabel("Host caller GID").fill("5001");
  await page.getByLabel("Subordinate UID start").fill("200000");
  await page.getByLabel("Subordinate GID start").fill("300000");
  await page.getByLabel("Mount is declared read-only").check();
  await page.getByRole("button", { name: "Load safe example" }).click();

  for (const label of ["Owner UID", "Owner GID", "Remote UID", "Remote GID", "Host caller UID", "Host caller GID"]) {
    await expect(page.getByLabel(label)).toHaveValue("1000");
  }
  await expect(page.getByLabel("Subordinate UID start")).toHaveValue("100000");
  await expect(page.getByLabel("Subordinate GID start")).toHaveValue("100000");
  await expect(page.getByLabel("Directory mode")).toHaveValue("0755");
  await expect(page.getByLabel("Runtime")).toHaveValue("podman");
  await expect(page.getByLabel("Podman user namespace")).toHaveValue("keep-id");
  await expect(page.getByLabel("Mount is declared read-only")).not.toBeChecked();
  await expect(page.getByRole("alert")).toBeHidden();
  await expect(page.locator("#status-stamp")).toHaveText("pass");
});

test("rejects the reserved Linux identity in direct Docker mode", async ({ page }) => {
  await page.goto("/#demo");
  for (const label of ["Owner UID", "Owner GID", "Remote UID", "Remote GID"]) {
    await page.getByLabel(label).fill("4294967295");
  }
  await page.getByLabel("Directory mode").fill("0777");
  await page.locator("#runtime").selectOption("docker");
  await page.getByRole("button", { name: "Check mount permissions" }).click();
  await expect(page.getByRole("alert")).toContainText("reserved 4294967295");
  await expect(page.locator("#status-stamp")).toHaveText("Ready");
});

test("has no serious or critical accessibility violations on every page", async ({ page }) => {
  for (const path of ["/", "/demo/", "/privacy/", "/terms/", "/404.html"]) {
    await test.step(path, async () => {
      await page.goto(path);
      const results = await new AxeBuilder({ page }).analyze();
      const serious = results.violations.filter((violation) => ["serious", "critical"].includes(violation.impact ?? ""));
      expect(serious).toEqual([]);
    });
  }
});

test("fits the viewport and keeps controls reachable", async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== "mobile", "mobile-only layout check");
  await page.goto("/");
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  expect(overflow).toBeLessThanOrEqual(1);
  await page.getByRole("link", { name: "Try it with sample data" }).click();
  await expect(page.getByRole("heading", { name: "Check numeric workspace access" })).toBeInViewport();
  await page.getByRole("button", { name: "Check mount permissions" }).scrollIntoViewIfNeeded();
  await expect(page.getByRole("button", { name: "Check mount permissions" })).toBeVisible();
});

test("turns the adapter comparison into labelled rows at 390px", async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== "mobile", "mobile-only adapter layout check");
  await page.goto("/");

  const comparison = page.getByRole("region", { name: "Runtime behavior comparison" });
  await comparison.scrollIntoViewIfNeeded();
  const dimensions = await comparison.evaluate((element) => ({
    clientWidth: element.clientWidth,
    scrollWidth: element.scrollWidth
  }));
  expect(dimensions.scrollWidth).toBeLessThanOrEqual(dimensions.clientWidth + 1);

  await expect(comparison.locator('tbody tr').first().locator('td').nth(0)).toHaveAttribute("data-label", "Identity evidence");
  await expect(comparison.locator('tbody tr').first().locator('td').nth(1)).toHaveAttribute("data-label", "What stays untouched");
  const visibleLabels = await comparison.locator("tbody td").evaluateAll((cells) =>
    cells.map((cell) => getComputedStyle(cell, "::before").content.replaceAll('"', ""))
  );
  expect(visibleLabels).toEqual([
    "Identity evidence",
    "What stays untouched",
    "Identity evidence",
    "What stays untouched"
  ]);
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
  await page.getByRole("button", { name: "Check mount permissions" }).click();
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
    if (await page.getByRole("button", { name: "Check mount permissions" }).evaluate((element) => element === document.activeElement)) break;
    await page.keyboard.press("Tab");
  }
  await expect(page.getByRole("button", { name: "Check mount permissions" })).toBeFocused();
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
  await page.getByRole("button", { name: "Check mount permissions" }).click();
  expect(requests.join("\n")).not.toContain("1234");
  expect(requests.every((request) => new URL(request).origin === new URL(page.url()).origin)).toBe(true);
  expect(await context.cookies()).toEqual([]);
  const storage = await page.evaluate(async () => ({
    local: localStorage.length,
    session: sessionStorage.length,
    indexedDb: (await indexedDB.databases()).length
  }));
  expect(storage).toEqual({ local: 0, session: 0, indexedDb: 0 });
});

test("gives every public route 44px mobile link and button targets", async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== "mobile", "mobile-only target-size check");
  for (const path of ["/", "/demo/", "/privacy/", "/terms/", "/404.html"]) {
    await page.goto(path);
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
    expect(undersized, `${path} contains an undersized target`).toEqual([]);
  }
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
  await expect(page.getByRole("heading", { level: 1 })).toContainText("Check mount permissions");
  expect(await page.evaluate(() => navigator.serviceWorker.controller !== null)).toBe(true);
  await context.setOffline(false);
  await page.goto("/privacy/");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Privacy");
  await page.goto("/terms/");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Terms");
});

test("demo resets sample state and exits to the real calculator", async ({ page }) => {
  await page.goto("/demo/");
  await expect(page.locator("#status-stamp")).toHaveText("fail");
  await page.getByRole("button", { name: "Load safe example" }).click();
  await expect(page.locator("#status-stamp")).toHaveText("pass");
  await page.getByRole("button", { name: "Reset demo" }).click();
  await expect(page.locator("#status-stamp")).toHaveText("fail");
  await expect(page.getByLabel("Podman user namespace")).toHaveValue("default");
  await page.getByRole("link", { name: "Open blank browser check" }).click();
  await expect(page).toHaveURL(/\/#demo$/);
  await expect(page.getByRole("heading", { name: "Check numeric workspace access" })).toBeVisible();
});

test("every route exposes complete metadata and the standard shell", async ({ page }) => {
  for (const path of ["/", "/demo/", "/privacy/", "/terms/", "/404.html"]) {
    await page.goto(path);
    await expect(page.locator('link[rel="canonical"]')).toHaveCount(1);
    await expect(page.locator('meta[property="og:image"]')).toHaveCount(1);
    await expect(page.locator('meta[name="twitter:card"]')).toHaveCount(1);
    await expect(page.locator('link[rel="apple-touch-icon"]')).toHaveCount(1);
    await expect(page.locator("header nav")).toBeVisible();
    await expect(page.locator("footer")).toContainText("v0.1.0 · polish-1");
  }
});
