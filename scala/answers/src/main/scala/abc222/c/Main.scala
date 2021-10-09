package abc222.c

import java.util.Scanner

object SolveTests {
  import Solves._
  private def check[A](actual: A, expected: A): Unit =
    if (actual != expected) {
      val callerLine = (new Throwable).getStackTrace.toList(1).getLineNumber
      println(s"(actual, expected) = ($actual, $expected): on $callerLine")
    }
  implicit class StringOpt(value: String) {
    def toStringListList: List[List[String]] = value.split("\n").toList.map(_.toList.map(_.toString))
    def toIntList: List[Int] = value.split("\n").map(_.toInt).toList
  }

  def test(solve: Solve): Unit = {
    check(
      solve(2, 3, "GCP\nPPP\nCCC\nPPC".toStringListList),
      "3\n1\n2\n4".toIntList
    )
    check(
      solve(2, 2, "GC\nPG\nCG\nPP".toStringListList),
      "1\n2\n3\n4".toIntList
    )
  }

}

object Solves {
  trait Solve {
    def apply(n: Int, m: Int, anm: List[List[String]]): Seq[Int]
  }

  // submission in contest
  class Solve1 extends Solve {
    case class Person(entry: Int, var count: Int, strategy: Seq[String]) {
      def win(): Unit = count = count + 1
    }
    def round(m: Int, personA: Person, personB: Person): Unit = {
      val a = personA.strategy(m)
      val b = personB.strategy(m)
      a match {
        case "G" =>
          b match {
            case "G" => ()
            case "C" => personA.win()
            case "P" => personB.win()
          }
        case "C" =>
          b match {
            case "G" => personB.win()
            case "C" => ()
            case "P" => personA.win()
          }
        case "P" =>
          b match {
            case "G" => personA.win()
            case "C" => personB.win()
            case "P" => ()
          }
      }
    }
    override def apply(n: Int, m: Int, anm: List[List[String]]): Seq[Int] = {
      var persons = (0 to 2 * n - 1).map { i =>
        val starategy = anm(i)
        Person(i, 0, starategy)
      }
      (0 to m - 1).foreach { mm =>
        (0 to n - 1).foreach { nn =>
          round(mm, persons(2 * nn), persons(2 * nn + 1))
        }
        persons = persons.sortBy(_.entry).sortBy(_.count * -1)
      }
      persons.map(_.entry).map(_ + 1)
    }

  }

  // refactor after Contest
  class Solve2 extends Solve {

    import Janken._
    sealed abstract class Janken
    object Janken {
      case object Gu extends Janken
      case object Choki extends Janken
      case object Pa extends Janken
      def of(janken: String): Janken =
        janken match {
          case "G" => Gu
          case "C" => Choki
          case "P" => Pa
          case _ => throw new Throwable("unexpected Janken string")
        }
    }

    /** じゃんけんする人.
      * @param entry 番号.
      * @param count じゃんけんで勝った数.
      * @param strategy Mラウンド目のじゃんけんでだすもの.
      */
    case class Person(entry: Int, count: Int, strategy: Seq[Janken]) {
      def win: Person = this.copy(count = count + 1)
    }

    /** ラウンドMでのじゃんけん. */
    def round(m: Int, personA: Person, personB: Person): (Person, Person) = {
      val jankenA = personA.strategy(m)
      val jankenB = personB.strategy(m)
      (jankenA, jankenB) match {
        case (Gu, Choki) | (Choki, Pa) | (Pa, Gu) =>
          (personA.win, personB)
        case (Choki, Gu) | (Pa, Choki) | (Gu, Pa) =>
          (personA, personB.win)
        case _ =>
          (personA, personB)
      }
    }

    override def apply(n: Int, m: Int, anm: List[List[String]]): Seq[Int] = {
      val defaultPersons: Seq[Person] = (0 until 2 * n).map(i => Person(i + 1, 0, anm(i).map(Janken.of(_))))
      val resultPersons: Seq[Person] = (0 until m).foldLeft(defaultPersons) { (persons, mm) =>
        val personsAfterRound: Seq[Person] = (0 until n).flatMap { nn =>
          val pair = round(mm, persons(2 * nn), persons(2 * nn + 1))
          List(pair._1, pair._2)
        }
        personsAfterRound.sortBy(_.entry).sortBy(_.count * -1)
      }
      resultPersons.map(_.entry)
    }

  }
}

object Main extends App {
  import Solves._
  import SolveTests._

  def main: Unit = {
    val solve = new Solve2
    val sc = new Scanner(System.in)
    val n, m = sc.nextInt
    val anm: List[List[String]] = List.fill(2 * n)(sc.next().toList.map(_.toString))
    solve(n, m, anm).foreach(res => println(res))
  }

  test(new Solve1())
  test(new Solve2())
  main
}
