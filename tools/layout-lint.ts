/** Android viewport smoke test. Run after the WebAssembly module has been built. */
import { mkdirSync } from "node:fs";
import { chromium } from "playwright-core";
import { createServer } from "vite";

const sizes = [
  { width: 360, height: 800 },
  { width: 375, height: 812 },
  { width: 390, height: 844 },
  { width: 412, height: 915 },
  { width: 432, height: 960 },
  { width: 480, height: 960 },
  { width: 768, height: 1024 },
];
const modes = ["Play", "Speak", "Sing", "Bestiary", "Beatbox"];
const out = "out/layout";
mkdirSync(out, { recursive: true });
const server = await createServer({ base: "/", server: { host: "127.0.0.1", port: 0, strictPort: false }, logLevel: "error" });
await server.listen();
const url = server.resolvedUrls?.local[0];
if (!url) throw new Error("Vite did not provide a local URL");
const browser = await chromium.launch({
  headless: true,
  args: ["--no-sandbox", "--autoplay-policy=no-user-gesture-required", "--use-angle=swiftshader", "--enable-unsafe-swiftshader"],
});
let failures = 0;
try {
  for (const size of sizes) {
    const page = await browser.newPage({ viewport: size, deviceScaleFactor: 1, isMobile: true, hasTouch: true });
    page.on("pageerror", (error) => { console.error(`${size.width}x${size.height}: ${error.message}`); failures++; });
    await page.goto(url, { waitUntil: "networkidle" });
    await page.waitForSelector(".topbar", { timeout: 15000 });
    for (const mode of modes) {
      await page.getByRole("radio", { name: new RegExp(`^${mode}$`, "i") }).click();
      await page.waitForTimeout(220);
      const result = await page.evaluate(() => {
        const root = document.documentElement;
        const body = document.body;
        const screens = [...document.querySelectorAll<HTMLElement>(".screen")]
          .filter((el) => el.getClientRects().length)
          .map((el) => ({ name: el.getAttribute("aria-label") || el.className, left: el.getBoundingClientRect().left, right: el.getBoundingClientRect().right }));
        return {
          horizontalOverflow: Math.max(root.scrollWidth - root.clientWidth, body.scrollWidth - root.clientWidth),
          screensOutside: screens.filter((screen) => screen.left < -2 || screen.right > innerWidth + 2),
          viewport: innerWidth,
        };
      });
      if (result.horizontalOverflow > 2 || result.screensOutside.length) {
        failures++;
        console.error(`${size.width}x${size.height} ${mode}: ${JSON.stringify(result)}`);
        await page.screenshot({ path: `${out}/${size.width}x${size.height}-${mode.toLowerCase()}.png`, fullPage: true });
      }
    }
    await page.close();
  }
} finally {
  await browser.close();
  await server.close();
}
if (failures) throw new Error(`Android layout check found ${failures} issue(s)`);
console.log(`Android layout check passed: ${sizes.length} screen sizes × ${modes.length} modes`);
