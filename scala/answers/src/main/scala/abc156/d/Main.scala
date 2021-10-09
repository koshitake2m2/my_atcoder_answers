package abc156.d

import java.util.Scanner

object Main extends App {
  import Solves._

  private def check[A](actual: A, expected: A): Unit =
    if (actual != expected) { println(s"(actual, expected) = ($actual, $expected)") }
    else { println("success") }

  def test(solve: Solve): Unit = {
    check(solve(4, 1, 3), 7)
    check(solve(1000000000, 141421, 173205), 34076506)
    println("done")
  }

  def main(): Unit = {
    val solve = new Solve1
    val sc = new Scanner(System.in)
    val n, a, b = sc.nextInt
    println(solve(n, a, b))
  }

  test(new Solve1())
  main()
}

object Solves {
  trait Solve {
    def apply(n: Int, a: Int, b: Int): Long
  }

  class Solve1 extends Solve {
    private def repeatedSquaresWithMod(x: Int, y: Int, mod: Int): Long = {
      assert(0 <= x && 0 <= y)
      val binaryList = y.toBinaryString.reverse.toList.map(_.toString.toInt)
      val (acc, _) = binaryList.foldLeft((BigInt(1), x: BigInt)) { case ((acc, kake), bit) =>
        val newAcc = {
          if (bit == 1) (acc * kake) % mod
          else acc
        }
        (newAcc, (kake * kake) % mod)
      }
      (if (acc < 0) acc + mod else acc).toLong
    }

    private def nCkWithMod(n: Int, k: Int, mod: Int): Long = {
      val bunshi = (n - k + 1 to n).foldLeft(BigInt(1))((acc, kk) => (acc * kk) % mod)
      val res = (2 to k).foldLeft(bunshi)((acc, kk) => (BigInt(kk).modInverse(mod) * acc).toLong % mod).toLong
      if (res < 0) res + mod else res
    }

    override def apply(n: Int, a: Int, b: Int): Long = {
      assert(1 <= a && a <= b && b <= math.min(n, 2 * math.pow(10, 5)))
      assert(2 <= n && n <= math.pow(10, 9))
      val mod = 1000000007
      val ans = repeatedSquaresWithMod(2, n, mod) - nCkWithMod(n, a, mod) - nCkWithMod(n, b, mod) - 1
      (ans + 2 * mod) % mod
    }
  }
}
