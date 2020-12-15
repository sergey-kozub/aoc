import java.io.File

fun play(N: Int): Int {
    val spoken = input.mapIndexed { k, v -> v to k + 1 }
        .toMap().toMutableMap()
    var prev = input.last()
    for (i in input.size..(N - 1)) {
        val pos = spoken.get(prev)
        spoken[prev] = i
        prev = if (pos != null) i - pos else 0
    }
    return prev
}

fun main() {
    val input = File("aoc2020/day15/day15.txt").readText().trim()
        .split(',').map { it.toInt() }
    println(play(2020))
    println(play(30000000))
}
