package abc222.a

import java.util.Scanner

object SolveTests {
  import Solves._
  private def check[A](actual: A, expected: A): Unit =
    if (actual != expected) {
      val callerLine = (new Throwable).getStackTrace.toList(1).getLineNumber
      println(s"(actual, expected) = ($actual, $expected): on $callerLine")
    }
  def test(solve: Solve): Unit = {
    check(solve(321), "0321")
    check(solve(7777), "7777")
  }

}

object Solves {
  trait Solve {
    def apply(n: Int): String
  }

  class Solve1 extends Solve {
    override def apply(n: Int): String =
      f"$n%04d"
  }
}

object Main extends App {
  import Solves._
  import SolveTests._

  def main: Unit = {
    val solve = new Solve1
    val sc = new Scanner(System.in)
    val n = sc.nextInt
    println(solve(n))
  }

  test(new Solve1())
  main
}
