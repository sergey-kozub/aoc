import java.io.File

fun main() {
    data class Line(val min: Int, val max: Int, val letter: Char, val value: String)
    val input = File("day2.txt").readLines()
        .map { Regex("""(\d+)-(\d+) (\w): (\w+)""").matchEntire(it)!!.groupValues }
        .map { Line(it[1].toInt(), it[2].toInt(), it[3][0], it[4]) }
    println(input.count { x -> x.value.count { it == x.letter } in x.min..x.max })
    println(input.count { x -> listOf(x.min, x.max).count { x.value[it - 1] == x.letter } == 1 })
}
