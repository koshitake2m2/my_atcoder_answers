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

  /** 傾き */
  case class Slope(toDouble: Double)

  /** tan */

  /** tan
    * @param theta 角度[ラジアン]. -π/2 < theta < π/2
    * @return 傾き
    */
  def tan(theta: Radian): Slope = Slope(math.tan(theta.toDouble))

  /** arctan
    * @param m 傾き
    * @return theta ラジアン. -π/2 < theta < π/2
    */
  def atan(m: Slope): Radian = Radian(math.atan(m.toDouble))

  val degree45 = Degree(45.0)
  println(s"45°: $degree45")
  println(s"45° to radian: ${degree45.toRadian}")
  println(s"45° to radian with pi: ${degree45.toRadian.toStringWithPi}")

  println(s"tan(π/4)=1, actual=${tan(degree45.toRadian)}")
  println(s"arctan(1)=π/4, actual=${atan(Slope(1))}")

  println(s"atan(-1)=${atan(Slope(-1))}")
  println(s"tan(π/2)=∞, actual=${tan(Degree(90.0).toRadian)}")

}
