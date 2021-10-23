package abc144.d

import java.util.Scanner
import scala.annotation.tailrec

object SolveTests {
  import Solves._
  private def check(actual: Double, expected: Double): Unit =
    // 相対誤差10^(-6)以下の時に正解.
    if (math.abs(expected * math.pow(10, 6) - actual * math.pow(10, 6)) > 1) {
      val callerLine = (new Throwable).getStackTrace.toList(1).getLineNumber
      println(s"(actual, expected) = ($actual, $expected): on $callerLine")
    }

  def test(solve: Solve): Unit = {
    check(solve(2, 2, 4), 45.0000000000)
    check(solve(12, 21, 10), 89.7834636934)
    check(solve(3, 1, 8), 4.2363947991)
  }

}

object Solves {
  trait Solve {
    def apply(a: Double, b: Double, x: Double): Double
  }

  class Solve1 extends Solve {
    override def apply(a: Double, b: Double, x: Double): Double = {
      val halfMaxV = a * a * b / 2
      val tanTheta = if (x <= halfMaxV) {
        (a * b * b) / (2 * x)
      } else {
        2 * (a * a * b - x) / (a * a * a)
      }
      val theta = math.atan(tanTheta)
      theta * 180.0 / math.Pi
    }
  }
}

object Main extends App {
  import Solves._
  import SolveTests._

  def main: Unit = {
    val solve = new Solve1
    val sc = new Scanner(System.in)
    val a, b, x = sc.nextDouble

    println(f"${solve(a, b, x)}%.10f")
  }

  test(new Solve1())
  main
}
