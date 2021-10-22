package sample

object MyTestLib {
  def check[A](actual: A, expected: A): Unit =
    if (actual != expected) {
      val callerLine = (new Throwable).getStackTrace.toList(1).getLineNumber
      println(s"(actual: $actual, expected: $expected): on $callerLine")
    }
}
