class Circle(input: String, val total: Int) {
    data class Node(val label: Int, var next: Node?)
    val nodes = List<Node>(total) { Node(it + 1, null) }

    init {
        val a = input.map { it - '1' }
        (0 until total).map { i ->
            val j = a.getOrElse(i) { it }
            val k = a.getOrElse(i + 1) { if (it < total) it else a[0] }
            nodes[j].next = nodes[k]
        }
    }
    
    fun slice(label: Int, count: Int): List<Int> {
        var node = nodes[label - 1]
        return (1..count).map { node = node.next!!; node.label }
    }
    
    fun move(label: Int, count: Int, block: Int = 3) {
        var node = nodes[label - 1]
        (1..count).forEach {
            var temp = node
            val slice = (0..block).map { temp = temp.next!!; temp }
            val labels = slice.dropLast(1).map { it.label }.toSet()
            var n = node.label
            do { n = if (n > 1) n - 1 else total } while (labels.contains(n))
            val dest = nodes[n - 1]
            slice[slice.size - 2].next = dest.next
            dest.next = slice[0]
            node.next = slice[slice.lastIndex]
            node = node.next!!
        }
    }
}

fun main() {
    val input = "137826495"

    val small = Circle(input, 9)
    small.move(1, 100)
    println(small.slice(1, 8).joinToString(""))

    val big = Circle(input, 1000000)
    big.move(1, 10000000)
    println(big.slice(1, 2).fold(1L) { a, b -> a * b })
}
