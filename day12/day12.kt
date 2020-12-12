import java.io.File

class Ship(var tx: Int, var ty: Int, val movingTarget: Boolean) {
    var x: Int = 0
    var y: Int = 0
    
    fun move(cmd: Char, dist: Int) {
        when (cmd) {
            'N', 'S' -> {
                val dy = if (cmd == 'N') dist else -dist
                if (movingTarget) ty += dy else y += dy
            }
            'E', 'W' -> {
                val dx = if (cmd == 'E') dist else -dist
                if (movingTarget) tx += dx else x += dx
            }
            'L', 'R' -> {
                val px = tx; val py = ty
                val rad = (if (cmd == 'R') dist else -dist) * PI / 180
                tx = (px * cos(rad) + py * sin(rad)).roundToInt()
                ty = (-px * sin(rad) + py * cos(rad)).roundToInt()
            }
            'F' -> {
                x += tx * dist
                y += ty * dist
            }
        }
    }
    
    fun moveAll(input: List<String>): Pair<Int, Int> {
        for (line in input) move(line[0], line.drop(1).toInt())
        return Pair(x, y)
    }
}

fun main() {
    val input = File("day12.txt").readLines()
    val result = { t: Pair<Int,Int> -> t.first.absoluteValue + t.second.absoluteValue }
    println(result(Ship(1, 0, false).moveAll(input)))
    println(result(Ship(10, 1, true).moveAll(input)))
}
