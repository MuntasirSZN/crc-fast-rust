/* crc_fast library C/C++ API - Copyright 2025 Don MacAskill */
/* This header is auto-generated. Do not edit directly. */

#ifndef CRC_FAST_H
#define CRC_FAST_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Error codes for FFI operations
 */
typedef enum CrcFastError {
  /**
   * Operation completed successfully
   */
  Success = 0,
  /**
   * Null pointer was passed where non-null required
   */
  NullPointer = 2,
  /**
   * Invalid key count for CRC parameters
   */
  InvalidKeyCount = 3,
  /**
   * Unsupported CRC width (must be 32 or 64)
   */
  UnsupportedWidth = 4,
  /**
   * Invalid UTF-8 string
   */
  InvalidUtf8 = 5,
  /**
   * File I/O error
   */
  IoError = 6,
  /**
   * Internal string conversion error
   */
  StringConversionError = 7,
} CrcFastError;

/**
 * The supported CRC algorithms
 */
typedef enum CrcFastAlgorithm {
  CrcCustom,
  Crc16Arc,
  Crc16Cdma2000,
  Crc16Cms,
  Crc16Dds110,
  Crc16DectR,
  Crc16DectX,
  Crc16Dnp,
  Crc16En13757,
  Crc16Genibus,
  Crc16Gsm,
  Crc16Ibm3740,
  Crc16IbmSdlc,
  Crc16IsoIec144433A,
  Crc16Kermit,
  Crc16Lj1200,
  Crc16M17,
  Crc16MaximDow,
  Crc16Mcrf4xx,
  Crc16Modbus,
  Crc16Nrsc5,
  Crc16OpensafetyA,
  Crc16OpensafetyB,
  Crc16Profibus,
  Crc16Riello,
  Crc16SpiFujitsu,
  Crc16T10Dif,
  Crc16Teledisk,
  Crc16Tms37157,
  Crc16Umts,
  Crc16Usb,
  Crc16Xmodem,
  Crc32Aixm,
  Crc32Autosar,
  Crc32Base91D,
  Crc32Bzip2,
  Crc32CdRomEdc,
  Crc32Cksum,
  Crc32Custom,
  Crc32Iscsi,
  Crc32IsoHdlc,
  Crc32Jamcrc,
  Crc32Mef,
  Crc32Mpeg2,
  Crc32Xfer,
  Crc64Custom,
  Crc64Ecma182,
  Crc64GoIso,
  Crc64Ms,
  Crc64Nvme,
  Crc64Redis,
  Crc64We,
  Crc64Xz,
} CrcFastAlgorithm;

typedef struct AlignedTableForward AlignedTableForward;

/**
 * Lookup tables for byte reflection and other operations
 */
typedef struct AlignedTableReverse AlignedTableReverse;

/**
 * Represents a CRC Digest, which is used to compute CRC checksums.
 *
 * The `Digest` struct maintains the state of the CRC computation, including
 * the current state, the amount of data processed, the CRC parameters, and
 * the calculator function used to perform the CRC calculation.
 */
typedef struct CrcFastDigest CrcFastDigest;

/**
 * A handle to the Digest object
 */
typedef struct CrcFastDigestHandle {
  struct CrcFastDigest *_0;
} CrcFastDigestHandle;

/**
 * Custom CRC parameters
 */
typedef struct CrcFastParams {
  enum CrcFastAlgorithm algorithm;
  uint8_t width;
  uint64_t poly;
  uint64_t init;
  bool refin;
  bool refout;
  uint64_t xorout;
  uint64_t check;
  uint32_t key_count;
  const uint64_t *keys;
} CrcFastParams;

#define KEYS_8005_FORWARD { 0, 9683583623776698368ull, 9662191525546688512ull, 9684146573730119680ull, 9256867559083343872ull, 9683583623776698368ull, 9257993458990186496ull, 8589672423, 6442778624, 7672444915179061248, 6686438071761633280, 30117822508040192, 17128033807820128256ull, 17984843636927365120ull, 10108047888656367616ull, 13833369205421899776ull, 9663317425453531136ull, 16916364625333714944ull, 17107767609496961024ull, 17678598862266171392ull, 9254615759269658624ull, 17525476474935574528ull, 9690339023217754112ull, }

