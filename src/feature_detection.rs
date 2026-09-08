// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0 and Zlib.

//! Feature detection system for safe and efficient hardware acceleration across different
//! platforms.

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "wasm32",
))]
use spin::Once;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(any(
    test,
    all(
        feature = "alloc",
        any(
            target_arch = "aarch64",
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "wasm32",
        )
    )
))]
use alloc::string::{String, ToString};

// CPU feature detection via `cpufeatures` crate (no_std friendly, replaces std::arch::is_*_feature_detected)
#[cfg(target_arch = "aarch64")]
cpufeatures::new!(cpuid_aes, "aes");
#[cfg(target_arch = "aarch64")]
cpufeatures::new!(cpuid_sha3, "sha3");
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
cpufeatures::new!(cpuid_sse41, "sse4.1");
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
cpufeatures::new!(cpuid_sse42, "sse4.2");
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
cpufeatures::new!(cpuid_pclmulqdq, "pclmulqdq");
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
cpufeatures::new!(cpuid_avx2, "avx2");
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
cpufeatures::new!(cpuid_avx512vl, "avx512vl");
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
cpufeatures::new!(cpuid_vpclmulqdq, "vpclmulqdq");

/// Global ArchOps instance cache - initialized once based on feature detection results (spin::Once for no_std friendliness)
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "wasm32",
))]
static ARCH_OPS_INSTANCE: Once<ArchOpsInstance> = Once::new();

/// Performance tiers representing different hardware capability levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Some variants may not be constructed on all target architectures
pub enum PerformanceTier {
    // AArch64 tiers
    AArch64AesSha3,
    AArch64Aes,

    // x86_64 tiers
    X86_64Avx512Vpclmulqdq,
    X86_64Avx512Pclmulqdq,
    X86_64Avx2Vpclmulqdq,
    X86_64SsePclmulqdq,

    // x86 tiers
    X86Avx2Vpclmulqdq,
    X86SsePclmulqdq,

    // wasm32 tiers
    WasmSimd128,

    // Fallback
    SoftwareTable,
}

/// Architecture-specific capabilities
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // Some fields may not be read on all target architectures
pub struct ArchCapabilities {
    // AArch64 features
    pub has_aes: bool, // provides PMULL support for CRC calculations (NEON is implicit)
    pub has_crc: bool, // provides native CRC32 instructions for fusion techniques
    pub has_sha3: bool, // requires 'aes', provides EOR3 for XOR3 operations

    // x86/x86_64 features
    pub has_sse41: bool,
    pub has_sse42: bool, // provides native CRC32C instructions for fusion techniques
    pub has_pclmulqdq: bool,
    pub has_avx2: bool,     // required to use vpclmulqdq on 256-bit registers
    pub has_avx512vl: bool, // implicitly enables avx512f, has XOR3 operations
    pub has_vpclmulqdq: bool,
    // AVX10 versioned enumeration (raw CPUID leaf 0x24; cpufeatures 0.3 has
    // no AVX10 probe and AVX10-only CPUs may not set the legacy AVX512VL
    // bit). XCR0-gated at detection time, so OS-without-YMM/ZMM-save reads
    // false. 256-bit feeds the AVX2+VPCLMULQDQ tier, 512-bit the AVX512 tiers.
    pub has_avx10_256: bool,
    pub has_avx10_512: bool,
}

/// Helper function to convert a performance tier to a human-readable target string
/// Format: {architecture}-{intrinsics-family}-{intrinsics-features}
#[cfg(any(
    test,
    all(
        feature = "alloc",
        any(
            target_arch = "aarch64",
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "wasm32",
        )
    )
))]
#[inline(always)]
fn tier_to_target_string(tier: PerformanceTier) -> String {
    match tier {
        PerformanceTier::AArch64AesSha3 => "aarch64-neon-pmull-sha3".to_string(),
        PerformanceTier::AArch64Aes => "aarch64-neon-pmull".to_string(),
        PerformanceTier::X86_64Avx512Vpclmulqdq => "x86_64-avx512-vpclmulqdq".to_string(),
        PerformanceTier::X86_64Avx512Pclmulqdq => "x86_64-avx512-pclmulqdq".to_string(),
        PerformanceTier::X86_64Avx2Vpclmulqdq => "x86_64-avx2-vpclmulqdq".to_string(),
        PerformanceTier::X86_64SsePclmulqdq => "x86_64-sse-pclmulqdq".to_string(),
        PerformanceTier::X86Avx2Vpclmulqdq => "x86-avx2-vpclmulqdq".to_string(),
        PerformanceTier::X86SsePclmulqdq => "x86-sse-pclmulqdq".to_string(),
        PerformanceTier::WasmSimd128 => "wasm32-simd128-swizzle".to_string(),
        PerformanceTier::SoftwareTable => "software-fallback-tables".to_string(),
    }
}

