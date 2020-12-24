import java.io.File

fun main() {
    val input = File("day5.txt").readLines()
    val seats = input.map { it.fold(0) { n, c ->
        n * 2 + if (c == 'B' || c == 'R') 1 else 0 } }.toSet()
    println(seats.max())
    val allSeats = (seats.min()!!..seats.max()!!).toSet()
    println(allSeats - seats)
}
