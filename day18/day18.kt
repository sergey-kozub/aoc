import java.io.File

class Calc(val expr: String, val prec: Boolean) {
    val solve = { eval(0).first }

    fun eval(start: Int): Pair<Long,Int> {
        val items = mutableListOf<Pair<Char,Long>>()
        var pos = start; var op = '+'
        while (pos < expr.length) {
            when (expr[pos]) {
                in '0'..'9' -> {
                    val n = (expr[pos] - '0').toLong()
                    items.add(op to n)
                }
                '*', '+' -> op = expr[pos]
                '(' -> {
                    val (n, len) = eval(pos + 1)
                    items.add(op to n)
                    pos += len + 1
                }
                ')' -> break
            }
            pos += 1
        }
        val result = if (!prec) items.fold(0L) { a, b ->
            when (b.first) {
                '+' -> a + b.second
                '*' -> a * b.second
                else -> a
            }
        } else {
            val acc = mutableListOf(0L)
            items.forEach { when (it.first) {
                '+' -> acc[acc.lastIndex] += it.second
                '*' -> acc.add(it.second)
            }}
            acc.fold(1L) { a, b -> a * b }
        }
        return Pair(result, pos - start)
    }
}

fun main() {
    val input = File("day18.txt").readLines()
    val sumAll = { prec: Boolean -> input.map { Calc(it, prec).solve() }.sum() }
    println(sumAll(false))
    println(sumAll(true))
}
