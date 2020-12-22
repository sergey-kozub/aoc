import java.io.File

typealias Cards = List<Int>
fun play(player1: Cards, player2: Cards, recursive: Boolean): Pair<Cards, Cards> {
    val played = mutableSetOf<List<Int>>()
    val p1 = ArrayDeque<Int>(player1)
    val p2 = ArrayDeque<Int>(player2)
    while (p1.isNotEmpty() && p2.isNotEmpty()) {
        val state = listOf(p1.size) + p1 + p2
        if (played.contains(state)) break
        val c1 = p1.removeFirst()
        val c2 = p2.removeFirst()
        var win = if (recursive && p1.size >= c1 && p2.size >= c2) {
            val (t1, _) = play(p1.take(c1), p2.take(c2), true)
            t1.size > 0
        } else c1 > c2
        if (win) p1.addAll(listOf(c1, c2)) else p2.addAll(listOf(c2, c1))
        played.add(state)
    }
    return Pair(p1.toList(), p2.toList())
}

fun score(res: Pair<Cards, Cards>) = 
    (if (res.first.size > 0) res.first else res.second).reversed()
    .mapIndexed { k, v -> v * (k + 1) }.sum()

fun main() {
    val input = File("day22.txt").readText().trimEnd()
    val (player1, player2) = input.split("\n\n").map {
        it.split('\n').drop(1).map { it.toInt() }
    }
    println(score(play(player1, player2, false)))
    println(score(play(player1, player2, true)))
}
