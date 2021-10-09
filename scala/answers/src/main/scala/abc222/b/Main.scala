package abc222.b

import java.util.Scanner

object SolveTests {
  import Solves._
  private def check[A](actual: A, expected: A): Unit =
    if (actual != expected) {
      val callerLine = (new Throwable).getStackTrace.toList(1).getLineNumber
      println(s"(actual, expected) = ($actual, $expected): on $callerLine")
    }
  def test(solve: Solve): Unit = {
    check(solve(4, 50, "80 60 40 0".split(" ").map(_.toInt)), 2)
    check(solve(3, 90, "89 89 89".split(" ").map(_.toInt)), 3)
    check(solve(2, 22, "6 37".split(" ").map(_.toInt)), 1)
  }

}

object Solves {
  trait Solve {
    def apply(n: Int, p: Int, an: Seq[Int]): Int
  }

  class Solve1 extends Solve {
    override def apply(n: Int, p: Int, an: Seq[Int]): Int =
      an.count(a => a < p)

  }
}

object Main extends App {
  import Solves._
  import SolveTests._

  def main: Unit = {
    val solve = new Solve1
    val sc = new Scanner(System.in)
    val n, p = sc.nextInt
    val an = Seq.fill(n)(sc.nextInt)
    println(solve(n, p, an))
  }

  test(new Solve1())
  main
}
