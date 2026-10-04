package com.eagle.app.security

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import java.security.MessageDigest
import java.util.Locale

private enum class ExpectedLabel { NORMAL, ANOMALY, DRIFT }

private data class BenchmarkSample(
    val id: Int,
    val feature: SecurityFeature,
    val history: List<Long>,
    val current: Long,
    val label: ExpectedLabel
)

private data class Confusion(
    val truePositive: Int,
    val trueNegative: Int,
    val falsePositive: Int,
    val falseNegative: Int
) {
    val precision: Double get() = ratio(truePositive, truePositive + falsePositive)
    val recall: Double get() = ratio(truePositive, truePositive + falseNegative)
    val falsePositiveRate: Double get() = ratio(falsePositive, falsePositive + trueNegative)
    val falseNegativeRate: Double get() = ratio(falseNegative, falseNegative + truePositive)

    private fun ratio(numerator: Int, denominator: Int): Double =
        if (denominator == 0) 0.0 else numerator.toDouble() / denominator
}

private data class BenchmarkMethodResult(
    val method: AnomalyMethod,
    val thresholdMilli: Int,
    val confusion: Confusion,
    val driftSensitivity: Double,
    val medianNsPerInference: Long,
    val p95NsPerInference: Long,
    val deterministicDigest: String
)

class StatisticalBaselineBenchmarkTest {

    private val feature = SecurityFeature.REJECTION_RATE_BPS
    private val defaultThreshold = StatisticalBaseline.DEFAULT_THRESHOLD_MILLI

    @Test
    fun benchmarkUsesTheSameDatasetForAllMethods() {
        val dataset = BenchmarkDataset.create(feature)
        val datasetDigest = BenchmarkDataset.digest(dataset)

        val results = AnomalyMethod.entries.map { method ->
            evaluate(method, dataset, defaultThreshold, datasetDigest)
        }

        assertEquals(3, results.size)
        assertTrue(results.all {
            it.confusion.truePositive + it.confusion.falseNegative > 0
        })
        assertTrue(results.all { it.deterministicDigest.isNotBlank() })
    }

    @Test
    fun thresholdSensitivityIsEvaluatedPerMethod() {
        val dataset = BenchmarkDataset.create(feature)
        val datasetDigest = BenchmarkDataset.digest(dataset)
        val thresholds = listOf(1_000, 2_000, 3_000, 4_000, 5_000, 7_500)

        val report = AnomalyMethod.entries.associateWith { method ->
            thresholds.associateWith { threshold ->
                evaluate(method, dataset, threshold, datasetDigest).confusion
            }
        }

        assertEquals(3, report.size)
        assertEquals(thresholds.size * 3, report.values.sumOf { it.size })
    }

    @Test
    fun evaluationIsRepeatable() {
        val dataset = BenchmarkDataset.create(feature)
        val datasetDigest = BenchmarkDataset.digest(dataset)

        val first = AnomalyMethod.entries.associateWith {
            evaluate(it, dataset, defaultThreshold, datasetDigest).deterministicDigest
        }
        val second = AnomalyMethod.entries.associateWith {
            evaluate(it, dataset, defaultThreshold, datasetDigest).deterministicDigest
        }

        assertEquals(first, second)
    }

    @Test
    fun benchmarkReportIsProducedForManualReview() {
        val dataset = BenchmarkDataset.create(feature)
        val datasetDigest = BenchmarkDataset.digest(dataset)
        val results = AnomalyMethod.entries.map {
            evaluate(it, dataset, defaultThreshold, datasetDigest)
        }

        println(StatisticalBenchmarkReport.toJson(datasetDigest, dataset, results))
        assertEquals(300, dataset.size)
    }

    private fun evaluate(
        method: AnomalyMethod,
        dataset: List<BenchmarkSample>,
        thresholdMilli: Int,
        datasetDigest: String
    ): BenchmarkMethodResult {
        val observations = dataset.map { sample ->
            sample to run(method, sample.history, sample.current, thresholdMilli)
        }

        val confusion = observations.fold(Confusion(0, 0, 0, 0)) { acc, pair ->
            val expected = pair.first.label != ExpectedLabel.NORMAL
            val actual = pair.second.anomalous
            when {
                expected && actual -> acc.copy(truePositive = acc.truePositive + 1)
                !expected && !actual -> acc.copy(trueNegative = acc.trueNegative + 1)
                !expected && actual -> acc.copy(falsePositive = acc.falsePositive + 1)
                else -> acc.copy(falseNegative = acc.falseNegative + 1)
            }
        }

        val drift = observations.count { it.first.label == ExpectedLabel.DRIFT }
        val detectedDrift = observations.count {
            it.first.label == ExpectedLabel.DRIFT && it.second.anomalous
        }
        val driftSensitivity = if (drift == 0) 0.0 else detectedDrift.toDouble() / drift

        val deterministicPayload = observations.joinToString(",") {
            it.second.method.name + ":" + it.second.feature.name + ":" +
                it.second.scoreSemantics.name + ":" + it.second.scoreMilli + ":" + it.second.anomalous
        }
        val deterministicDigest = sha256(
            (datasetDigest + "|" + method.name + "|" + thresholdMilli + "|" + deterministicPayload)
                .toByteArray()
        )

        val timings = LongArray(25)
        repeat(5) {
            dataset.forEach { sample ->
                run(method, sample.history, sample.current, thresholdMilli)
            }
        }

        for (i in timings.indices) {
            val start = System.nanoTime()
            repeat(5) {
                dataset.forEach { sample ->
                    run(method, sample.history, sample.current, thresholdMilli)
                }
            }
            val end = System.nanoTime()
            timings[i] = (end - start) / (dataset.size * 5L)
        }
        timings.sort()

        return BenchmarkMethodResult(
            method = method,
            thresholdMilli = thresholdMilli,
            confusion = confusion,
            driftSensitivity = driftSensitivity,
            medianNsPerInference = timings[timings.size / 2],
            p95NsPerInference = timings[(timings.size * 95 / 100).coerceAtMost(timings.lastIndex)],
            deterministicDigest = deterministicDigest
        )
    }

