package sample

object SinCosTanSample extends App {
  // 1 [radian] = 180.0 / Pi [degree]
  // 1 [degree] = Pi / 180.0 [radian]

  /** ラジアン */
  case class Radian(toDouble: Double) {
    def toStringWithPi: String = s"${toDouble / math.Pi} * π [rad]"
    def toDegree: Degree = Degree(toDouble * 180.0 / math.Pi)
  }

  /** 度 */
  case class Degree(toDouble: Double) {
    def toRadian: Radian = Radian(toDouble * math.Pi / 180.0)
  }

  val degree45 = Degree(45.0)
  println(s"45°: $degree45")
  println(s"45° to radian: ${degree45.toRadian}")
  println(s"45° to radian with pi: ${degree45.toRadian.toStringWithPi}")

  println(s"tan(π/4)=1, actual=${math.tan(degree45.toRadian.toDouble)}")
  println(s"arctan(1)=π/4, actual=${math.atan(1)}")

}
