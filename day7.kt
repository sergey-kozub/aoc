import java.io.File

val rules = mutableMapOf<String,Map<String,Int>>()
val regex = Regex("""(\d*)\s*(\w+ \w+) bags?""")
File("aoc2020/day7.txt").forEachLine {
    val matches = regex.findAll(it).map { it.groupValues }.toList()
    rules[matches[0][2]] = matches.drop(1).mapNotNull {
        if (it[2] != "no other") it[2] to it[1].toInt() else null
    }.toMap()
}

val parents = mutableMapOf<String,MutableSet<String>>()
for ((src, bags) in rules) {
    for (dst in bags.keys) {
        parents.getOrPut(dst) { mutableSetOf<String>() }.add(src)
    }
}

fun all_parents(bag: String): Set<String> {
    if (!parents.contains(bag)) return emptySet<String>()
    var res = parents.getValue(bag).toList()
    return (res + res.flatMap { all_parents(it) }).toSet()
}
println(all_parents("shiny gold").size)

fun count_all(bag: String): Int {
    if (!rules.contains(bag)) return 1
    return rules.getValue(bag).map { it.value * count_all(it.key) }.sum() + 1
}
println(count_all("shiny gold") - 1)
