package year2021.day21

import common.*

fun main() = day(2021, 21, part1, part2) {
    val lines = readLines()
    lines[0].last().digitToInt() to lines[1].last().digitToInt()
}

typealias Input = Pair<Int, Int>

val part1 = { (p1position, p2position): Input ->
    val game = Game(listOf(Player(p1position - 1), Player(p2position - 1)), 1000)
    var nbDiceRolled = 0
    val dice = generateSequence(0) { (it + 1) % 100 }.map { it + 1 }.onEach { nbDiceRolled += 1 }.iterator()

    val finalGame = generateSequence(game) {
        it.play(dice.next() + dice.next() + dice.next())
    }.first { it.isOver() }

    finalGame.players.minOf { it.score } * nbDiceRolled
}

val part2 = { (p1position, p2position): Input ->
    val game = Game(listOf(Player(p1position - 1), Player(p2position - 1)), 21)

    val diceSumsProb = (1..3).flatMap { a -> (1..3).flatMap { b -> (1..3).map { c -> a + b + c } } }
        .groupingBy { it }
        .eachCount()

    val quantumGames = mutableMapOf<Game, Long>()
    generateSequence(QuantumGame(game, 1)) {
        quantumGames
            .minByOrNull { it.key.players.minOf { it.score } }
            ?.also { quantumGames.remove(it.key) }
            ?.let { (game, prob) -> QuantumGame(game, prob) }
    }
        .flatMap { currentQg -> diceSumsProb.map { (diceSum, prob) -> currentQg.play(diceSum, prob.toLong()) } }
        .filter { it.game.isOver().also { isOver -> if (!isOver) quantumGames.merge(it.game, it.prob, Long::plus) } }
        .fold(mutableListOf(0L, 0L)) { winprobs, qg ->
            winprobs.also {
                val winner = qg.game.indexOfWinner()
                winprobs[winner] = winprobs[winner] + qg.prob
            }
        }.maxOf { it }
}

data class QuantumGame(val game: Game, val prob: Long) {
    fun play(diceSum: Int, prob: Long) = QuantumGame(game.play(diceSum), this.prob * prob)
}

data class Game(val players: List<Player>, val winScore: Int, val nextPlayerIndex: Int = 0) {
    fun play(diceSum: Int) = Game(
        players.mapIndexed { index, player -> if (index == nextPlayerIndex) player.play(diceSum) else player },
        winScore,
        (nextPlayerIndex + 1) % players.size,
                                 )

    fun isOver() = indexOfWinner() >= 0
    fun indexOfWinner() = players.indexOfFirst { it.score >= winScore }
}


data class Player(val position: Int, val score: Int = 0) {
    fun play(diceSum: Int) = ((position + diceSum) % 10).let { newPosition ->
        Player(newPosition, score + newPosition + 1)
    }
}


