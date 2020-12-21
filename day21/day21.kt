import java.io.File

fun main() {
    val input = File("day21.txt").readLines().map {
        val m = Regex("""(.+) \(contains (.+)\)""").matchEntire(it)!!
        Pair(m.groupValues[1].split(" ").toSet(), m.groupValues[2].split(", ").toSet())
    }

    val allergens = mutableMapOf<String,String>()
    val unknown = input.flatMap { it.second }.toMutableSet()
    while (unknown.isNotEmpty()) {
        val (l, r) = unknown.asSequence().mapNotNull { test ->
            val (l, r) = input.filter { it.second.contains(test) }.unzip().toList()
                .map { it.reduce { a, b -> a intersect b }.toMutableSet() }
            l -= allergens.keys
            r -= allergens.values
            if (l.isNotEmpty() && l.size == r.size) l to r else null
        }.first()
        l.zip(r).associateTo(allergens) { it }
        unknown -= r
    }

    println(input.map { (it.first - allergens.keys).size }.sum())
    println(allergens.entries.sortedBy { it.value }.map { it.key }.joinToString(","))
}
