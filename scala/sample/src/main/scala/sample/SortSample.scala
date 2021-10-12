package sample

object SortSample extends App {

  val list1 = List(1, 2, 3, 1, 2, 3)
  // 昇順
  check(list1.sorted, List(1, 1, 2, 2, 3, 3))
  check(list1.sortBy(identity), List(1, 1, 2, 2, 3, 3))

  // 降順
  check(list1.sortBy(-1 * _), List(3, 3, 2, 2, 1, 1))

  private def check[A](actual: A, expected: A): Unit =
    if (actual != expected) {
      val callerLine = (new Throwable).getStackTrace.toList(1).getLineNumber
      println(s"(actual, expected) = ($actual, $expected): on $callerLine")
    }
}
