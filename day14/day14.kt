import java.io.File

val toBin = { v: Long -> java.lang.Long.toBinaryString(v).padStart(36, '0') }
data class SetMask(val mask: String)
data class WriteValue(val addr: Long, val value: Long)

fun main() {
    val program = File("day14.txt").readLines().map {
        val m = Regex("""mask = ([X01]+)|mem\[(\d+)\] = (\d+)""").find(it)!!.groups
        if (m[1] != null) SetMask(m[1]!!.value) else
            WriteValue(m[2]!!.value.toLong(), m[3]!!.value.toLong())
    }

    val memory = mutableMapOf<Long,Long>()
    var mask = ""
    program.forEach {
        when {
            it is SetMask -> mask = it.mask
            it is WriteValue -> {
                memory[it.addr] = mask.zip(toBin(it.value)) {
                    a, b -> if (a == 'X') b else a
                }.joinToString("").toLong(2)
            }
        }
    }
    println(memory.values.sum())

    memory.clear()
    fun write(addr: String, value: Long) {
        if (addr.contains('X')) {
            write(addr.replaceFirst('X', '0'), value)
            write(addr.replaceFirst('X', '1'), value)
        } else memory[addr.toLong(2)] = value
    }
    program.forEach {
        when {
            it is SetMask -> mask = it.mask
            it is WriteValue -> {
                write(mask.zip(toBin(it.addr)) {
                    a, b -> if (a == '0') b else a
                }.joinToString(""), it.value)
            }
        }
    }
    println(memory.values.sum())
}
