package abc156.b

import java.util.Scanner

object Main extends App {

  def solve(n: Int, k: Int): Int = {
    assert(1 <= n && n <= math.pow(10, 9))
    assert(2 <= k && k <= 10)
    var i = 0
    while (n >= math.pow(k, i))
      i += 1
    i
  }

  def test: Unit = {
    assert(solve(11, 2) == 4)
    assert(solve(8, 2) == 4)
    assert(solve(1010101, 10) == 7)
    assert(solve(314159265, 3) == 18)
  }
  test

  val sc = new Scanner(System.in)
  val n = sc.nextInt()
  val k = sc.nextInt()

  println(solve(n, k))
}
