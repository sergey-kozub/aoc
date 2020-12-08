import java.io.File

fun main() {
    val program = File("day8.txt").readLines().map {
        val (instr, value) = it.split(' ')
        instr to value.toInt()
    }

    fun emulate(program: List<Pair<String,Int>>): Pair<Int,Boolean> {
        var acc = 0; var ptr = 0
        val visited = mutableSetOf<Int>()
        while (ptr < program.size && !visited.contains(ptr)) {
            visited.add(ptr)
            val (instr, value) = program[ptr]
            if (instr == "jmp") { ptr += value; continue }
            if (instr == "acc") acc += value
            ptr += 1
        }
        return acc to visited.contains(ptr)
    }
    println(emulate(program).first)

    program.mapIndexed { idx, (instr, value) ->
        if (instr == "acc") null else {
            val line = (if (instr != "jmp") "jmp" else "nop") to value
            val test = program.take(idx) + listOf(line) + program.drop(idx + 1)
            val (acc, loop) = emulate(test)
            if (loop) null else acc
        }
    }.filterNotNull()
}
