import java.io.File

fun main() {
    val input = File("day10.txt").readLines().map { it.toInt() }.sorted()
    val input0 = listOf(0) + input
    val diff = input0.windowed(2).map { it[1] - it[0] }.groupingBy { it }.eachCount()
    println(diff.get(1)!! * (1 + diff.get(3)!!))

    val a = input0.reversed()
    var m = listOf(1L, 0L, 0L)
    for (i in 0..input.size) {
        val s = m[0] + 
            (if (i > 1 && a[i-2] - a[i] <= 3) m[1] else 0) +
            (if (i > 2 && a[i-3] - a[i] <= 3) m[2] else 0)
        m = listOf(s, m[0], m[1])
    }
    println(m[0])
}
