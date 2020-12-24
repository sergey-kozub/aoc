import java.io.File

class Floor(input: List<String>) {
    data class Tile(val x: Int, val y: Int)
    val tiles = mutableSetOf<Tile>()
    val dir = mapOf(
        "e" to Pair(1, 0),
        "w" to Pair(-1, 0),
        "ne" to Pair(0, 1),
        "nw" to Pair(-1, 1),
        "se" to Pair(1, -1),
        "sw" to Pair(0, -1),
    )
    
    init {
        input.map { parse(it) }.forEach {
            if (it !in tiles) tiles.add(it)
            else tiles.remove(it)
        }
    }

    fun parse(line: String): Tile {
        var p = 0
        var x = 0; var y = 0
        while (p < line.length) {
            val single = dir.contains(line[p].toString())
            val key = line.substring(p, p + if (single) 1 else 2)
            p += key.length
            x += dir[key]!!.first
            y += dir[key]!!.second
        }
        return Tile(x, y)
    }
    
    fun move() {
        val state = tiles.toSet()
        val white = mutableSetOf<Tile>()
        for (b in state) {
            val a = dir.values.map { (dx, dy) -> Tile(b.x + dx, b.y + dy) }
            val n = a.count { it in state }
            if (n == 0 || n > 2) tiles.remove(b)
            white.addAll(a)
        }
        for (w in white.sortedWith { a, b -> if (a.x != b.x) a.x - b.x else a.y - b.y }) {
            val a = dir.values.map { (dx, dy) -> Tile(w.x + dx, w.y + dy) }
            val n = a.count { it in state }
            if (n == 2) tiles.add(w)
        }
    }
}

fun main() {
    val input = File("day24.txt").readLines()
    val floor = Floor(input)
    println(floor.tiles.size)
    (1..100).forEach { floor.move() }
    println(floor.tiles.size)
}
