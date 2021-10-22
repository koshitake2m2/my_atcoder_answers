package sample.template.a

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
    def toStringListFromMultiline: List[String] = value.split("\n").toList
  }

  def test(solve: Solve): Unit = {
    check(solve(0, ""), "")
    check(solve(1, ""), "")
    check(solve(2, ""), "")
  }

}

object Solves {
  trait Solve {
    def apply(n: Int, s: String): String
  }

  class Solve1 extends Solve {
    override def apply(n: Int, s: String): String = ???
  }

}

object Main extends App {
  import Solves._
  import SolveTests._

  def main: Unit = {
    val solve = new Solve1
    val sc = new Scanner(System.in)
    val n = sc.nextInt
    val s = sc.next
    val an = Seq.fill(n)(sc.next)

    println(solve(n, s))
//    solve(n, an).foreach(println(_))
  }

  test(new Solve1())
  main
}
