import java.io.File

fun findWeak(A: List<Long>, N: Int): Long? {
    val sums = (0..N-1).flatMap { i -> (i+1..N-1).map { j -> A[i] + A[j] } }
        .groupingBy { it }.eachCount().toMutableMap()
    for (i in N..(A.size - 1)) {
        if (!sums.contains(A[i])) return A[i]
        for (j in (i - N + 1)..(i - 1)) {
            sums.merge(A[i - N] + A[j], 1) { a, b -> a - b }
            sums.merge(A[i] + A[j], 1) { a, b -> a + b }
        }
    }
    return null
}

fun findRange(A: List<Long>, W: Long): Pair<Int,Int>? {
    val psum = mutableListOf<Long>()
    A.fold(0L) { l, r -> psum.add(l + r); l + r }
    val match = (0..A.size-1).flatMap { i -> (i+1..A.size-1).mapNotNull {
        j -> if (psum[j] - psum[i] == W) i to j else null }}
    return match.getOrNull(0)
}

fun main() {
    val input = File("day9.txt").readLines().map { it.toLong() }
    val W = findWeak(input, 25)
    val R = findRange(input, W)!!
    val range = input.slice(R.first..R.second)
    println(W)
    println(range.min()!! + range.max()!!)
}
