package sample

object FormatSample extends App {
  val a = math.pow(10, 9)
  var i = 0
  while (a > math.pow(2, i))
    i += 1
  println(i)
  println(f"${math.pow(2, i)}%.0f")
  println(f"$a%.0f")
}
