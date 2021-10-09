package sample

object RepeatedSquaresWithModSample extends App {

  /** 繰り返し2乗法とMod */
  trait RepeatedSquaresWithMod {
    def apply(x: Int, y: Int, mod: Int): Long
  }

  class RepeatedSquaresWithMod1 extends RepeatedSquaresWithMod {
    override def apply(x: Int, y: Int, mod: Int): Long = {
      assert(0 <= x && 0 <= y)
      val binaryList = y.toBinaryString.reverse.toList.map(_.toString.toInt)
      val (acc, _) = binaryList.foldLeft((BigInt(1), x: BigInt)) { case ((acc, kake), bit) =>
        val newAcc = {
          if (bit == 1) (acc * kake).mod(mod)
          else acc
        }
        (newAcc, (kake * kake).mod(mod))
      }
      acc.mod(mod).toLong
    }
  }

  private def check[A](actual: A, expected: A): Unit =
    if (actual != expected) {
      val callerLine = (new Throwable).getStackTrace.toList(1).getLineNumber
      println(s"(actual, expected) = ($actual, $expected): on $callerLine")
    }

  def test(repeatedSquaresWithMod: RepeatedSquaresWithMod): Unit = {
    println(s"test: ${repeatedSquaresWithMod.getClass.getName}")
    check(repeatedSquaresWithMod(2, 0, 3), 1)
    check(repeatedSquaresWithMod(2, 1, 3), 2)
    check(repeatedSquaresWithMod(2, 2, 3), 1)
    check(repeatedSquaresWithMod(2, 3, 3), 2)
    check(repeatedSquaresWithMod(5, 0, 3), 1)
    check(repeatedSquaresWithMod(5, 1, 3), 2)
    check(repeatedSquaresWithMod(5, 2, 3), 1)
    check(repeatedSquaresWithMod(5, 3, 3), 2)
  }

  test(new RepeatedSquaresWithMod1)

}
