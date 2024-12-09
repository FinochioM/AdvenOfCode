import java.io.File

fun main() {
    val (leftList, rightList) = File("src/list.csv").useLines { lines ->
        lines.map { line ->
            val (left, right) = line.split(";")
            left.toLong() to right.toLong()
        }.unzip()
    }.toList()

    val rightFrequencies = rightList.groupingBy { it }.eachCount()

    val similarityScore = leftList.sumOf { leftNumber ->
        leftNumber * rightFrequencies.getOrDefault(leftNumber, 0)
    }

    println("Similarity Score: $similarityScore")
}