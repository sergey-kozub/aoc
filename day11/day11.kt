import java.io.File

class Grid(input: List<String>, val near: Boolean) {
    enum class Cell { FLOOR, AVAILABLE, OCCUPIED }
    val width = input[0].trimEnd().length;
    val height = input.size;
    var buffer = List<Cell>(width * height) {
        when (input[it / width][it % width]) {
            'L' -> Cell.AVAILABLE
            else -> Cell.FLOOR
        }
    }
    
    fun seat(row: Int, col: Int): Cell? {
        return if (row in 0..height-1 && col in 0..width-1)
            buffer[row * width + col] else null
    }
    
    fun visible(row: Int, col: Int, dx: Int, dy: Int, dist: Int): Cell? {
        for (i in 1..dist) {
            val cell = seat(row + dy * i, col + dx * i)
            if (cell != Cell.FLOOR) return cell
        }
        return null
    }
    
    fun nextCell(row: Int, col: Int): Cell {
        val cell = seat(row, col)!!
        if (cell == Cell.FLOOR) return cell
        val count = (-1..1).flatMap { i -> (-1..1).map { j -> i to j } }.map {
            (i, j) -> if (i == 0 && j == 0) null else
            visible(row, col, i, j, if (near) 1 else 1000)
        }.count { it == Cell.OCCUPIED }
        val occupied = cell == Cell.OCCUPIED
        return when {
            if (occupied) count < if (near) 4 else 5 else count > 0 -> cell 
            else -> if (occupied) Cell.AVAILABLE else Cell.OCCUPIED
        }
    }
    
    fun updateGrid(): Boolean {
        val grid = (0..height-1).flatMap { i -> (0..width-1).map { j -> nextCell(i, j) }}
        return if (grid != buffer) { buffer = grid; true } else false
    }
    
    fun stabilize(): Int {
        while (updateGrid()) {}
        return buffer.count { it == Cell.OCCUPIED }
    }
}

fun main() {
    val input = File("day11.txt").readLines()
    println(Grid(input, true).stabilize())
    println(Grid(input, false).stabilize())
}
