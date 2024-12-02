import fs from "node:fs/promises";

const startTimeMs = performance.now();

const file = await fs.readFile("./real_input.txt");
const reports = file
  .toString()
  .split("\n")
  .map((line) => line.split(" ").map((n) => parseInt(n)));

let numSafeReports = 0;
for (const report of reports) {
  let reportDirection = 0;
  let isMainReportSafe = true;
  for (let i = 0; i < report.length; i++) {
    const item = report[i];
    if (i === 0) {
      continue;
    }
    const prev = report[i - 1];
    const differenceFromPrev = item - prev;
    const direction = Math.sign(differenceFromPrev);
    if (reportDirection === 0) {
      reportDirection = direction;
    }
    if (
      direction !== reportDirection ||
      Math.abs(differenceFromPrev) < 1 ||
      Math.abs(differenceFromPrev) > 3
    ) {
      const anyVariantsSafe = report
        .map((_, i) => report.filter((_, j) => j !== i))
        .some((variant) => {
          let reportDirection = 0;
          for (let i = 0; i < variant.length; i++) {
            const item = variant[i];
            if (i === 0) {
              continue;
            }
            const prev = variant[i - 1];
            const differenceFromPrev = item - prev;
            const direction = Math.sign(differenceFromPrev);
            if (reportDirection == 0) {
              reportDirection = direction;
            }
            if (
              direction !== reportDirection ||
              Math.abs(differenceFromPrev) < 1 ||
              Math.abs(differenceFromPrev) > 3
            ) {
              return false;
            }
          }
          return true;
        });
      if (anyVariantsSafe) {
        numSafeReports++;
      }
      isMainReportSafe = false;
      break;
    }
  }
  if (isMainReportSafe) {
    numSafeReports++;
    continue;
  }
}

console.log({ numSafeReports, ms: performance.now() - startTimeMs });
