package abc155.b

import java.util.Scanner

object SolveTests {
  import Solves._
  private def check[A](actual: A, expected: A): Unit =
    if (actual != expected) {
      val callerLine = (new Throwable).getStackTrace.toList(1).getLineNumber
      println(s"(actual, expected) = ($actual, $expected): on $callerLine")
    }
  implicit class StringOpt(value: String) {
    def toIntList: List[Int] = value.split(" ").map(_.toInt).toList
  }

  def test(solve: Solve): Unit = {
    check(solve(5, "6 7 9 10 31".toIntList), "APPROVED")
    check(solve(3, "28 27 24".toIntList), "DENIED")
  }

}

object Solves {
  trait Solve {
    def apply(n: Int, an: Seq[Int]): String
  }

  class Solve1 extends Solve {
    override def apply(n: Int, an: Seq[Int]): String = {
      val even = an.filter(a => a % 2 == 0)
      if (even.count(a => a % 3 == 0 || a % 5 == 0) == even.size)
        "APPROVED"
      else
        "DENIED"
    }
  }

  class Solve2 extends Solve {
    override def apply(n: Int, an: Seq[Int]): String =
      if (an.filter(_ % 2 == 0).forall(a => a % 3 == 0 || a % 5 == 0))
        "APPROVED"
      else
        "DENIED"
  }
}

object Main extends App {
  import Solves._
  import SolveTests._

  def main: Unit = {
    val solve = new Solve1
    val sc = new Scanner(System.in)
    val n = sc.nextInt
    val an = Seq.fill(n)(sc.nextInt)
    println(solve(n, an))
  }

  test(new Solve1())
  test(new Solve2())
  main
}
