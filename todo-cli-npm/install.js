const { copyFileSync } = require("fs");
const { join } = require("path");
const os = require("os");

const platform = os.platform();
const arch = os.arch();

let srcBinary;

if (platform === "darwin" && arch === "arm64") {
  srcBinary = "bin/mac-aarch64/todo";
}
// else if (platform === "linux" && arch === "x64") {
//   srcBinary = "bin/linux-x64/todo";
// }
else if (platform === "win32" && arch === "x64") {
  srcBinary = "bin/win-x64/todo.exe";
} else {
  throw new Error(`Unsupported platform: ${platform} ${arch}`);
}

// Copy to main bin location
const destBinary = join(
  __dirname,
  "bin",
  platform === "win32" ? "todo.exe" : "todo"
);
copyFileSync(srcBinary, destBinary);
console.log(`Installed todo CLI for ${platform} ${arch}`);
