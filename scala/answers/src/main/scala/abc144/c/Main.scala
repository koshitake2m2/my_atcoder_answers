package abc144.c

import java.util.Scanner
import scala.annotation.tailrec

object SolveTests {
  import Solves._
  private def check[A](actual: A, expected: A): Unit =
    if (actual != expected) {
      val callerLine = (new Throwable).getStackTrace.toList(1).getLineNumber
      println(s"(actual, expected) = ($actual, $expected): on $callerLine")
    }

  def test(solve: Solve): Unit = {
    check(solve(10), 5)
    check(solve(50), 13)
    check(solve(10000000019L), 10000000018L)
  }

}

object Solves {
  trait Solve {
    def apply(n: Long): Long
  }

  class Solve1 extends Solve {
    override def apply(n: Long): Long = {
      @tailrec
      def search(i: Long): Long =
        if (n % i == 0) {
          i
        } else {
          search(i - 1)
        }

      val sq = math.sqrt(n).toLong
      val i = search(sq)
      val j = n / i
      val distance = (i - 1) + (j - 1)
      distance
    }
  }
}

object Main extends App {
  import Solves._
  import SolveTests._

  def main: Unit = {
    val solve = new Solve1
    val sc = new Scanner(System.in)
    val n = sc.nextLong

    println(solve(n))
  }

  test(new Solve1())
  main
}
