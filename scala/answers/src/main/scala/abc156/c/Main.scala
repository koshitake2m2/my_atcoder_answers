package abc156.c

import java.util.Scanner

object Main extends App {
  import Solves._
  import SolveTests._

  def main: Unit = {
    val solve = new Solve1
    val sc = new Scanner(System.in)
    val n = sc.nextInt
    val xn = Seq.fill(n)(sc.nextInt)
    println(solve(n, xn))
  }

  test(new Solve1())
  main
}

object SolveTests {
  import Solves._

  def test(solve: Solve): Unit = {
    check(solve(2, "1 4".split(" ").map(_.toInt)), 5)
    check(solve(7, "14 14 2 13 56 2 37".split(" ").map(_.toInt)), 2354)
  }

  private def check[A](actual: A, expected: A): Boolean = {
    val assertion = actual == expected
    if (!assertion) {
      println(s"actual : expected = $actual : $expected.")
    }
    assertion
  }
}

object Solves {
  trait Solve {
    def apply(n: Int, xn: Seq[Int]): Int
  }

  class Solve1 extends Solve {
    override def apply(n: Int, xn: Seq[Int]): Int = {
      assert(1 <= n && n <= 100)
      xn.foreach(x => assert(1 <= x && x <= 100))

      val start = xn.min
      val end = xn.max
      (start to end).foldLeft(Int.MaxValue) { (min, zahyo) =>
        val sum = xn.map(x => math.pow((x - zahyo), 2).toInt).sum
        math.min(min, sum)
      }
    }
  }

}