#define KEYS_8005_REFLECTED { 0, 101570, 118978, 85186, 121858, 101570, 113666, 7784611839, 81923, 48300, 108148, 109568, 105326, 119614, 115810, 49146, 110786, 100014, 109934, 120158, 125954, 121246, 113858, }

#define KEYS_C867_FORWARD { 0, 1473521503080284160, 8324340961240940544, 9477825415801208832ull, 1482810177311735808, 1473521503080284160, 13036513546354032640ull, 7465454773, 7657160704, 8897986963777257472, 16384939869303996416ull, 6435362392535728128, 15545300013776109568ull, 5172947121988435968, 10889140949028438016ull, 7695244388292624384, 13639432946468257792ull, 11463631376494886912ull, 2140617197884538880, 15579077010981388288ull, 8126464052613349376, 12686640150302687232ull, 9046324276503773184, }

#define KEYS_0589_FORWARD { 0, 14046164287815155712ull, 17840165498898087936ull, 6406088994957819904, 11145564652811845632ull, 14046164287815155712ull, 12125660521718349824ull, 4388822335, 4387831808, 5922233509992202240, 3680848270445248512, 15393022051375644672ull, 7478508656225419264, 1480558377498050560, 5595441062031130624, 17780492803835428864ull, 6381600671983992832, 370139594374512640, 15046244880068116480ull, 8994251405812301824, 1595400167995998208, 17586838019858497536ull, 2114440025050447872, }

#define KEYS_3D65_REFLECTED { 0, 5650, 19852, 97672, 112736, 5650, 49896, 4162752057, 85369, 111622, 1024, 50772, 28564, 82106, 43666, 32, 22706, 114966, 38500, 89338, 115800, 52672, 46192, }

#define KEYS_3D65_FORWARD { 0, 10434840336617439232ull, 7161849307425931264, 2557200163416309760, 881298152081063936, 10434840336617439232ull, 3352366972623912960, 5249364030, 5324996608, 13860672278162833408ull, 18014398509481984, 6108569944574656512, 6047208399651733504, 13404119865938149376ull, 10568259475578290176ull, 576460752303423488, 11111506180629856256ull, 15062007478763913216ull, 5535486891991760896, 13723875439481454592ull, 3766979613318709248, 533113605889982464, 2042945380965941248, }

#define KEYS_1021_FORWARD { 0, 16943386223097937920ull, 1216534849343455232, 18018057684179222528ull, 3961760297202483200, 16943386223097937920ull, 12272590459561312256ull, 4583339121, 4565565440, 17647073664874577920ull, 4847843523887628288, 1564719395534536704, 11401425406641831936ull, 9520609612261228544ull, 11248303019311235072ull, 11521615221697282048ull, 8697295305382559744, 13321647697761927168ull, 13647032770839445504ull, 5022639484424945664, 10222045254224183296ull, 2347219830790160384, 4568901821967368192, }

#define KEYS_6F63_FORWARD { 0, 16573528103700135936ull, 17888016244938899456ull, 3008404551083491328, 18021435383899750400ull, 16573528103700135936ull, 5608107435983110144, 6392880844, 6163726336, 5999357653610921984, 11723432779998822400ull, 1504202275541745664, 14608832766259757056ull, 2999678826805460992, 5861716389999411200, 752101137770872832, 15199367267398713344ull, 11772690900923187200ull, 11465320226355150848ull, 376050568885436416, 16052517921808711680ull, 4811533251891953664, 605452674904621056, }

#define KEYS_5935_FORWARD { 0, 940689372167012352, 5864531139766517760, 1334754339561930752, 1152921504606846976, 940689372167012352, 6369778722962145280, 5585094846, 5791612928, 18119670150771769344ull, 15842819064159272960ull, 2669508679123861504, 2305843009213693952, 12671159026583601152ull, 17189958302696472576ull, 5339017358247723008, 4611686018427387904, 470344686083506176, 9523705837005045760ull, 10678034716495446016ull, 9223372036854775808ull, 15049059629835223040ull, 72057594037927936, }

#define KEYS_080B_REFLECTED { 0, 104556, 27544, 5800, 73462, 104556, 94458, 6451962913, 106529, 129106, 110966, 6204, 94204, 5620, 13064, 6308, 52498, 65618, 129706, 81542, 39876, 24378, 114980, }

