fun main() {
    val M = 20201227
    fun encode(s: Int) = generateSequence(1L) { it * s % M }
    fun transform(s: Int, n: Int) = encode(s).take(n + 1).last()
    fun reverse(x: Int) = encode(7).withIndex()
        .takeWhile { (_, v) -> v.toInt() != x }.last().index + 1

    val (A, B) = Pair(10705932, 12301431)
    println(transform(B, reverse(A)))
    println(transform(A, reverse(B)))
}
