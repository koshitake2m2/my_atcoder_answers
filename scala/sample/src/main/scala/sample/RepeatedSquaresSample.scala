package sample

object RepeatedSquaresSample extends App {

  /** 繰り返し2乗法 */
  trait RepeatedSquares {

    /** ロジック. ans = x ** y の計算の高速化.
      * 1. yを2の累乗の和に分解する.
      * 2. あとは例をよしなにみて.
      *
      * ex) x=2, y=11
      * {{{
      *   ans = 2 ** 11
      *       = 2 ** (8 * 1 + 4 * 1 + 2 * 0 + 1 * 0)
      *       = 2 ** (8 + 2 + 1)
      *       = (2 ** 8) * (2 ** 2) * (2 ** 1)
      * }}}
      *
      * ex) x=3, y=5
      * {{{
      *   ans = 3 ** 5
      *       = 3 ** (4 * 1 + 2 * 0 + 1 * 1)
      *       = (3 ** 4) * (3 ** 1)
      * }}}
      */
    def apply(x: Int, y: Int): Long
  }

  class RepeatedSquares1 extends RepeatedSquares {
    override def apply(x: Int, y: Int): Long = {
      assert(0 <= x && 0 <= y)
      val binaryList = y.toBinaryString.reverse.toList.map(_.toString.toInt)
      binaryList.zipWithIndex.foldLeft(1L) { case (acc, (bit, index)) =>
        if (bit == 1) acc * math.pow(x, math.pow(2, index)).toLong
        else acc
      }
    }
  }

  /** foldLeft利用. */
  class RepeatedSquares2 extends RepeatedSquares {
    override def apply(x: Int, y: Int): Long = {
      assert(0 <= x && 0 <= y)
      val binaryList = y.toBinaryString.reverse.toList.map(_.toString.toInt)
      val (acc, _) = binaryList.foldLeft((1L, x)) { case ((acc, kake), bit) =>
        val newAcc =
          if (bit == 1) acc * kake
          else acc
        (newAcc, kake * kake)
      }
      acc
    }
  }

  /** ビット演算利用. */
  class RepeatedSquares3 extends RepeatedSquares {
    override def apply(x: Int, y: Int): Long = {
      assert(0 <= x && 0 <= y)
      var kake = x
      var yy = y
      var acc = 1L
      while (yy > 0) {
        if ((yy & 1) == 1) {
          acc = acc * kake
        }
        kake = kake * kake
        yy = yy >> 1
      }
      acc
    }
  }

  private def check[A](actual: A, expected: A): Unit =
    if (actual != expected) {
      val callerLine = (new Throwable).getStackTrace.toList(1).getLineNumber
      println(s"(actual, expected) = ($actual, $expected): on $callerLine")
    }

  def test(repeatedSquares: RepeatedSquares): Unit = {
    println(s"test: ${repeatedSquares.getClass.getName}")
    check(repeatedSquares(2, 0), 1)
    check(repeatedSquares(2, 1), 2)
    check(repeatedSquares(2, 2), 4)
    check(repeatedSquares(2, 3), 8)
    check(repeatedSquares(5, 0), 1)
    check(repeatedSquares(5, 1), 5)
    check(repeatedSquares(5, 2), 25)
    check(repeatedSquares(5, 3), 125)
  }

  test(new RepeatedSquares1)
  test(new RepeatedSquares2)
  test(new RepeatedSquares3)

}
