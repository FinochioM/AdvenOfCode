import java.io.File

fun main() {
    val (leftList, rightList) = File("src/list.txt").useLines{lines ->
        lines.map { line ->
            line.trim().split(Regex("\\s+")).map { it.toLong()}
        }.toList()
    }.let {it[0] to it[1]}

    val sortedLeft = leftList.sorted()
    val sortedRight = rightList.sorted()

    val totalDistance = sortedLeft.zip(sortedRight)
        .sumOf {(left, right) ->
            kotlin.math.abs(left - right)
        }

    println("Total Distance: $totalDistance")
}