#define KEYS_755B_FORWARD { 0, 1035546439318503424, 2437291823337570304, 1639028789386149888, 10241185552640507904ull, 1035546439318503424, 4355825264597401600, 6036153305, 6263865344, 12162815218644156416ull, 8458323050155212800, 1116048282657751040, 10124091962328875008ull, 7099080387619454976, 10103544289028997120ull, 479351885338247168, 10627650695664238592ull, 11145283177835134976ull, 13093934441603006464ull, 18387915803577024512ull, 17428649082947108864ull, 4646870390516219904, 6448591716441128960, }

#define KEYS_1DCF_FORWARD { 0, 5684105679694987264, 9539186960724131840ull, 18307695435214487552ull, 2778720970087596032, 5684105679694987264, 1850416496895852544, 4773390162, 4795072512, 16160885787842314240ull, 779685685488517120, 5506776444367273984, 16710887892334936064ull, 1819454249457680384, 2495557143516676096, 12528169738414587904ull, 6905425603642523648, 2938880231835959296, 7027304268558237696, 16966748646164922368ull, 2139772772954406912, 11775505650690293760ull, 3843540806984007680, }

#define KEYS_A097_FORWARD { 0, 5494110070415294464, 14459369553626398720ull, 7946601542495240192, 7819093378045313024, 5494110070415294464, 17392620285928144896ull, 7836795929, 6989217792, 10158431909487575040ull, 4181592254013505536, 18396923002831765504ull, 9740160094095540224ull, 16405487542603874304ull, 4095742386116755456, 15541359364102160384ull, 6620572927211339776, 13105756390624854016ull, 3054847922240749568, 2963931504763207680, 9051390826084564992, 17813143901133864960ull, 13394268241753276416ull, }

#define KEYS_8BB7_FORWARD { 0, 3266798579703873536, 495114484034043904, 11357233835298258944ull, 9004103029997174784, 3266798579703873536, 1398367684298539008, 8428083192, 6638993408, 14892841017760808960ull, 13823236106260316160ull, 2167920270625472512, 8159396624888496128, 17868312996569153536ull, 9270096882988744704ull, 309622474381721600, 16598016426673963008ull, 12472719168002588672ull, 11859948143703490560ull, 7990230163885391872, 16696251193545981952ull, 15910936008523251712ull, 5407415777588412416, }

#define KEYS_1021_REVERSE { 0, 100782, 36368, 90302, 114392, 100782, 70826, 4770502929, 67601, 118366, 116100, 121680, 47346, 2114, 45170, 86002, 105020, 3642, 19834, 23364, 30562, 102920, 11768, }

#define KEYS_814141AB_FORWARD { 0, 11234659793702486016ull, 9633946051081666560ull, 3073737407451889664, 11855904466354044928ull, 11234659793702486016ull, 12821684324499718144ull, 8573124450, 6463504811, 1457931330141552640, 10976117923717840896ull, 8662364070076219392, 12261473851543650304ull, 4312597582699298816, 6926638292498644992, 8426831435112906752, 3837215432553529344, 306818238694555648, 3899897383567228928, 7446682394511278080, 18429906663742898176ull, 942000552668037120, 10637567775150702592ull, }

#define KEYS_F4ACFB13_REFLECTED { 0, 5925539882, 1346538140, 4481522184, 3615209556, 5925539882, 7261742784, 5318229795, 6740142687, 2969956504, 7287652746, 7261815064, 3337663954, 3466239816, 4230269942, 415699496, 5557856608, 6518918094, 1238068328, 3386203510, 579966550, 8212146846, 834940, }

#define KEYS_A833982B_REFLECTED { 0, 8059738262, 7191583120, 2126939230, 2594239422, 8059738262, 3479769074, 2439511351, 7116920875, 8321646152, 1421528332, 6030436794, 2891116264, 1757287536, 6208158802, 7564519932, 4881759640, 6758519052, 6780315642, 3255059812, 6987424308, 4563163366, 5790291878, }

