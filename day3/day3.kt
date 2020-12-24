import java.io.File

fun main() {
    val input = File("day3.txt").readLines()
    fun count(dx: Int, dy: Int) = (0 until input.size step dy).count {
        input[it][(it * dx / dy) % input[0].length] == '#' }
    println(count(3, 1))
    println(listOf(1 to 1, 3 to 1, 5 to 1, 7 to 1, 1 to 2).map {
        (dx, dy) -> count(dx, dy) }.fold(1L) { a, b -> a * b })
}
