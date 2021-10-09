package sample

object CombinationSample extends App {

  /** 組み合わせの数. */
  trait Combination {
    def apply(n: Int, k: Int): Long
  }

  /** 素朴な実装: nCk = n! / (k! * (n - k)!) */
  class Combination1 extends Combination {
    private def nPn(n: Int): Long = (1 to n).foldLeft(1L)(_ * _)
    override def apply(n: Int, k: Int): Long = {
      assert(0 < n)
      nPn(n) / (nPn(k) * nPn(n - k))
    }
  }

  /** 素朴な実装: nCk = n*(n-1)*(n-2)*...*(n-k+1) / k! */
  class Combination2 extends Combination {
    override def apply(n: Int, k: Int): Long = {
      val bunshi = (n - k + 1 to n).foldLeft(1L)((acc, kk) => acc * kk)
      (1 to k).foldLeft(bunshi)((acc, kk) => acc / kk)
    }
  }

  /** 素朴な実装のメモ化: nCk = n! / (k! * (n - k)!) */
  class Combination3 extends Combination {
    override def apply(n: Int, k: Int): Long = {
      val arrayBuffer = scala.collection.mutable.ArrayBuffer.empty[Long]
      arrayBuffer += 1
      (1 to n).foreach(i => arrayBuffer += i * arrayBuffer(i - 1))

      arrayBuffer(n) / (arrayBuffer(k) * arrayBuffer(n - k))
    }
  }

  private def check[A](actual: A, expected: A): Unit =
    if (actual != expected) { println(s"(actual, expected) = ($actual, $expected)") }

  def test(combination: Combination): Unit = {
    println(s"test: ${combination.getClass.getName}")
    check(combination(1, 0), 1)
    check(combination(1, 1), 1)
    check(combination(2, 0), 1)
    check(combination(2, 1), 2)
    check(combination(2, 2), 1)
    check(combination(3, 0), 1)
    check(combination(3, 1), 3)
    check(combination(3, 2), 3)
    check(combination(3, 3), 1)
    check(combination(4, 0), 1)
    check(combination(4, 1), 4)
    check(combination(4, 2), 6)
    check(combination(4, 3), 4)
    check(combination(4, 4), 1)
    check(combination(5, 0), 1)
    check(combination(5, 1), 5)
    check(combination(5, 2), 10)
    check(combination(5, 3), 10)
    check(combination(5, 4), 5)
    check(combination(5, 5), 1)
    check(combination(6, 0), 1)
    check(combination(6, 1), 6)
    check(combination(6, 2), 15)
    check(combination(6, 3), 20)
    check(combination(7, 0), 1)
    check(combination(7, 1), 7)
    check(combination(7, 2), 21)
    check(combination(7, 3), 35)
    check(combination(8, 0), 1)
    check(combination(8, 1), 8)
    check(combination(8, 2), 28)
    check(combination(8, 3), 56)
    check(combination(8, 4), 70)
  }

  test(new Combination1)
  test(new Combination2)
  test(new Combination3)

}