#define KEYS_8001801B_REFLECTED { 0, 7878164738, 1821425920, 8521410976, 1694552064, 7878164738, 8338472962, 6174081023, 7247953923, 6320710398, 5187409358, 2242010744, 2721881280, 1588430160, 8232852946, 5074659490, 8465355010, 2259480768, 5662282178, 8444862850, 3171008512, 7460816928, 4522660098, }

#define KEYS_741B8CD7_REFLECTED { 0, 5553652472, 2068564088, 598770696, 8216373412, 5553652472, 415585868, 399651533, 7891824733, 8419795490, 4983874506, 76376706, 1672763460, 7142372662, 6722411466, 3194851128, 7523178518, 7342418210, 2535825178, 2918863256, 2626261040, 7213322382, 8474282224, }

#define KEYS_000000AF_FORWARD { 0, 11645077973630976, 18084856833792540672ull, 6690215671592124416, 12641383852271468544ull, 11645077973630976, 75131862908928, 4294967471, 4294967471, 11229016885280374784ull, 13232938429618061312ull, 1936548123237154816, 16677405452576751616ull, 11276952985206259712ull, 4072937381085315072, 13004053149898506240ull, 775373662877384704, 6822563924882227200, 8667819838183309312, 9016698073132302336, 18080778032200548352ull, 2732902306218508288, 7866523122801836032, }

/**
 * SIMD constants for reflected 32-bit-space CRC operations
 */
#define WIDTH32_CONSTANTS_REFLECTED { { 579005069656919567, 283686952306183, }, { 9259542123273814144ull, 9259542123273814144ull, }, { 18446744069414584320ull, 18446744073709551615ull, }, { 0, 0, }, }

/**
 * SIMD constants for forward (non-reflected) 32-bit-space CRC operations
 */
#define WIDTH32_CONSTANTS_FORWARD { { 579005069656919567, 283686952306183, }, { 9259542123273814144ull, 9259542123273814144ull, }, { 18446744073709551615ull, 4294967295, }, { 0, 0, }, }

#define SIMD_CONSTANTS { { 579005069656919567, 283686952306183, }, { 9259542123273814144ull, 9259542123273814144ull, }, { 18446744073709551615ull, 4294967295, }, { 0, 18446744073709551615ull, }, }

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Gets the last error that occurred in the current thread
 * Returns CrcFastError::Success if no error has occurred
 */
enum CrcFastError crc_fast_get_last_error(void);

/**
 * Clears the last error for the current thread
 */
void crc_fast_clear_error(void);

/**
 * Gets a human-readable error message for the given error code
 * Returns a pointer to a static string (do not free)
 */
const char *crc_fast_error_message(enum CrcFastError error);

/**
 * Creates a new Digest to compute CRC checksums using algorithm
 */
struct CrcFastDigestHandle *crc_fast_digest_new(enum CrcFastAlgorithm algorithm);

/**
 * Creates a new Digest with a custom initial state
 */
struct CrcFastDigestHandle *crc_fast_digest_new_with_init_state(enum CrcFastAlgorithm algorithm,
                                                                uint64_t init_state);

/**
 * Creates a new Digest to compute CRC checksums using custom parameters
 * Returns NULL if parameters are invalid (invalid key count or null pointer)
 * Call crc_fast_get_last_error() to get the specific error code
 */
struct CrcFastDigestHandle *crc_fast_digest_new_with_params(struct CrcFastParams params);

/**
 * Updates the Digest with data
 */
void crc_fast_digest_update(struct CrcFastDigestHandle *handle, const char *data, uintptr_t len);

/**
 * Calculates the CRC checksum for data that's been written to the Digest
 * Returns 0 on error (e.g. null handle)
 */
uint64_t crc_fast_digest_finalize(struct CrcFastDigestHandle *handle);

/**
 * Free the Digest resources without finalizing
 */
void crc_fast_digest_free(struct CrcFastDigestHandle *handle);

/**
 * Reset the Digest state
 */
void crc_fast_digest_reset(struct CrcFastDigestHandle *handle);

/**
 * Finalize and reset the Digest in one operation
 * Returns 0 on error (e.g. null handle)
 */
uint64_t crc_fast_digest_finalize_reset(struct CrcFastDigestHandle *handle);

/**
 * Combine two Digest checksums
 */
