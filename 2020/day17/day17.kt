import java.io.File

data class Point(val x: Int, val y: Int, val z: Int, val w: Int)

class Grid(val ndim: Int, input: List<String>) {
    var points = input.flatMapIndexed { y, s -> s.mapIndexed {
        x, c -> if (c == '#') Point(x, y, 0, 0) else null
    } }.filterNotNull().toSet()
    
    fun around(pt: Point) = sequence {
        val dw = if (ndim == 4) 1 else 0
        for (x in pt.x-1..pt.x+1)
        for (y in pt.y-1..pt.y+1)
        for (z in pt.z-1..pt.z+1)
        for (w in pt.w-dw..pt.w+dw)
            if (x != pt.x || y != pt.y || z != pt.z || w != pt.w)
                yield(Point(x, y, z, w))
    }
    
    fun neighbors(pt: Point): Int {
        return around(pt).count { it in points }
    }

    fun update() {
        val next = mutableSetOf<Point>()
        points.filterTo(next) { neighbors(it) in 2..3 }
        points.flatMap { around(it) }.filter { it !in points }.toSet()
            .filterTo(next) { neighbors(it) == 3 }
        points = next.toSet()
    }
    
    fun simulate(steps: Int): Int {
        for (i in 1..steps) update()
        return points.size
    }
}

fun main() {
    val input = File("day17.txt").readLines()
    println(Grid(3, input).simulate(6))
    println(Grid(4, input).simulate(6))
}
