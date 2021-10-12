package abc155.c

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
    def toStringList: List[String] = value.split("\n").toList
  }

  def test(solve: Solve): Unit = {
    check(solve(7, "beat\nvet\nbeet\nbed\nvet\nbet\nbeet".toStringList), "beet\nvet".toStringList)
    check(
      solve(8, "buffalo\nbuffalo\nbuffalo\nbuffalo\nbuffalo\nbuffalo\nbuffalo\nbuffalo".toStringList),
      "buffalo".toStringList
    )
    check(solve(7, "bass\nbass\nkick\nkick\nbass\nkick\nkick".toStringList), "kick".toStringList)
    check(solve(4, "ushi\ntapu\nnichia\nkun".toStringList), "kun\nnichia\ntapu\nushi".toStringList)
  }

}

object Solves {
  trait Solve {
    def apply(n: Int, sn: Seq[String]): Seq[String]
  }

  // TLE
  class Solve1 extends Solve {
    override def apply(n: Int, sn: Seq[String]): Seq[String] = {
      val hashMap = collection.mutable.HashMap[String, Int]()
      sn.foreach { s =>
        if (hashMap.contains(s)) {
          hashMap(s) = hashMap(s) + 1
        } else {
          hashMap(s) = 1
        }
      }
      val max = hashMap.values.max
      val maxCountStrings = hashMap.filter { case (_, count) => count == max }.keys
      maxCountStrings.toSeq.sortBy(_.toString)
    }
  }

  // TLE
  class Solve2 extends Solve {
    override def apply(n: Int, sn: Seq[String]): Seq[String] = {
      val hashMap = collection.mutable.HashMap[String, Int]()
      sn.foreach(s => hashMap(s) = hashMap.get(s).map(_ + 1).getOrElse(1))

      val max = hashMap.values.max
      val maxCountStrings = hashMap.collect { case (str, count) if count == max => str }.toSeq
      maxCountStrings.sorted
    }
  }

  // TLE
  class Solve3 extends Solve {
    override def apply(n: Int, sn: Seq[String]): Seq[String] = {
      // カウントで降順
      val seq: Seq[(String, Int)] = sn.groupBy(identity).mapValues(_.size).toSeq.sortBy(-_._2)

      val max = seq.head._2
      val maxCountStrings = seq.collect { case (str, count) if count == max => str }
      maxCountStrings.sorted
    }
  }

}

object Main extends App {
  import Solves._
  import SolveTests._

  def main: Unit = {
    val solve = new Solve3
    val sc = new Scanner(System.in)
    val n = sc.nextInt
    val sn = Seq.fill(n)(sc.next)
    solve(n, sn).foreach(println(_))
  }

  test(new Solve1())
  test(new Solve2())
  test(new Solve3())
  main
}
