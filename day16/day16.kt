import java.io.File

fun <E> List<List<E>>.transpose(): List<List<E>> {
    return (0 until first().size).map { col ->
        (0 until size).map { row -> this[row][col] }
    }
}

fun main() {
    val content = File("day16.txt").readText().trimEnd().split("\n\n")
    val rules = Regex("""([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)""").findAll(content[0]).map {
        val (r1, r2) = it.groupValues.drop(2).map { it.toInt() }.chunked(2).map { (a, b) -> (a..b) }
        it.groupValues[1] to r1.toSet() + r2.toSet()
    }.toMap()
    val parse = { s: String -> s.split('\n').drop(1).map { it.split(',').map { it.toInt() } } }
    val yourTicket = parse(content[1])[0]
    val nearbyTickets = parse(content[2])

    val values = rules.values.flatten().toSet()
    val validTickets = nearbyTickets.filter { it.all { it in values } }
    println(nearbyTickets.flatten().filter { it !in values }.sum())

    val options = validTickets.transpose().mapIndexed { k, v ->
        k to rules.entries.mapNotNull { (name, valid) ->
            if (v.all { it in valid }) name else null
        }
    }.sortedBy { it.second.size }
    val names = options.fold(mutableMapOf<String,Int>()) {
        m, (k, v) -> m[v.filter { it !in m }[0]] = k; m
    }

    val departure = rules.keys.filter { it.startsWith("departure") }.map { yourTicket[names[it]!!] }
    println(departure.fold(1L) { a, b -> a * b })
}
