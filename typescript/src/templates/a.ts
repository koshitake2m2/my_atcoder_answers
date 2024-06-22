import * as fs from "fs";

const main = (input: string) => {
  console.log(input);
};

const input = fs.readFileSync("/dev/stdin", "utf8");
main(input);