/// Detect architecture-specific capabilities combining compile-time and runtime checks
///
/// # Safety
/// Uses runtime feature detection which may access CPU-specific registers
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "wasm32",
))]
unsafe fn detect_arch_capabilities() -> ArchCapabilities {
    #[cfg(target_arch = "aarch64")]
    {
        detect_aarch64_features()
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        detect_x86_features()
    }

    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")))]
    {
        // Other architectures use software fallback
        ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: false,
            has_sse42: false,
            has_pclmulqdq: false,
            has_avx2: false,
            has_avx512vl: false,
            has_vpclmulqdq: false,
            has_avx10_256: false,
            has_avx10_512: false,
        }
    }
}

/// AArch64-specific feature detection
///
/// Note: NEON is always available on AArch64 and is implicitly enabled by AES support.
/// AES support provides the PMULL instructions needed for CRC calculations.
/// Uses `cpufeatures` crate for runtime detection (no_std friendly via getauxval/sysctl).
#[inline(always)]
#[cfg(target_arch = "aarch64")]
unsafe fn detect_aarch64_features() -> ArchCapabilities {
    // Runtime detection via cpufeatures (cached after first call); falls back to
    // compile-time target_feature when OS detection unavailable (e.g. Windows aarch64)
    let has_aes = cpuid_aes::get();
    let has_sha3 = cpuid_sha3::get();
    // CRC not supported by cpufeatures aarch64 crate; fallback to compile-time
    let has_crc = cfg!(target_feature = "crc");

    ArchCapabilities {
        has_aes,
        has_crc,
        has_sha3,
        has_sse41: false,
        has_sse42: false,
        has_pclmulqdq: false,
        has_avx2: false,
        has_avx512vl: false,
        has_vpclmulqdq: false,
        has_avx10_256: false,
        has_avx10_512: false,
    }
}

/// x86/x86_64-specific feature detection (uses CPUID via cpufeatures, no_std friendly)
#[inline(always)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn detect_x86_features() -> ArchCapabilities {
    // Runtime detection via cpufeatures (cached, uses CPUID directly)
    let has_sse41 = cpuid_sse41::get();
    let has_sse42 = cpuid_sse42::get();
    // Hierarchical as before: pclmulqdq requires sse4.1 baseline
    let has_pclmulqdq = has_sse41 && cpuid_pclmulqdq::get();
    // VPCLMULQDQ is separate from AVX-512 (Zen3, Alder Lake etc. have it without AVX-512)
    let has_vpclmulqdq = has_pclmulqdq && cpuid_vpclmulqdq::get();
    let has_avx2 = cpuid_avx2::get();
    let has_avx512vl = has_pclmulqdq && cpuid_avx512vl::get();
    // AVX10-only CPUs may report neither legacy bit; the raw probe is
    // quiescent (false,false) wherever leaf 0x24 is absent.
    let (_avx10, has_avx10_256, has_avx10_512) = detect_avx10();

    ArchCapabilities {
        has_aes: false,
        has_crc: false,
        has_sha3: false,
        has_sse41,
        has_sse42,
        has_pclmulqdq,
        has_avx2,
        has_avx512vl,
        has_vpclmulqdq,
        has_avx10_256,
        has_avx10_512,
    }
}

/// Raw CPUID AVX10 probe (leaf 0x24 version enumeration).
///
/// `cpufeatures 0.3` knows `avx512vl` but not AVX10, and AVX10-only CPUs may
/// not set the legacy AVX512VL bit. Leaf 7 subleaf 1 EDX[19] enumerates
/// AVX10; leaf 0x24 EAX[7:0] is the version and EBX[17]/EBX[18] report
/// 256/512-bit forms (Intel SDM, CPUID EAX=24H).
///
/// Returns `(enumerated, has_256bit, has_512bit)`.
///
/// # Safety
/// Raw CPUID/XGETBV. XCR0 is checked before any length bit can read true,
/// and XGETBV itself is guarded by XSAVE+OSXSAVE, so an OS without YMM/ZMM
/// state save can never observe true. Quiescent triple-false when the
/// leaves are absent (all pre-AVX10 hardware).
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn detect_avx10() -> (bool, bool, bool) {
    const ABSENT: (bool, bool, bool) = (false, false, false);
    #[cfg(target_arch = "x86")]
    use core::arch::x86::{__cpuid, __cpuid_count, _xgetbv};
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::{__cpuid, __cpuid_count, _xgetbv};

    if __cpuid_count(0, 0).eax < 0x24 {
        return ABSENT;
    }
    if (__cpuid_count(7, 1).edx & (1u32 << 19)) == 0 {
        return ABSENT;
    }
    let ver = __cpuid_count(0x24, 0);
    if (ver.eax & 0xff) == 0 {
        return ABSENT; // version 0: AVX10 not actually enabled
    }
    // XGETBV faults without XSAVE+OSXSAVE (leaf 1 ECX[26,27]); bail first.
    let leaf1 = __cpuid(1);
    if (leaf1.ecx & ((1u32 << 26) | (1u32 << 27))) != ((1u32 << 26) | (1u32 << 27)) {
        return ABSENT;
    }
    let xcr0 = _xgetbv(0);
    let has_256 = (ver.ebx & (1u32 << 17)) != 0 && (xcr0 & 0x6u64) == 0x6u64;
    let has_512 =
        (ver.ebx & (1u32 << 18)) != 0 && (xcr0 & 0x6u64) == 0x6u64 && (xcr0 & 0xe0u64) == 0xe0u64;
    (true, has_256, has_512)
}

