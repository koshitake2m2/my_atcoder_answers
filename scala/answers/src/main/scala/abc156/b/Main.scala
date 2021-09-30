package abc156.b

import java.util.Scanner
import scala.annotation.tailrec

object Main extends App {
  trait Solve {
    def apply(n: Int, k: Int): Int
  }

  class Solve1 extends Solve {
    override def apply(n: Int, k: Int): Int = {
      assert(1 <= n && n <= math.pow(10, 9))
      assert(2 <= k && k <= 10)
      var i = 0
      while (n >= math.pow(k, i))
        i += 1
      i
    }
  }

  class Solve2 extends Solve {
    override def apply(n: Int, k: Int): Int = {
      assert(1 <= n && n <= math.pow(10, 9))
      assert(2 <= k && k <= 10)

      @tailrec
      def run(n: Int, k: Int, i: Int): Int =
        if (n >= math.pow(k, i))
          run(n, k, i + 1)
        else i
      run(n, k, 0)
    }
  }

  def test(solve: Solve): Unit = {
    assert(solve(11, 2) == 4)
    assert(solve(8, 2) == 4)
    assert(solve(1010101, 10) == 7)
    assert(solve(314159265, 3) == 18)
  }
  test(new Solve1())
  test(new Solve2())

  val solve = new Solve2
  val sc = new Scanner(System.in)
  val n = sc.nextInt()
  val k = sc.nextInt()

  println(solve(n, k))
}
