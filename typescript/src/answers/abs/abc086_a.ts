import * as fs from "fs";

const solve = (input: string): string => {
  const lines = input.split("\n");
  const [a, b] = lines[0].split(" ").map(Number);
  const isEven = (i: number) => i % 2 === 0;
  const result = isEven(a) || isEven(b) ? "Even" : "Odd";
  return result;
};
const testCases = [
  {
    input: `3 4`,
    output: `Even`,
  },
  {
    input: `1 21`,
    output: `Odd`,
  },
];

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

const runMain = () => {
  const input = fs.readFileSync("/dev/stdin", "utf8");
  const output = solve(input);
  console.log(output);
};

runMain();
// runTest(testCases);