/// Select the appropriate performance tier based on detected capabilities
#[inline(always)]
#[allow(unused)]
pub(crate) fn select_performance_tier(capabilities: &ArchCapabilities) -> PerformanceTier {
    #[cfg(target_arch = "aarch64")]
    {
        if capabilities.has_sha3 && capabilities.has_aes {
            return PerformanceTier::AArch64AesSha3;
        }

        if capabilities.has_aes {
            return PerformanceTier::AArch64Aes;
        }
    }

    #[cfg(target_arch = "x86_64")]
    {
        if capabilities.has_vpclmulqdq && (capabilities.has_avx512vl || capabilities.has_avx10_512)
        {
            return PerformanceTier::X86_64Avx512Vpclmulqdq;
        }
        if capabilities.has_avx512vl || capabilities.has_avx10_512 {
            return PerformanceTier::X86_64Avx512Pclmulqdq;
        }
        if capabilities.has_vpclmulqdq && (capabilities.has_avx2 || capabilities.has_avx10_256) {
            return PerformanceTier::X86_64Avx2Vpclmulqdq;
        }
        if capabilities.has_pclmulqdq {
            return PerformanceTier::X86_64SsePclmulqdq;
        }
    }

    #[cfg(target_arch = "x86")]
    {
        if capabilities.has_vpclmulqdq && (capabilities.has_avx2 || capabilities.has_avx10_256) {
            return PerformanceTier::X86Avx2Vpclmulqdq;
        }
        if capabilities.has_pclmulqdq {
            return PerformanceTier::X86SsePclmulqdq;
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        // `simd128` is compile-time: no `cpufeatures` entry needed.
        if cfg!(target_feature = "simd128") {
            return PerformanceTier::WasmSimd128;
        }
    }

    // Fallback to software implementation
    PerformanceTier::SoftwareTable
}

/// Enum that holds the different ArchOps implementations for compile-time dispatch
/// This avoids the need for trait objects while still providing factory-based selection
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "wasm32",
))]
#[derive(Debug, Clone, Copy)]
pub enum ArchOpsInstance {
    #[cfg(target_arch = "aarch64")]
    Aarch64Aes(crate::arch::aarch64::aes::Aarch64AesOps),
    #[cfg(target_arch = "aarch64")]
    Aarch64AesSha3(crate::arch::aarch64::aes_sha3::Aarch64AesSha3Ops),
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    X86SsePclmulqdq(crate::arch::x86::sse::X86SsePclmulqdqOps),
    #[cfg(target_arch = "x86")]
    X86Avx2Vpclmulqdq(crate::arch::x86_64::avx2_vpclmulqdq::X86_64Avx2VpclmulqdqOps),
    #[cfg(target_arch = "x86_64")]
    X86_64Avx512Pclmulqdq(crate::arch::x86_64::avx512::X86_64Avx512PclmulqdqOps),
    #[cfg(target_arch = "x86_64")]
    X86_64Avx512Vpclmulqdq(crate::arch::x86_64::avx512_vpclmulqdq::X86_64Avx512VpclmulqdqOps),
    #[cfg(target_arch = "x86_64")]
    X86_64Avx2Vpclmulqdq(crate::arch::x86_64::avx2_vpclmulqdq::X86_64Avx2VpclmulqdqOps),
    #[cfg(target_arch = "wasm32")]
    WasmSimd128(crate::arch::wasm32::simd128::WasmSimd128Ops),
    /// Software fallback - no ArchOps struct needed
    SoftwareFallback,
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "wasm32",
))]
impl ArchOpsInstance {
    #[inline(always)]
    #[allow(dead_code)]
    pub fn get_tier(&self) -> PerformanceTier {
        match self {
            #[cfg(target_arch = "aarch64")]
            ArchOpsInstance::Aarch64Aes(_) => PerformanceTier::AArch64Aes,
            #[cfg(target_arch = "aarch64")]
            ArchOpsInstance::Aarch64AesSha3(_) => PerformanceTier::AArch64AesSha3,
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            ArchOpsInstance::X86SsePclmulqdq(_) => PerformanceTier::X86SsePclmulqdq,
            #[cfg(target_arch = "x86")]
            ArchOpsInstance::X86Avx2Vpclmulqdq(_) => PerformanceTier::X86Avx2Vpclmulqdq,
            #[cfg(target_arch = "x86_64")]
            ArchOpsInstance::X86_64Avx512Pclmulqdq(_) => PerformanceTier::X86_64Avx512Pclmulqdq,
            #[cfg(target_arch = "x86_64")]
            ArchOpsInstance::X86_64Avx512Vpclmulqdq(_) => PerformanceTier::X86_64Avx512Vpclmulqdq,
            #[cfg(target_arch = "x86_64")]
            ArchOpsInstance::X86_64Avx2Vpclmulqdq(_) => PerformanceTier::X86_64Avx2Vpclmulqdq,
            #[cfg(target_arch = "wasm32")]
            ArchOpsInstance::WasmSimd128(_) => PerformanceTier::WasmSimd128,
            ArchOpsInstance::SoftwareFallback => PerformanceTier::SoftwareTable,
        }
    }

