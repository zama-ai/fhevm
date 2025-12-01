import { test, expect } from "./fixtures";

test.describe("Auth0 Authorization Code Flow", () => {
  test("completes login via authorization code flow", async ({ page }) => {
    await page.goto("/");

    await expect(page.getByRole("link", { name: /sign in/i })).toBeVisible();
    await page.getByRole("link", { name: /sign in/i }).click();

    await page.waitForURL("**/dashboard", { timeout: 15_000 });
    await expect(page).toHaveURL(/\/dashboard/);
    await expect(
      page.getByRole("main").getByText("Test User", { exact: true })
    ).toBeVisible();
    await expect(
      page.getByRole("main").getByText("test@example.com", { exact: true })
    ).toBeVisible();
    await expect(page.getByRole("button", { name: /logout/i })).toBeVisible();
  });

  test("sign up flow mirrors login", async ({ page }) => {
    await page.goto("/");

    await expect(
      page.getByRole("link", { name: /get started/i })
    ).toBeVisible();
    await page.getByRole("link", { name: /get started/i }).click();

    await page.waitForURL("**/dashboard", { timeout: 15_000 });
    await expect(
      page.getByRole("main").getByText("Test User", { exact: true })
    ).toBeVisible();
  });

  test("logout redirects back home", async ({ page }) => {
    await page.goto("/");
    await page.getByRole("link", { name: /sign in/i }).click();
    await page.waitForURL("**/dashboard", { timeout: 15_000 });

    await page.getByRole("button", { name: /logout/i }).click();
    await page.waitForURL("/", { timeout: 5_000 });
    await expect(page.getByRole("link", { name: /sign in/i })).toBeVisible();
  });
});
