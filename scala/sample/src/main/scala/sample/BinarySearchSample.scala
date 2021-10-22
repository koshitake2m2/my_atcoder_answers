package sample

import sample.MyTestLib.check

import scala.annotation.tailrec

object BinarySearchSample extends App {

  /** 二分探索木 */
  trait BinarySearch {
    def lowerBound(key: Long): Int
    def upperBound(key: Long): Int
    def count(key: Long): Int = upperBound(key) - lowerBound(key)
  }

  /** @param array 単調増加な配列.
    * @param f arrayの要素に適用する関数.
    */
  case class BinarySearch1(
      array: Array[Long],
      f: Long => Long
  ) extends BinarySearch {

    /** key <= f(array(index)) となる最小のindexを求める.
      * この時, f(array(x))が単調増加であれば, leftをngとすれば扱いやすくなる.
      * 最小のindex見つからない場合は, array.sizeを返す.
      */
    def lowerBound(key: Long): Int = {
      @tailrec
      def lowerBound(ng: Int, ok: Int): Int =
        if (ok - ng == 1) {
          ok
        } else {
          val mid = (ng + ok) / 2
          if (key <= f(array(mid))) {
            lowerBound(ng, mid)
          } else {
            lowerBound(mid, ok)
          }
        }
      lowerBound(-1, array.length)
    }

    /** key < f(array(index)) となる最小のindexを求める.
      * 最小のindex見つからない場合は, array.sizeを返す.
      */
    def upperBound(key: Long): Int = {
      @tailrec
      def upperBound(ng: Int, ok: Int): Int =
        if (ok - ng == 1) {
          ok
        } else {
          val mid = (ng + ok) / 2
          if (key < f(array(mid))) {
            upperBound(ng, mid)
          } else {
            upperBound(mid, ok)
          }
        }
      upperBound(-1, array.length)
    }
  }

  val array1: Array[Long] = Array(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
  val array2: Array[Long] = Array(1, 4, 9, 16, 25, 36, 49, 64, 81, 100)
  val array3: Array[Long] = Array(1, 4, 9, 9, 9, 10, 16, 25, 36, 49, 64, 81, 100)

  val bs1 = BinarySearch1(array1, identity)
  check(bs1.lowerBound(4), 3)
  check(bs1.upperBound(4), 4)
  val bs2 = BinarySearch1(array2, identity)
  check(bs2.lowerBound(8), 2)
  check(bs2.upperBound(8), 2)
  check(bs2.lowerBound(0), 0)
  check(bs2.upperBound(0), 0)
  check(bs2.lowerBound(1), 0)
  check(bs2.upperBound(1), 1)
  check(bs2.lowerBound(100), 9)
  check(bs2.upperBound(100), 10)
  check(bs2.lowerBound(101), 10)
  check(bs2.upperBound(101), 10)
  val bs3 = BinarySearch1(array3, identity)
  check(bs3.lowerBound(9), 2)
  check(bs3.upperBound(9), 5)
  check(bs3.count(9), 3)
  check(bs3.count(11), 0)
  println("done")

}
