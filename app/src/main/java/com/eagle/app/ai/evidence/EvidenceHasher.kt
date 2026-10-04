package com.eagle.app.ai.evidence

import com.eagle.app.ai.findings.SecurityFinding
import java.io.ByteArrayOutputStream
import java.io.DataOutputStream
import java.nio.charset.StandardCharsets
import java.security.MessageDigest

/**
 * Canonical SHA-256 hasher for EvidenceRecord.
 *
 * The framing is length-delimited and type-aware. recordHash and signature are
 * deliberately excluded from the preimage to avoid circular hashing.
 */
class EvidenceHasher {

    fun hashRecord(record: EvidenceRecord): String {
        val bytes = ByteArrayOutputStream()
        DataOutputStream(bytes).use { out ->
            writeString(out, "EAGLE-EVIDENCE-RECORD-V1")
            writeString(out, record.id)
            writeInstant(out, record.timestamp)
            writeEnum(out, record.taskType)
            writeString(out, record.providerId)
            writeString(out, record.providerVersion)
            writeString(out, record.codeHash)
            writeString(out, record.promptHash)
            writeString(out, record.rawOutput)
            writeNullable(out, record.parsedFinding) { writeFinding(out, it) }
            writeNullable(out, record.reproductionTestId) { writeString(out, it) }
            writeNullable(out, record.sastResultId) { writeString(out, it) }
            writeList(out, record.regressionTestIds)
            writeNullable(out, record.buildId) { writeString(out, it) }
            writeNullable(out, record.humanApproval) { approval ->
                writeString(out, approval.approvedBy)
                writeInstant(out, approval.approvedAt)
                writeEnum(out, approval.decision)
                writeString(out, approval.note)
            }
            writeNullable(out, record.previousHash) { writeString(out, it) }
        }

        return MessageDigest.getInstance("SHA-256")
            .digest(bytes.toByteArray())
            .joinToString("") { "%02x".format(it) }
    }

    private fun writeFinding(out: DataOutputStream, finding: SecurityFinding) {
        writeString(out, finding.id)
        writeEnum(out, finding.taskType)
        writeEnum(out, finding.severity)
        writeEnum(out, finding.status)
        writeString(out, finding.title)
        writeString(out, finding.description)
        writeSortedSet(out, finding.affectedPaths)
        writeInstant(out, finding.createdAt)
        writeString(out, finding.proposerProviderId)
    }

    private fun writeInstant(out: DataOutputStream, instant: java.time.Instant) {
        out.writeLong(instant.epochSecond)
        out.writeInt(instant.nano)
    }

    private fun writeEnum(out: DataOutputStream, value: Enum<*>) {
        writeString(out, value.javaClass.name)
        writeString(out, value.name)
    }

    private fun writeString(out: DataOutputStream, value: String) {
        val bytes = value.toByteArray(StandardCharsets.UTF_8)
        out.writeInt(bytes.size)
        out.write(bytes)
    }

    private fun <T> writeNullable(
        out: DataOutputStream,
        value: T?,
        writer: (T) -> Unit
    ) {
        if (value == null) {
            out.writeByte(0)
        } else {
            out.writeByte(1)
            writer(value)
        }
    }

    private fun writeList(out: DataOutputStream, values: List<String>) {
        out.writeInt(values.size)
        values.forEach { writeString(out, it) }
    }

    private fun writeSortedSet(out: DataOutputStream, values: Set<String>) {
        val sorted = values.toList().sorted()
        out.writeInt(sorted.size)
        sorted.forEach { writeString(out, it) }
    }
}