    private fun run(
        method: AnomalyMethod,
        history: List<Long>,
        current: Long,
        thresholdMilli: Int
    ): AnomalySignal = when (method) {
        AnomalyMethod.Z_SCORE ->
            StatisticalBaseline.zScore(feature, history, current, thresholdMilli)
        AnomalyMethod.EWMA ->
            StatisticalBaseline.ewma(feature, history, current, thresholdMilli)
        AnomalyMethod.MEDIAN_MAD ->
            StatisticalBaseline.medianMad(feature, history, current, thresholdMilli)
    }

    private fun sha256(bytes: ByteArray): String =
        MessageDigest.getInstance("SHA-256")
            .digest(bytes)
            .joinToString("") { "%02x".format(it) }
}

private object BenchmarkDataset {

    fun create(feature: SecurityFeature): List<BenchmarkSample> {
        require(feature == SecurityFeature.REJECTION_RATE_BPS)

        return (0 until 300).map { i ->
            val history = (0 until 32).map { j ->
                (1_000L + (((i * 37 + j * 17) % 81) - 40)).coerceAtLeast(0L)
            }

            when {
                i % 10 == 0 ->
                    BenchmarkSample(i, feature, history, 5_000L + (i % 300), ExpectedLabel.ANOMALY)
                i % 10 == 5 ->
                    BenchmarkSample(i, feature, history, 1_450L + (i % 25), ExpectedLabel.DRIFT)
                else ->
                    BenchmarkSample(
                        i,
                        feature,
                        history,
                        1_000L + (((i * 29) % 61) - 30),
                        ExpectedLabel.NORMAL
                    )
            }
        }
    }

    fun digest(dataset: List<BenchmarkSample>): String {
        val canonical = dataset.joinToString(System.lineSeparator()) { sample ->
            listOf(
                sample.id,
                sample.feature.name,
                sample.history.joinToString(","),
                sample.current,
                sample.label.name
            ).joinToString("|")
        }
        return MessageDigest.getInstance("SHA-256")
            .digest(canonical.toByteArray())
            .joinToString("") { "%02x".format(it) }
    }
}

private object StatisticalBenchmarkReport {

    fun toJson(
        datasetDigest: String,
        dataset: List<BenchmarkSample>,
        results: List<BenchmarkMethodResult>
    ): String {
        val out = StringBuilder()
        out.append("{")
            .append("\"datasetSize\":").append(dataset.size)
            .append(",\"datasetDigest\":\"").append(datasetDigest).append("\"")
            .append(",\"feature\":\"").append(dataset.first().feature.name).append("\"")
            .append(",\"thresholdNote\":\"Threshold values are method-specific score units; equal numeric thresholds are not statistically equivalent.\"")
            .append(",\"methods\":[")

        results.forEachIndexed { index, result ->
            if (index > 0) out.append(",")
            out.append("{")
                .append("\"method\":\"").append(result.method.name).append("\"")
                .append(",\"thresholdMilli\":").append(result.thresholdMilli)
                .append(",\"precision\":").append(format(result.confusion.precision))
                .append(",\"recall\":").append(format(result.confusion.recall))
                .append(",\"falsePositiveRate\":").append(format(result.confusion.falsePositiveRate))
                .append(",\"falseNegativeRate\":").append(format(result.confusion.falseNegativeRate))
                .append(",\"driftSensitivity\":").append(format(result.driftSensitivity))
                .append(",\"medianNsPerInference\":").append(result.medianNsPerInference)
                .append(",\"p95NsPerInference\":").append(result.p95NsPerInference)
                .append(",\"deterministicDigest\":\"").append(result.deterministicDigest).append("\"")
                .append("}")
        }

        return out.append("]}").toString()
    }

    private fun format(value: Double): String =
        String.format(Locale.ROOT, "%.6f", value)
}