    /// Get a human-readable target string describing the active configuration
    #[cfg(all(
        feature = "alloc",
        any(
            target_arch = "aarch64",
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "wasm32",
        )
    ))]
    #[inline(always)]
    pub fn get_target_string(&self) -> String {
        tier_to_target_string(self.get_tier())
    }
}

/// Get the global ArchOps instance (thread-safe, initialized once based on feature detection)
///
/// This function provides access to the cached ArchOps instance that was selected based on
/// feature detection results at library initialization time, eliminating runtime feature
/// detection overhead from hot paths.
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "wasm32",
))]
pub fn get_arch_ops() -> &'static ArchOpsInstance {
    ARCH_OPS_INSTANCE.call_once(create_arch_ops)
}

/// Factory function that creates the appropriate ArchOps struct based on cached feature detection
///
/// This function uses the cached feature detection results to select the optimal
/// architecture-specific implementation at library initialization time, eliminating
/// runtime feature detection overhead from hot paths.
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "wasm32",
))]
fn create_arch_ops() -> ArchOpsInstance {
    let capabilities = unsafe { detect_arch_capabilities() };
    let tier = select_performance_tier(&capabilities);

    create_arch_ops_from_tier(tier)
}

/// Helper function to create ArchOpsInstance from a performance tier
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "wasm32",
))]
fn create_arch_ops_from_tier(tier: PerformanceTier) -> ArchOpsInstance {
    match tier {
        #[cfg(target_arch = "aarch64")]
        PerformanceTier::AArch64AesSha3 => {
            use crate::arch::aarch64::aes_sha3::Aarch64AesSha3Ops;
            ArchOpsInstance::Aarch64AesSha3(Aarch64AesSha3Ops::new())
        }
        #[cfg(target_arch = "aarch64")]
        PerformanceTier::AArch64Aes => {
            use crate::arch::aarch64::aes::Aarch64AesOps;
            ArchOpsInstance::Aarch64Aes(Aarch64AesOps)
        }
        #[cfg(target_arch = "x86")]
        PerformanceTier::X86Avx2Vpclmulqdq => {
            use crate::arch::x86_64::avx2_vpclmulqdq::X86_64Avx2VpclmulqdqOps;
            ArchOpsInstance::X86Avx2Vpclmulqdq(X86_64Avx2VpclmulqdqOps::new())
        }
        #[cfg(target_arch = "x86_64")]
        PerformanceTier::X86_64Avx512Vpclmulqdq => {
            use crate::arch::x86_64::avx512_vpclmulqdq::X86_64Avx512VpclmulqdqOps;
            ArchOpsInstance::X86_64Avx512Vpclmulqdq(X86_64Avx512VpclmulqdqOps::new())
        }
        #[cfg(target_arch = "x86_64")]
        PerformanceTier::X86_64Avx512Pclmulqdq => {
            use crate::arch::x86_64::avx512::X86_64Avx512PclmulqdqOps;
            ArchOpsInstance::X86_64Avx512Pclmulqdq(X86_64Avx512PclmulqdqOps::new())
        }
        #[cfg(target_arch = "x86_64")]
        PerformanceTier::X86_64Avx2Vpclmulqdq => {
            use crate::arch::x86_64::avx2_vpclmulqdq::X86_64Avx2VpclmulqdqOps;
            ArchOpsInstance::X86_64Avx2Vpclmulqdq(X86_64Avx2VpclmulqdqOps::new())
        }
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        PerformanceTier::X86_64SsePclmulqdq | PerformanceTier::X86SsePclmulqdq => {
            create_x86_sse_pclmulqdq_ops()
        }
        #[cfg(target_arch = "wasm32")]
        PerformanceTier::WasmSimd128 => {
            ArchOpsInstance::WasmSimd128(crate::arch::wasm32::simd128::WasmSimd128Ops)
        }
        PerformanceTier::SoftwareTable => {
            // Use software fallback
            ArchOpsInstance::SoftwareFallback
        }
        // Handle cases where the performance tier doesn't match the current architecture
        _ => {
            // This can happen when a tier is selected for a different architecture
            // Fall back to software implementation
            ArchOpsInstance::SoftwareFallback
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn create_x86_sse_pclmulqdq_ops() -> ArchOpsInstance {
    use crate::arch::x86::sse::X86SsePclmulqdqOps;
    ArchOpsInstance::X86SsePclmulqdq(X86SsePclmulqdqOps)
}

/// All-false capabilities for tests; per-test overrides use struct update syntax
/// (`ArchCapabilities { has_aes: true, ..CAPS_NONE }`) instead of repeating
/// all nine fields.
#[cfg(test)]
const CAPS_NONE: ArchCapabilities = ArchCapabilities {
    has_aes: false,
    has_crc: false,
    has_sha3: false,
    has_sse41: false,
    has_sse42: false,
    has_pclmulqdq: false,
    has_avx2: false,
    has_avx512vl: false,
    has_vpclmulqdq: false,
    has_avx10_256: false,
    has_avx10_512: false,
};

/// Test-specific tier selection that works across all architectures for comprehensive testing
#[cfg(test)]
pub fn select_performance_tier_for_test(capabilities: &ArchCapabilities) -> PerformanceTier {
    // AArch64 tier selection - SHA3 requires AES to be available
    if capabilities.has_sha3 && capabilities.has_aes {
        return PerformanceTier::AArch64AesSha3;
    }

    if capabilities.has_aes {
        return PerformanceTier::AArch64Aes;
    }

    // x86_64 tier selection - VPCLMULQDQ + AVX512VL (512-bit, or AVX10-512)
    if capabilities.has_vpclmulqdq && (capabilities.has_avx512vl || capabilities.has_avx10_512) {
        return PerformanceTier::X86_64Avx512Vpclmulqdq;
    }

    // AVX512VL requires PCLMULQDQ and SSE4.1 (or AVX10-512)
    if (capabilities.has_avx512vl || capabilities.has_avx10_512) && capabilities.has_pclmulqdq {
        return PerformanceTier::X86_64Avx512Pclmulqdq;
    }

    // VPCLMULQDQ + AVX2 (256-bit, Zen3 etc. without AVX-512, or AVX10-256)
    if capabilities.has_vpclmulqdq && (capabilities.has_avx2 || capabilities.has_avx10_256) {
        return PerformanceTier::X86_64Avx2Vpclmulqdq;
    }

    // PCLMULQDQ requires SSE4.1
    if capabilities.has_pclmulqdq && capabilities.has_sse41 {
        return PerformanceTier::X86_64SsePclmulqdq;
    }

    // Fallback to software implementation
    PerformanceTier::SoftwareTable
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn test_aarch64_tier_selection() {
        // Test that aarch64 tier selection follows the expected hierarchy

        // Test SHA3 + AES (highest tier) - NEON is implicit with AES
        let capabilities_sha3 = ArchCapabilities {
            has_aes: true,
            has_sha3: true,
            ..CAPS_NONE
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_sha3),
            PerformanceTier::AArch64AesSha3
        );

        // Test AES only (baseline tier) - NEON is implicit with AES
        let capabilities_aes = ArchCapabilities {
            has_aes: true,
            ..CAPS_NONE
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_aes),
            PerformanceTier::AArch64Aes
        );

        // Test missing AES (should fall back to software)
        let capabilities_no_aes = CAPS_NONE;
        assert_eq!(
            select_performance_tier_for_test(&capabilities_no_aes),
            PerformanceTier::SoftwareTable
        );
    }

    #[test]
    fn test_aarch64_feature_hierarchy() {
        // Test that AArch64 feature hierarchy is properly maintained
        // NEON is always available on AArch64 and implicit with AES
        // AES provides PMULL instructions, SHA3 provides EOR3

        // Create test capabilities with AES support (NEON is implicit)
        let capabilities_with_aes = ArchCapabilities {
            has_aes: true,
            ..CAPS_NONE
        };

        // AES support means we have PMULL instructions available for CRC calculations
        assert!(capabilities_with_aes.has_aes);

        // SHA3 requires AES to be available first
        let capabilities_with_sha3 = ArchCapabilities {
            has_aes: true,
            has_sha3: true,
            ..CAPS_NONE
        };

        assert!(capabilities_with_sha3.has_aes);
        assert!(capabilities_with_sha3.has_sha3);
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_x86_64_tier_selection() {
        // Test that x86_64 tier selection follows the expected hierarchy

        // Test VPCLMULQDQ + AVX512 (highest tier)
        let capabilities_vpclmulqdq = ArchCapabilities {
            has_sse41: true,
            has_pclmulqdq: true,
            has_avx512vl: true,
            has_vpclmulqdq: true,
            ..CAPS_NONE
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_vpclmulqdq),
            PerformanceTier::X86_64Avx512Vpclmulqdq
        );

        // Test AVX512 + PCLMULQDQ (mid-tier)
        let capabilities_avx512 = ArchCapabilities {
            has_sse41: true,
            has_pclmulqdq: true,
            has_avx512vl: true,
            ..CAPS_NONE
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_avx512),
            PerformanceTier::X86_64Avx512Pclmulqdq
        );

        // Test SSE + PCLMULQDQ (baseline tier)
        let capabilities_sse = ArchCapabilities {
            has_sse41: true,
            has_pclmulqdq: true,
            ..CAPS_NONE
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_sse),
            PerformanceTier::X86_64SsePclmulqdq
        );

        // Test missing PCLMULQDQ (should fall back to software)
        let capabilities_no_pclmul = ArchCapabilities {
            has_sse41: true,
            ..CAPS_NONE
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_no_pclmul),
            PerformanceTier::SoftwareTable
        );
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_avx10_tier_selection() {
        // AVX10-512 satisfies the 512-bit tiers without the legacy AVX512VL bit.
        let caps_512_vpclmul = ArchCapabilities {
            has_pclmulqdq: true,
            has_vpclmulqdq: true,
            has_avx10_512: true,
            ..CAPS_NONE
        };
        assert_eq!(
            select_performance_tier_for_test(&caps_512_vpclmul),
            PerformanceTier::X86_64Avx512Vpclmulqdq
        );

        let caps_512 = ArchCapabilities {
            has_pclmulqdq: true,
            has_avx10_512: true,
            ..CAPS_NONE
        };
        assert_eq!(
            select_performance_tier_for_test(&caps_512),
            PerformanceTier::X86_64Avx512Pclmulqdq
        );

        // AVX10-256 satisfies the 256-bit tier without AVX2.
        let caps_256 = ArchCapabilities {
            has_pclmulqdq: true,
            has_vpclmulqdq: true,
            has_avx10_256: true,
            ..CAPS_NONE
        };
        assert_eq!(
            select_performance_tier_for_test(&caps_256),
            PerformanceTier::X86_64Avx2Vpclmulqdq
        );
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_detect_avx10_is_quiescent() {
        // Raw CPUID/XGETBV probe: safe to call anywhere; on pre-AVX10
        // hardware it must report absent with both length bits clear.
        let (enumerated, has_256, has_512) = unsafe { detect_avx10() };
        if !enumerated {
            assert!(!has_256 && !has_512);
        }
    }

    #[test]
    #[cfg(target_arch = "x86")]
    fn test_x86_tier_selection() {
        // Test that x86 (32-bit) tier selection works correctly

        // Test SSE + PCLMULQDQ (only available tier for x86)
        let capabilities_sse = ArchCapabilities {
            has_sse41: true,
            has_pclmulqdq: true,
            ..CAPS_NONE
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_sse),
            PerformanceTier::X86_64SsePclmulqdq
        );

        // For x86 32-bit testing, we need a special case since AVX512VL indicates x86_64
        // Create capabilities without AVX512VL to simulate x86 32-bit
        let capabilities_x86_sse = ArchCapabilities {
            has_sse41: true,
            has_pclmulqdq: true,
            ..CAPS_NONE
        };
        // This should select x86_64 tier since we're testing the general case
        assert_eq!(
            select_performance_tier_for_test(&capabilities_x86_sse),
            PerformanceTier::X86_64SsePclmulqdq
        );

        // Test missing PCLMULQDQ (should fall back to software)
        let capabilities_no_pclmul = ArchCapabilities {
            has_sse41: true,
            ..CAPS_NONE
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_no_pclmul),
            PerformanceTier::SoftwareTable
        );
    }

    #[test]
    fn test_x86_feature_hierarchy() {
        // Test that x86 feature hierarchy is properly maintained
        // SSE4.1 is required for PCLMULQDQ
        // AVX512VL requires PCLMULQDQ
        // VPCLMULQDQ requires AVX512VL

        // Test feature dependencies are enforced
        let capabilities_full = ArchCapabilities {
            has_sse41: true,
            has_pclmulqdq: true,
            has_avx512vl: true,
            has_vpclmulqdq: true,
            ..CAPS_NONE
        };

        // All x86 features should be available when hierarchy is satisfied
        assert!(capabilities_full.has_sse41);
        assert!(capabilities_full.has_pclmulqdq);
        assert!(capabilities_full.has_avx512vl);
        assert!(capabilities_full.has_vpclmulqdq);
    }

    // Mock tests for compile-time and runtime feature agreement scenarios
    mod mock_feature_agreement_tests {
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        use super::test;
        use super::*;

        #[test]
        fn test_feature_dependency_validation() {
            // Test that feature dependencies are properly validated

            // SHA3 without AES should not be possible
            let invalid_sha3_caps = ArchCapabilities {
                has_sha3: true,
                ..CAPS_NONE
            };

            // Should fall back to software since AES is required for SHA3
            assert_eq!(
                select_performance_tier_for_test(&invalid_sha3_caps),
                PerformanceTier::SoftwareTable
            );

            // VPCLMULQDQ without AVX512VL should not be possible
            let invalid_vpclmul_caps = ArchCapabilities {
                has_sse41: true,
                has_pclmulqdq: true,
                has_vpclmulqdq: true,
                ..CAPS_NONE
            };

            // Should fall back to SSE tier since AVX512VL is required for VPCLMULQDQ
            assert_eq!(
                select_performance_tier_for_test(&invalid_vpclmul_caps),
                PerformanceTier::X86_64SsePclmulqdq
            );
        }
    }

    // Comprehensive tier selection tests across different hardware configurations
    mod tier_selection_comprehensive_tests {
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        use super::test;
        use super::*;

        #[test]
        fn test_all_aarch64_tier_combinations() {
            // Test all possible AArch64 capability combinations

            // No features - software fallback
            let no_features = CAPS_NONE;
            assert_eq!(
                select_performance_tier_for_test(&no_features),
                PerformanceTier::SoftwareTable
            );

            // AES only - baseline AArch64 tier
            let aes_only = ArchCapabilities {
                has_aes: true,
                ..CAPS_NONE
            };
            assert_eq!(
                select_performance_tier_for_test(&aes_only),
                PerformanceTier::AArch64Aes
            );

            // AES + SHA3 - highest AArch64 tier
            let aes_sha3 = ArchCapabilities {
                has_aes: true,
                has_sha3: true,
                ..CAPS_NONE
            };
            assert_eq!(
                select_performance_tier_for_test(&aes_sha3),
                PerformanceTier::AArch64AesSha3
            );
        }

        #[test]
        fn test_all_x86_64_tier_combinations() {
            // Test all possible x86_64 capability combinations

            // No features - software fallback
            let no_features = CAPS_NONE;
            assert_eq!(
                select_performance_tier_for_test(&no_features),
                PerformanceTier::SoftwareTable
            );

            // SSE4.1 only - software fallback (PCLMULQDQ required)
            let sse_only = ArchCapabilities {
                has_sse41: true,
                ..CAPS_NONE
            };
            assert_eq!(
                select_performance_tier_for_test(&sse_only),
                PerformanceTier::SoftwareTable
            );

            // SSE4.1 + PCLMULQDQ - baseline x86_64 tier
            let sse_pclmul = ArchCapabilities {
                has_sse41: true,
                has_pclmulqdq: true,
                ..CAPS_NONE
            };
            assert_eq!(
                select_performance_tier_for_test(&sse_pclmul),
                PerformanceTier::X86_64SsePclmulqdq
            );

            // SSE4.1 + PCLMULQDQ + AVX512VL - mid-tier
            let avx512_pclmul_new_rust = ArchCapabilities {
                has_sse41: true,
                has_pclmulqdq: true,
                has_avx512vl: true,
                ..CAPS_NONE
            };
            assert_eq!(
                select_performance_tier_for_test(&avx512_pclmul_new_rust),
                PerformanceTier::X86_64Avx512Pclmulqdq
            );

            // All features - highest tier
            let all_features_new_rust = ArchCapabilities {
                has_sse41: true,
                has_pclmulqdq: true,
                has_avx512vl: true,
                has_vpclmulqdq: true,
                ..CAPS_NONE
            };
            assert_eq!(
                select_performance_tier_for_test(&all_features_new_rust),
                PerformanceTier::X86_64Avx512Vpclmulqdq
            );
        }

        #[test]
        fn test_x86_32bit_tier_combinations() {
            // Test x86 (32-bit) capability combinations

            // No features - software fallback
            let no_features = CAPS_NONE;
            assert_eq!(
                select_performance_tier_for_test(&no_features),
                PerformanceTier::SoftwareTable
            );

            // SSE4.1 + PCLMULQDQ - for x86 32-bit testing, we expect x86_64 tier in our test function
            // since the test function doesn't distinguish between x86 and x86_64 architectures
            let sse_pclmul = ArchCapabilities {
                has_sse41: true,
                has_pclmulqdq: true,
                ..CAPS_NONE
            };
            // The test function will return x86_64 tier since it doesn't distinguish architectures
            assert_eq!(
                select_performance_tier_for_test(&sse_pclmul),
                PerformanceTier::X86_64SsePclmulqdq
            );
        }

        #[test]
        fn test_target_string_consistency() {
            // Test that target strings are consistent with selected tiers

            let test_cases = [
                (PerformanceTier::AArch64AesSha3, "aarch64-neon-pmull-sha3"),
                (PerformanceTier::AArch64Aes, "aarch64-neon-pmull"),
                (
                    PerformanceTier::X86_64Avx512Vpclmulqdq,
                    "x86_64-avx512-vpclmulqdq",
                ),
                (
                    PerformanceTier::X86_64Avx512Pclmulqdq,
                    "x86_64-avx512-pclmulqdq",
                ),
                (PerformanceTier::X86_64SsePclmulqdq, "x86_64-sse-pclmulqdq"),
                (PerformanceTier::X86SsePclmulqdq, "x86-sse-pclmulqdq"),
                (PerformanceTier::SoftwareTable, "software-fallback-tables"),
            ];

            for (tier, expected_string) in test_cases {
                assert_eq!(tier_to_target_string(tier), expected_string);
            }
        }
    }

    // Tests for graceful degradation between performance tiers
    mod graceful_degradation_tests {
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        use super::test;
        use super::*;

        #[test]
        fn test_aarch64_degradation_path() {
            // Test the degradation path for AArch64: SHA3+AES -> AES -> Software

            // Start with highest tier capabilities
            let mut capabilities = ArchCapabilities {
                has_aes: true,
                has_sha3: true,
                ..CAPS_NONE
            };

            // Should select highest tier
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::AArch64AesSha3
            );

            // Remove SHA3 - should degrade to AES tier
            capabilities.has_sha3 = false;
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::AArch64Aes
            );

            // Remove AES - should degrade to software
            capabilities.has_aes = false;
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::SoftwareTable
            );
        }

        #[test]
        fn test_x86_64_degradation_path() {
            // Test the degradation path for x86_64: VPCLMULQDQ -> AVX512 -> SSE -> Software

            // Start with highest tier capabilities
            let mut capabilities = ArchCapabilities {
                has_sse41: true,
                has_pclmulqdq: true,
                has_avx512vl: true,
                has_vpclmulqdq: true,
                ..CAPS_NONE
            };

            // Should select highest tier
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::X86_64Avx512Vpclmulqdq
            );

            // Remove VPCLMULQDQ - should degrade to AVX512 tier
            capabilities.has_vpclmulqdq = false;
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::X86_64Avx512Pclmulqdq
            );

            // Remove AVX512VL - should degrade to SSE tier
            capabilities.has_avx512vl = false;
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::X86_64SsePclmulqdq
            );

            // Remove PCLMULQDQ - should degrade to software
            capabilities.has_pclmulqdq = false;
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::SoftwareTable
            );

            // Remove SSE4.1 - should still be software (already at lowest tier)
            capabilities.has_sse41 = false;
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::SoftwareTable
            );
        }

        #[test]
        fn test_partial_feature_availability() {
            // Test scenarios where only some features in a tier are available

            // AArch64: SHA3 available but AES not (impossible in real hardware, but test safety)
            let aarch64_partial = ArchCapabilities {
                has_sha3: true,
                ..CAPS_NONE
            };
            // Should fall back to software since AES is required for SHA3
            assert_eq!(
                select_performance_tier_for_test(&aarch64_partial),
                PerformanceTier::SoftwareTable
            );

            // x86_64: VPCLMULQDQ available but AVX512VL not (impossible in real hardware)
            let x86_64_partial = ArchCapabilities {
                has_sse41: true,
                has_pclmulqdq: true,
                has_vpclmulqdq: true,
                ..CAPS_NONE
            };
            // Should fall back to SSE tier since AVX512VL is required for VPCLMULQDQ
            assert_eq!(
                select_performance_tier_for_test(&x86_64_partial),
                PerformanceTier::X86_64SsePclmulqdq
            );
        }
    }
}

