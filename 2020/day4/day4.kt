import java.io.File

val rules = mapOf(
    "byr" to { s: String -> s.toIntOrNull() in 1920..2002 },
    "iyr" to { s: String -> s.toIntOrNull() in 2010..2020 },
    "eyr" to { s: String -> s.toIntOrNull() in 2020..2030 },
    "hgt" to { s: String -> when {
        s.endsWith("cm") -> s.dropLast(2).toIntOrNull() in 150..193
        s.endsWith("in") -> s.dropLast(2).toIntOrNull() in 59..76
        else -> false
    }},
    "hcl" to { s: String -> Regex("#[0-9a-f]{6}").matches(s) },
    "ecl" to { s: String -> Regex("amb|blu|brn|gry|grn|hzl|oth").matches(s) },
    "pid" to { s: String -> s.length == 9 && s.toIntOrNull() != null },
)

fun main() {
    val input = File("day4.txt").readText().split("\n\n")
        .map { Regex("""(\w+):(\S+)""").findAll(it)
        .map { it.groupValues[1] to it.groupValues[2] }.toMap() }
    println(input.count { item -> rules.keys.all { item.contains(it) } })
    println(input.count { item -> rules.entries.all {
        (key, test) -> test(item.getOrDefault(key, "")) } })
}
