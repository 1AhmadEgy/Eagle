package com.eagle.app.ai.evidence

fun EvidenceRecord.withComputedHash(
    hasher: EvidenceHasher = EvidenceHasher()
): EvidenceRecord =
    copy(
        recordHash = hasher.hashRecord(this),
        signature = null
    )
