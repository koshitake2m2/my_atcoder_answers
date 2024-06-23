import * as fs from "fs";

const runMain = () => {
  const input = fs.readFileSync("/dev/stdin", "utf8");
  const output = solve(input);
  console.log(output);
};

const runTest = (testCases: { input: string; output: string }[]) => {
  testCases.forEach((testCase, i) => {
    const input = testCase.input;
    const expected = testCase.output;
    const actual = solve(input);
    if (actual !== expected) {
      console.log(`❌ Test ${i + 1}`);
      console.log(`Expected: ${expected}`);
      console.log(`Actual: ${actual}`);
    } else {
      console.log(`✅ Test ${i + 1}`);
    }
  });
};
const testCases = [
  {
    input: `1
2 3
test
`,
    output: `6 test`,
  },
];

const solve = (input: string): string => {
  const lines = input.trim().split("\n");
  const a = Number(lines[0]);
  const [b, c] = lines[1].split(" ").map(Number);
  const s = lines[2];
  return `${a + b + c} ${s}`;
};

// runMain();
runTest(testCases);
