import java.io.File

fun main() {
    val (leftList, rightList) = File("src/list.csv").useLines { lines ->
        lines.map { line ->
            val (left, right) = line.split(";")
            left.toLong() to right.toLong()
        }.unzip()
    }.toList()

    val sortedLeft = leftList.sorted()
    val sortedRight = rightList.sorted()

    val totalDistance = sortedLeft.zip(sortedRight)
        .sumOf { (left, right) ->
            kotlin.math.abs(left - right)
        }

    println("Total distance is: $totalDistance")
}