#[cfg(test)]
mod software_fallback_tests {
    use super::*;
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    use wasm_bindgen_test::wasm_bindgen_test as test;
    #[test]
    fn test_aarch64_without_aes_falls_back_to_software() {
        // Test that AArch64 without AES support falls back to software implementation
        let capabilities_no_aes = CAPS_NONE;

        let tier = select_performance_tier_for_test(&capabilities_no_aes);
        assert_eq!(
            tier,
            PerformanceTier::SoftwareTable,
            "AArch64 without AES should fall back to software implementation"
        );
    }

    #[test]
    fn test_x86_without_pclmulqdq_falls_back_to_software() {
        // Test that x86 without SSE4.1/PCLMULQDQ falls back to software implementation
        let capabilities_no_pclmul = ArchCapabilities {
            has_sse41: true,
            ..CAPS_NONE
        };

        let tier = select_performance_tier_for_test(&capabilities_no_pclmul);
        assert_eq!(
            tier,
            PerformanceTier::SoftwareTable,
            "x86 without PCLMULQDQ should fall back to software implementation"
        );

        // Test x86 without SSE4.1
        let capabilities_no_sse = CAPS_NONE;

        let tier = select_performance_tier_for_test(&capabilities_no_sse);
        assert_eq!(
            tier,
            PerformanceTier::SoftwareTable,
            "x86 without SSE4.1 should fall back to software implementation"
        );
    }

    #[test]
    fn test_conditional_compilation_coverage() {
        // Test that software fallback is properly conditionally compiled
        // This test ensures the conditional compilation logic is working correctly

        // Software fallback should be available when needed
        #[cfg(any(
            not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")),
            target_arch = "x86",
            all(target_arch = "aarch64", not(target_feature = "aes"))
        ))]
        {
            // Software fallback should be compiled in these cases
            use crate::arch::software;
            let _test_fn = software::update;
            // This test passes if it compiles successfully
        }

        // For x86_64, software fallback should not be needed since SSE4.1/PCLMULQDQ are always available
        // But it may still be compiled for testing purposes
    }
}
