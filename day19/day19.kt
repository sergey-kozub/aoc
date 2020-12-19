import java.io.File

fun main() {
    val input = File("day19.txt").readText()
    val (allRules, allMessages) = input.split("\n\n")
    val rules = allRules.split("\n").map { it.split(": ") }.map {
        (k, v) -> k.toInt() to if (v[0] == '"') v[1].toString() else
            v.split(" | ").map { it.split(" ").map { it.toInt() } }
    }.toMap()
    val messages = allMessages.trimEnd().split("\n")

    // Part 1
    val options = mutableMapOf<Int,Set<String>>()
    rules.forEach { k, v -> if (v is String) options[k] = setOf(v) }

    fun build(key: Int): Set<String> {
        return options.getOrElse(key) {
            val rule = rules[key] as List<List<Int>>
            rule.flatMap {
                it.fold(listOf("")) {
                    a, b -> a.flatMap { s -> build(b).map { s + it } }
                }
            }.toSet().also { options[key] = it }
        }
    }

    build(0)
    println(messages.count { it in options[0]!! })

    // Part 2
    val l42 = options[42]!!.first().length
    val l31 = options[31]!!.first().length
    val matches_8 = { msg: String ->
        options[42]!!.any { msg.startsWith(it) } &&
        (msg.length == l42 || matches_8(msg.drop(l42))) }
    val matches_11 = { msg: String ->
        options[42]!!.any { msg.startsWith(it) } &&
        options[31]!!.any { msg.endsWith(it) } &&
        (msg.length == l42 + l31 || matches_11(msg.drop(l42).dropLast(l31))) }
    val matches_0 = { msg: String -> (0..msg.length).any {
        matches_8(msg.take(it)) && matches_11(msg.drop(it)) } }
    println(messages.count { matches_0(it) })
}
