import java.io.File

fun main() {
    val lines = File("day13.txt").readLines()
    val N = lines[0].toInt()
    val buses = lines[1].split(',').map { if (it != "x") it.toInt() else null }

    val wait = buses.filterNotNull().map { it - N % it to it }.toMap()
    val key = wait.keys.minOrNull()!!
    println(key * wait.get(key)!!)

    val lookup = buses.mapIndexed { k, v ->
        if (v != null) v to k + 1 else null }.filterNotNull()
    var start = 0L; var step = 1L
    for (idx in 1..lookup.size) {
        while (!lookup.take(idx).all { (k, v) ->
            k - start % k == (v % k).toLong() }) start += step
        step *= lookup[idx - 1].first
    }
    println(start + 1)
}
