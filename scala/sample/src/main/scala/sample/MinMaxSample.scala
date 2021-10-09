package sample

object MinMaxSample extends App {
  // Int
  assert(Int.MinValue == -math.pow(2, 31))
  assert(Int.MaxValue == math.pow(2, 31) - 1)
  assert(Int.MinValue == -2147483648)
  assert(Int.MaxValue == 2147483647)

  // Long
  assert(Long.MinValue == -math.pow(2, 63))
  assert(Long.MaxValue.toDouble == math.pow(2, 63) - 1)
  assert(Long.MinValue == -9223372036854775808L)
  assert(Long.MaxValue == 9223372036854775807L)

  // BigInt

  // compare
  assert(Long.MaxValue < Double.MaxValue)
  assert(math.pow(10, 9) < Int.MaxValue)
  assert(Int.MaxValue < math.pow(10, 10))

}
