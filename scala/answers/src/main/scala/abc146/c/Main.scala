package abc146.c

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
    check(solve(10, 7, 100), 9)
    check(solve(2, 1, 100000000000L), 1000000000L)
    check(solve(1000000000, 1000000000, 100), 0)
    check(solve(1234, 56789, 314159265), 254309)
  }

}

object Solves {
  trait Solve {
    def apply(a: Int, b: Int, x: Long): Long
  }

  /** WA.
    *
    * アプローチ.
    * a * i + b * i.digits <= x を満たす最大の整数iを求める. (1 <= i <= 10**9)
    * すなわち, i <= (x - b * i.digits) / a を満たす最大のiを求めれば良い.
    * そこで, i.digitsを固定して計算する.
    * 整数Nは1~10**9の範囲なので, 1桁~10桁の範囲で計算. ただし, 10桁目は最大値が10**9であることに注意.
    *
    * その他観点. 整数Nの値段は単調増加である.
    */
  class Solve1 extends Solve {
    implicit class LongOpt(n: Long) {

      /** 桁数 */
      def digits: Int = n.toString.length
    }

    /** とある桁数での最大のi */
    private def maxIInThisDigits(a: Int, b: Int, x: Long, digits: Int): Option[Long] = {
      val maxNInThisDigits = (math.pow(10, digits) - 1).toLong
      val maxI: Long = (x - b * digits) / a
      if (maxI <= 0 || maxI.digits < digits) {
        None
      } else if (maxI.digits == digits) {
        Some(maxI)
      } else { // maxI.digits > digits
        Some(maxNInThisDigits)
      }
    }

    override def apply(a: Int, b: Int, x: Long): Long = {
      val maxN = math.pow(10, 9).toInt
      var maxBuyableN: Long = 0
      (1 to 10).foreach { digits =>
        maxIInThisDigits(a, b, x, digits) match {
          case Some(maxI) =>
            if (maxN < maxI)
              maxBuyableN = maxN
            else
              maxBuyableN = maxI
          case None => ()
        }
      }
      maxBuyableN
    }
  }

  /** 二分探索 */
  class Solve2 extends Solve {
    implicit class LongOpt(n: Long) {

      /** 桁数 */
      def digits: Int = n.toString.length
    }

    private def price(a: Long, b: Long, n: Long): Long = a * n + b * n.digits

    override def apply(a: Int, b: Int, x: Long): Long = {
      @tailrec
      def binarySearch(low: Long, high: Long): Long = {
        if (high - low <= 1) return low
        val mid = (low + high) / 2
        if (price(a, b, mid) <= x) {
          binarySearch(mid, high)
        } else {
          binarySearch(low, mid)
        }
      }
      // 買うことができない時に0を出力する, かつ, 所持金は1以上.
      // 仮に整数0の値段があるとすると0円であるため, 0は必ず買うことができるとみなせる.
      val (start, end) = (0, 1e9.toInt + 1)
      binarySearch(start, end)
    }
  }

}

object Main extends App {
  import Solves._
  import SolveTests._

  def main: Unit = {
    val solve = new Solve2
    val sc = new Scanner(System.in)
    val a, b = sc.nextInt
    val x = sc.nextLong
    println(solve(a, b, x))
  }

  test(new Solve1())
  test(new Solve2())
  main
}
