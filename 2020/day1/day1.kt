import java.io.File

fun main() {
    val S = File("day1.txt").readLines()
        .map { it.toInt() }.toSet()
    val r1 = S.filter { (2020 - it) in S }.first()
    val r2 = S.flatMap { a -> S.mapNotNull { b ->
        if ((2020 - a - b) in S) Pair(a, b) else null } }.first()
    println(r1 * (2020 - r1))
    println(r2.first * r2.second * (2020 - r2.toList().sum()))
}
