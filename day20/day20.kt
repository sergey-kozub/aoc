import java.io.File

class Tile(val id: Int, val grid: List<String>) {
    override fun hashCode(): Int = id
    override fun toString(): String {
        return "Tile $id:\n" + grid.joinToString("\n")
    }
    
    fun rotated() = Tile(id, (0 until grid.size).map {
        i -> grid.fold("") { a, b -> a + b[i] }.reversed() })
    fun hflipped() = Tile(id, grid.map { it.reversed() })
    fun vflipped() = Tile(id, grid.reversed())
    fun option(n: Int) = when (n) {
        1 -> rotated()
        2 -> hflipped()
        3 -> vflipped()
        4 -> rotated().rotated()
        5 -> rotated().hflipped()
        6 -> rotated().vflipped()
        7 -> rotated().rotated().rotated()
        else -> this
    }
    
    fun matchTop(other: Tile) = grid.first() == other.grid.last()
    fun matchBottom(other: Tile) = grid.last() == other.grid.first()
    fun matchLeft(other: Tile) = (0 until grid.size).all {
        grid[it].first() == other.grid[it].last() }
    fun matchRight(other: Tile) = (0 until grid.size).all {
        grid[it].last() == other.grid[it].first() }
    
    fun findAll(pattern: List<String>): Int {
        val pos = pattern.flatMapIndexed { y, s -> s.mapIndexedNotNull {
            x, c -> if (c == '#') Pair(x, y) else null } }.toSet()
        return (0..grid.size - pattern.size).flatMap { y -> 
            (0..grid[0].length - pattern[0].length).map { x -> Pair(x, y) }
        }.count { (x, y) -> pos.all {
            (dx, dy) -> grid[y + dy][x + dx] == '#'
        }}
    }
}

fun main() {
    val input = File("day20.txt").readText().trimEnd()
    val tiles = input.split("\n\n").map { it.split("\n") }.map {
        Tile(it[0].split(" ")[1].trimEnd(':').toInt(), it.drop(1)) }

    // Part 1
    val pos = mutableMapOf(Pair(0, 0) to tiles[0])
    val unassigned = tiles.drop(1).toMutableSet()
    while (unassigned.isNotEmpty()) {
        val (key, item) = unassigned.asSequence().flatMap { tile ->
            (0..7).map { tile.option(it) }.flatMap { temp ->
                pos.entries.map { (at, other) -> 
                    val (x, y) = at; val v = tile to temp
                    when {
                        temp.matchTop(other) -> Pair(x, y - 1) to v
                        temp.matchRight(other) -> Pair(x - 1, y) to v
                        temp.matchBottom(other) -> Pair(x, y + 1) to v
                        temp.matchLeft(other) -> Pair(x + 1, y) to v
                        else -> null
                    }
                }.filterNotNull()
            }
        }.take(1).toList()[0]
        pos[key] = item.second
        unassigned.remove(item.first)
    }

    val xmin = pos.keys.minOf { it.first }
    val xmax = pos.keys.maxOf { it.first }
    val ymin = pos.keys.minOf { it.second }
    val ymax = pos.keys.maxOf { it.second }
    val corners = listOf(xmin to ymin, xmax to ymin, xmax to ymax, xmin to ymax)
    println(corners.fold(1L) { a, b -> a * pos[b]!!.id })

    // Part 2
    val sea = Tile(0, (ymax downTo ymin).flatMap { ty -> (1..8).map { y ->
        (xmin..xmax).flatMap { tx -> (1..8).map { x ->
            pos[Pair(tx, ty)]!!.grid[y][x]
        }}.joinToString("")
    }})
    val monster = """
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
""".trim('\n').split('\n')
    val nMonsters = (0..7).map { sea.option(it) }.maxOf { it.findAll(monster) }
    print("Monsters: $nMonsters")

    fun count(a: List<String>) = a.map { it.count { it == '#' } }.sum()
    println(count(sea.grid) - nMonsters * count(monster))
}
