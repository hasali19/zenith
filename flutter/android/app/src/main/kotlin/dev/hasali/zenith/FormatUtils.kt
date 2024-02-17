package dev.hasali.zenith

fun Long.formatAsSize(): String {
    if (this < 1024) return "$this B"
    val z = (63 - this.countLeadingZeroBits()) / 10
    return String.format("%.1f %sB", this.toDouble() / (1L shl z * 10), " KMGTPE"[z])
}
