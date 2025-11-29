import {chromium, firefox} from '@playwright/test';
import {spawn} from 'node:child_process';
import fs from 'node:fs/promises';
import os from 'node:os';

// Ensure Object.groupBy is available (Node 20+)
if (!Object.groupBy) {
  throw new Error("Object.groupBy is not available in this Node version. Please use Node 20 or later.");
}

const timeout = ms => new Promise(resolve => setTimeout(resolve, ms));

const runtime = process.argv[0];
const port = 6010;

async function runServer() {
  console.log('Starting web server as background process...');
  const server = spawn(runtime, ["server.mjs", port]);
  await timeout(3000); // Give it a moment to start up
  return server;
}

 function runBenchmark(driver, options = {}) {
  // biome-ignore lint/suspicious/noAsyncPromiseExecutor: <explanation>
  return new Promise(async (resolve, reject) => {
    const start = performance.now();

    console.log(`${driver.name()} launching...`);
    const browser = await driver.launch({
      headless: false,
      ...options
    });
    const page = await browser.newPage();

    // Use browser console as a way to get results back from the page
    const results = [];
    page.on('console', async msg => {
      if (msg.type() === "log") {
        const text = msg.text();
        if (text.startsWith('DONE.')) {
          // Finished
          const end = performance.now();
          await browser.close();
          console.log(`${driver.name()} done in ${(end - start / 1000).toFixed(2)}ms.`);
          resolve(results);
        } else {
          results.push(text);
        }
      } else {
        console.error(`${driver.name()} ${msg.type()}: ${msg.text()}`);
      }
    });
    page.on('pageerror', exception => {
      console.error(`${driver.name()} Page error: ${exception.stack}`);
    });

    await page.goto(`http://localhost:${port}`);
  });
}

async function writeJsonFile(filename, data) {
  await fs.writeFile(filename, JSON.stringify(data, null, 2));
  console.log(`Wrote results to ${filename}`);
}

const platform = os.platform();

const FIREFOX_PATH = platform === 'darwin' 
  ? "/Applications/Firefox Nightly.app/Contents/MacOS/firefox"
  : "/usr/bin/firefox-nightly";

const RUNS = 1;
(async () => {
  const server = await runServer();
  
  const benchmark_runs = {
    "chromium": [],
    "firefox": []
  };
  for (let i = 0; i < RUNS; i++) {
    console.log(`Running benchmark iteration ${i + 1}/${RUNS}...`);
    const chromiumres = await runBenchmark(chromium);
    const firefoxres = await runBenchmark(firefox, {
      channel: "moz-firefox",
      executablePath: FIREFOX_PATH
    });
    // Uncomment to run both in parallel
    // const [chromiumres, firefoxres] = await Promise.all([
    //   runBenchmark(chromium),
    //   runBenchmark(firefox, {
    //     channel: "moz-firefox",
    //     executablePath: FIREFOX_PATH
    //   })
    // ]);

    benchmark_runs.chromium.push(...(chromiumres).map(JSON.parse));
    benchmark_runs.firefox.push(...(firefoxres).map(JSON.parse));
  }

  await server.kill();

  // Parse results. First, separate by function name
  for (const browser in benchmark_runs) {
    const by_function = Object.groupBy(benchmark_runs[browser], r => r.fun);
    for (const fun in by_function) {
      // Now just keep the times
      console.log(`Processing ${browser} - ${fun}...`);
      console.log(by_function[fun]);
      const times = by_function[fun].flatMap(r => r.times).map(t => t * 1e6); // convert ms to ns

      by_function[fun] = times;
      await writeJsonFile(`benchmark_times_${browser}_${fun.replace(/\W+/g, '_')}.json`, times);
    }
  }
})();