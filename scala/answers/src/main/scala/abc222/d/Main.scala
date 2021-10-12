package abc222.d

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
    println(s"test: ${solve.getClass.getName}")
    check(solve(2, "1 1".toIntList, "2 3".toIntList), 5)
    check(solve(3, "2 2 2".toIntList, "2 2 2".toIntList), 1)
    check(solve(10, "1 2 3 4 5 6 7 8 9 10".toIntList, "1 4 9 16 25 36 49 64 81 100".toIntList), 978222082)
  }

}

object Solves {
  trait Solve {
    def apply(n: Int, an: Seq[Int], bn: Seq[Int]): Int
  }

  // TLE
  class Solve1 extends Solve {
    override def apply(n: Int, an: Seq[Int], bn: Seq[Int]): Int = {
      val mod = 998244353
      // an(0) ~ bn(0)の範囲を1で初期化. それ以外は0で初期化.
      val firstArray: Seq[Long] = (0 until 3000).map(y => if (an.head <= y && y <= bn.head) 1L else 0L)
      val lastArray = (1 until n).foldLeft(firstArray) { (prevArray, x) =>
        (0 until 3000).map { y =>
          // y座標: an(x-1) ~ min(bn(x-1), y) の範囲のaccArrayの合計のmod.
          // それ以外のy座標は0で初期化.
          if (an(x) <= y && y <= bn(x)) {
            (an(x - 1) to math.min(bn(x - 1), y)).foldLeft(0L)((acc, ck) => acc + prevArray(ck) % mod)
          } else 0
        }
      }
      (lastArray.sum % mod).toInt
    }
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
    val bn = Seq.fill(n)(sc.nextInt)
    println(solve(n, an, bn))
  }

  test(new Solve1())
  main
}
