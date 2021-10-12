package abc155.a

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
    check(solve(5, 7, 5), "Yes")
    check(solve(4, 4, 4), "No")
    check(solve(4, 9, 6), "No")
  }

}

object Solves {
  trait Solve {
    def apply(a: Int, b: Int, c: Int): String
  }

  class Solve1 extends Solve {
    override def apply(a: Int, b: Int, c: Int): String =
      if ((a == b && a != c) || (b == c && b != a) || (c == a && c != b))
        "Yes"
      else "No"
  }

  class Solve2 extends Solve {
    override def apply(a: Int, b: Int, c: Int): String = if (Set(a, b, c).size == 2) "Yes" else "No"
  }

  class Solve3 extends Solve {
    override def apply(a: Int, b: Int, c: Int): String = if (Seq(a, b, c).distinct.size == 2) "Yes" else "No"
  }
}

object Main extends App {
  import Solves._
  import SolveTests._

  def main: Unit = {
    val solve = new Solve1
    val sc = new Scanner(System.in)
    val a, b, c = sc.nextInt
    println(solve(a, b, c))
  }

  test(new Solve1())
  test(new Solve2())
  test(new Solve3())
  main
}
