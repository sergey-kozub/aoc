import java.io.File

fun main() {
    val input = File("day6.txt").readText().removeSuffix("\n").split("\n\n")
    val groups = input.map { t -> t.split("\n").map { s -> s.toSet() } }
    println(groups.map { g -> g.reduce { a, b -> a union b }.size }.sum())
    println(groups.map { g -> g.reduce { a, b -> a intersect b }.size }.sum())
}
