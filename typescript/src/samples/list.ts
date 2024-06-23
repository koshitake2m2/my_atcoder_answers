const generateTwoDimensionalArrays: <T>(x: number, y: number, d: T) => T[][] = <
  T
>(
  x: number,
  y: number,
  d: T
) => new Array<Array<T>>(y).fill([]).map(() => new Array<T>(x).fill(d));
