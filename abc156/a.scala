import java.util.Scanner

object Main extends App {
  val sc = new Scanner(System.in)

  val participationN = sc.nextInt()
  val displayRating = sc.nextInt()

  val internalRating = if (participationN >= 10) {
    displayRating
  } else {
    displayRating + 100 * (10 - participationN)
  }

  println(internalRating)
}