void crc_fast_digest_combine(struct CrcFastDigestHandle *handle1,
                             struct CrcFastDigestHandle *handle2);

/**
 * Gets the amount of data processed by the Digest so far
 * Returns 0 on error (e.g. null handle)
 */
uint64_t crc_fast_digest_get_amount(struct CrcFastDigestHandle *handle);

/**
 * Gets the current state of the Digest
 * Returns 0 on error (e.g. null handle)
 */
uint64_t crc_fast_digest_get_state(struct CrcFastDigestHandle *handle);

/**
 * Helper method to calculate a CRC checksum directly for a string using algorithm
 * Returns 0 on error (e.g. null data pointer)
 */
uint64_t crc_fast_checksum(enum CrcFastAlgorithm algorithm, const char *data, uintptr_t len);

/**
 * Helper method to calculate a CRC checksum directly for data using custom parameters
 * Returns 0 if parameters are invalid or data is null
 * Call crc_fast_get_last_error() to get the specific error code
 */
uint64_t crc_fast_checksum_with_params(struct CrcFastParams params,
                                       const char *data,
                                       uintptr_t len);

/**
 * Helper method to just calculate a CRC checksum directly for a file using algorithm
 * Returns 0 if path is null or file I/O fails
 * Call crc_fast_get_last_error() to get the specific error code
 */
uint64_t crc_fast_checksum_file(enum CrcFastAlgorithm algorithm,
                                const uint8_t *path_ptr,
                                uintptr_t path_len);

/**
 * Helper method to calculate a CRC checksum directly for a file using custom parameters
 * Returns 0 if parameters are invalid, path is null, or file I/O fails
 * Call crc_fast_get_last_error() to get the specific error code
 */
uint64_t crc_fast_checksum_file_with_params(struct CrcFastParams params,
                                            const uint8_t *path_ptr,
                                            uintptr_t path_len);

/**
 * Combine two CRC checksums using algorithm
 */
uint64_t crc_fast_checksum_combine(enum CrcFastAlgorithm algorithm,
                                   uint64_t checksum1,
                                   uint64_t checksum2,
                                   uint64_t checksum2_len);

/**
 * Combine two CRC checksums using custom parameters
 * Returns 0 if parameters are invalid
 * Call crc_fast_get_last_error() to get the specific error code
 */
uint64_t crc_fast_checksum_combine_with_params(struct CrcFastParams params,
                                               uint64_t checksum1,
                                               uint64_t checksum2,
                                               uint64_t checksum2_len);

/**
 * Returns the custom CRC parameters for a given set of Rocksoft CRC parameters
 * If width is not 32 or 64, sets error to UnsupportedWidth
 */
struct CrcFastParams crc_fast_get_custom_params(const char *name_ptr,
                                                uint8_t width,
                                                uint64_t poly,
                                                uint64_t init,
                                                bool reflected,
                                                uint64_t xorout,
                                                uint64_t check);

/**
 * Gets the target build properties (CPU architecture and fine-tuning parameters) for this algorithm
 * Returns NULL if string conversion fails
 * Call crc_fast_get_last_error() to get the specific error code
 */
const char *crc_fast_get_calculator_target(enum CrcFastAlgorithm algorithm);

/**
 * Gets the version of this library
 * Returns a pointer to "unknown" if version string is invalid
 */
const char *crc_fast_get_version(void);

/**
 * Calculates the CRC-32/ISCSI checksum (commonly called "crc32c" in many, but not all,
 * implementations).
 *
 * https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iscsi
 *
 * Returns 0 on error (e.g. null data pointer)
 */
uint32_t crc_fast_crc32_iscsi(const char *data, uintptr_t len);

/**
 * Calculates the CRC-32/ISO-HDLC checksum (commonly called "crc32" in many, but not all,
 * implementations).
 *
 * https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iso-hdlc
 *
 * Returns 0 on error (e.g. null data pointer)
 */
uint32_t crc_fast_crc32_iso_hdlc(const char *data, uintptr_t len);

/**
 * Calculates the CRC-64/NVME checksum.
 *
 * https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-nvme
 *
 * Returns 0 on error (e.g. null data pointer)
 */
uint64_t crc_fast_crc64_nvme(const char *data, uintptr_t len);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* CRC_FAST_H */
