package abc146.b

import java.util.Scanner

object SolveTests {
  import Solves._
  private def check[A](actual: A, expected: A): Unit =
    if (actual != expected) {
      val callerLine = (new Throwable).getStackTrace.toList(1).getLineNumber
      println(s"(actual, expected) = ($actual, $expected): on $callerLine")
    }

  def test(solve: Solve): Unit = {
    check(solve(2, "ABCXYZ"), "CDEZAB")
    check(solve(0, "ABCXYZ"), "ABCXYZ")
    check(solve(13, "ABCDEFGHIJKLMNOPQRSTUVWXYZ"), "NOPQRSTUVWXYZABCDEFGHIJKLM")
  }

}

object Solves {
  trait Solve {
    def apply(n: Int, s: String): String
  }

  class Solve1 extends Solve {
    private def changeChar(c: Char, n: Int): Char =
      if (c + n <= 'Z') {
        (c + n).toChar
      } else {
        (c + n - 26).toChar
      }
    override def apply(n: Int, s: String): String =
      s.toCharArray.map(changeChar(_, n)).mkString
  }

  class Solve2 extends Solve {
    private def changeChar(c: Char, n: Int): Char =
      ((c + n - 'A') % 26 + 'A').toChar

    override def apply(n: Int, s: String): String =
      s.toCharArray.map(changeChar(_, n)).mkString
  }

}

object Main extends App {
  import Solves._
  import SolveTests._

  def main: Unit = {
    val solve = new Solve2
    val sc = new Scanner(System.in)
    val n = sc.nextInt
    val s = sc.next
    println(solve(n, s))
  }

  test(new Solve1())
  test(new Solve2())
  main
}
