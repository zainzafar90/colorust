const wasm = require("./pkg/rust_terminal_colors.js");

function checkColorSupport() {
  const noColor = process.env.NO_COLOR !== undefined;
  const forceColor = process.env.FORCE_COLOR !== undefined;
  const term = process.env.TERM;
  const isTTY = process.stdout.isTTY;

  if (noColor) return false;

  if (forceColor || isTTY || (term && term !== "dumb")) {
    return true;
  }

  return false;
}

const colors = new wasm.Colors(checkColorSupport());

console.log(colors.reset("Hello, World!"));
console.log(colors.bold("Hello, World!"));
console.log(colors.italic("Hello, World!"));
console.log(colors.underline("Hello, World!"));
console.log(colors.inverse("Hello, World!"));
console.log(colors.hidden("Hello, World!"));
console.log(colors.strikethrough("Hello, World!"));
console.log(colors.black("Hello, World!"));
console.log(colors.red("Hello, World!"));
console.log(colors.green("Hello, World!"));
console.log(colors.yellow("Hello, World!"));
console.log(colors.blue("Hello, World!"));
console.log(colors.magenta("Hello, World!"));
console.log(colors.cyan("Hello, World!"));
console.log(colors.white("Hello, World!"));
console.log(colors.gray("Hello, World!"));
console.log(colors.bg_black("Hello, World!"));
console.log(colors.bg_red("Hello, World!"));
console.log(colors.bg_green("Hello, World!"));
console.log(colors.bg_yellow("Hello, World!"));
console.log(colors.bg_blue("Hello, World!"));
console.log(colors.bg_magenta("Hello, World!"));
console.log(colors.bg_cyan("Hello, World!"));
console.log(colors.bg_white("Hello, World!"));
console.log(colors.bg_gray("Hello, World!"));
console.log(colors.bg_reset("Hello, World!"));
console.log(colors.reset("Hello, World!"));

console.log(colors.green(`How are ${colors.italic("you")} doing?`));

console.log(
  `I see a ${colors.red("red door")} and I want it painted ${colors.black(
    "black"
  )}`
);

console.log(
  colors.bg_black(
    `${colors.green(
      `Tom appeared on the sidewalk with a bucket of ${colors.italic(
        colors.red("red door")
      )} and a long-handled brush.`
    )}`
  )
);
