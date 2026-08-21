// ---------------------------------------------------------------------------
// Special tokens
// ---------------------------------------------------------------------------

pub const CLS_TOKEN: &str = "[CLS]";
pub const SEP_TOKEN: &str = "[SEP]";
pub const BOS_TOKEN: &str = "[BOS]";
pub const MASK_TOKEN: &str = "[MASK]";
pub const PAD_TOKEN: &str = "[PAD]";
pub const RESERVED_TOKEN: &str = "[RESERVED]";
pub const UNK_TOKEN: &str = "[UNK]";

pub const CLS_ID: i32 = 0;
pub const SEP_ID: i32 = 1;
pub const BOS_ID: i32 = 2;
pub const MASK_ID: i32 = 3;
pub const PAD_ID: i32 = 4;
pub const RESERVED_ID: i32 = 5;
pub const UNK_ID: i32 = 6;

/// First id available for non-special tokens. Codon/amino-acid ids below
/// are already final (baked into each match arm), so nothing needs to add
/// this offset at lookup time — it exists purely for documentation and for
/// building user-facing vocab dumps.
pub const SPECIAL_OFFSET: i32 = 7;

// ---------------------------------------------------------------------------
// Codon / amino-acid tables (kmer_size == 3 only)
//
// All patterns are lowercase ASCII — callers must lowercase the kmer before
// calling these (see `lowercase_kmer` in tokenizer.rs), so this file doesn't
// need to handle capitalization itself. Both DNA (t) and RNA (u) spellings
// are accepted. Returned values are final ids — start/stop codons
// intentionally reuse the [BOS]/[SEP] ids (2 and 1) per the vocab spec.
// ---------------------------------------------------------------------------

/// Maps a lowercase codon to its **amino acid** id (or stop/start).
/// Synonymous codons map to the same value (many-to-one), so this cannot
/// be reversed back to a codon — only to an amino-acid letter via `int_to_aa`.
#[inline]
pub const fn as_aa_to_int(key: &[u8]) -> Option<i32> {
    match key {
        b"[CLS]" =>     Some(0),
        b"[SEP]"   => Some(1),
        b"[BOS]" => Some(2),
        b"[MASK]" => Some(3),
        b"[PAD]" => Some(4),
        b"[RESERVED]" => Some(5),
        b"[UNK]" => Some(6),
        b"gct" | b"gcu" | b"gcc" | b"gca" | b"gcg" => Some(7),  // Ala
        b"tgt" | b"ugu" | b"tgc" | b"ugc" => Some(8),           // Cys
        b"gat" | b"gau" | b"gac" => Some(9),                    // Asp
        b"gaa" | b"gag" => Some(10),                            // Glu
        b"ttt" | b"uuu" | b"ttc" | b"uuc" => Some(11),          // Phe
        b"ggt" | b"ggu" | b"ggc" | b"gga" | b"ggg" => Some(12), // Gly
        b"cat" | b"cau" | b"cac" => Some(13),                   // His
        b"att" | b"auu" | b"atc" | b"auc" | b"ata" | b"aua" => Some(14), // Ile
        b"aaa" | b"aag" => Some(15),                            // Lys
        b"tta" | b"uua" | b"ttg" | b"uug"
        | b"ctt" | b"cuu" | b"ctc" | b"cuc"
        | b"cta" | b"cua" | b"ctg" | b"cug" => Some(16),        // Leu
        b"atg" | b"aug" => Some(2),                             // Met / start (== [BOS])
        b"aat" | b"aau" | b"aac" => Some(17),                   // Asn
        b"cct" | b"ccu" | b"ccc" | b"cca" | b"ccg" => Some(18), // Pro
        b"caa" | b"cag" => Some(19),                            // Gln
        b"cgt" | b"cgu" | b"cgc" | b"cga" | b"cgg"
        | b"aga" | b"agg" => Some(20),                          // Arg
        b"tct" | b"ucu" | b"tcc" | b"ucc" | b"tca" | b"uca"
        | b"tcg" | b"ucg" | b"agt" | b"agu" | b"agc" => Some(21), // Ser
        b"act" | b"acu" | b"acc" | b"aca" | b"acg" => Some(22), // Thr
        b"gtt" | b"guu" | b"gtc" | b"guc"
        | b"gta" | b"gua" | b"gtg" | b"gug" => Some(23),        // Val
        b"tgg" | b"ugg" => Some(24),                            // Trp
        b"tat" | b"uau" | b"tac" | b"uac" => Some(25),          // Tyr
        b"taa" | b"uaa" | b"tag" | b"uag" | b"tga" | b"uga" => Some(1), // Stop (== [SEP])
        _ => None,
    }
}

/// Maps a lowercase codon to a unique id **per codon** (not per amino acid).
/// DNA and RNA spellings of the same codon map to the same id.
/// Values run 7..=70; reversible via `int_to_codon`.
#[inline]
pub const fn codon_to_int(key: &[u8]) -> Option<i32> {
    match key {
        b"[CLS]" =>     Some(0),
        b"[SEP]"   => Some(1),
        b"[BOS]" => Some(2),
        b"[MASK]" => Some(3),
        b"[PAD]" => Some(4),
        b"[RESERVED]" => Some(5),
        b"[UNK]" => Some(6),
        b"aaa" => Some(7),
        b"aac" => Some(8),
        b"aag" => Some(9),
        b"aat" | b"aau" => Some(10),
        b"aca" => Some(11),
        b"acc" => Some(12),
        b"acg" => Some(13),
        b"act" | b"acu" => Some(14),
        b"aga" => Some(15),
        b"agc" => Some(16),
        b"agg" => Some(17),
        b"agt" | b"agu" => Some(18),
        b"ata" | b"aua" => Some(19),
        b"atc" | b"auc" => Some(20),
        b"atg" | b"aug" => Some(21),
        b"att" | b"auu" => Some(22),
        b"caa" => Some(23),
        b"cac" => Some(24),
        b"cag" => Some(25),
        b"cat" | b"cau" => Some(26),
        b"cca" => Some(27),
        b"ccc" => Some(28),
        b"ccg" => Some(29),
        b"cct" | b"ccu" => Some(30),
        b"cga" => Some(31),
        b"cgc" => Some(32),
        b"cgg" => Some(33),
        b"cgt" | b"cgu" => Some(34),
        b"cta" | b"cua" => Some(35),
        b"ctc" | b"cuc" => Some(36),
        b"ctg" | b"cug" => Some(37),
        b"ctt" | b"cuu" => Some(38),
        b"gaa" => Some(39),
        b"gac" => Some(40),
        b"gag" => Some(41),
        b"gat" | b"gau" => Some(42),
        b"gca" => Some(43),
        b"gcc" => Some(44),
        b"gcg" => Some(45),
        b"gct" | b"gcu" => Some(46),
        b"gga" => Some(47),
        b"ggc" => Some(48),
        b"ggg" => Some(49),
        b"ggt" | b"ggu" => Some(50),
        b"gta" | b"gua" => Some(51),
        b"gtc" | b"guc" => Some(52),
        b"gtg" | b"gug" => Some(53),
        b"gtt" | b"guu" => Some(54),
        b"taa" | b"uaa" => Some(55),
        b"tac" | b"uac" => Some(56),
        b"tag" | b"uag" => Some(57),
        b"tat" | b"uau" => Some(58),
        b"tca" | b"uca" => Some(59),
        b"tcc" | b"ucc" => Some(60),
        b"tcg" | b"ucg" => Some(61),
        b"tct" | b"ucu" => Some(62),
        b"tga" | b"uga" => Some(63),
        b"tgc" | b"ugc" => Some(64),
        b"tgg" | b"ugg" => Some(65),
        b"tgt" | b"ugu" => Some(66),
        b"tta" | b"uua" => Some(67),
        b"ttc" | b"uuc" => Some(68),
        b"ttg" | b"uug" => Some(69),
        b"ttt" | b"uuu" => Some(70),
        _ => None,
    }
}

/// Reverse of `codon_to_int`: returns the canonical (DNA, lowercase) codon
/// for an id produced by `codon_to_int`. There is deliberately no reverse
/// for `as_aa_to_int`'s output as a codon (many-to-one) — use `int_to_aa`.
#[inline]
pub const fn int_to_codon(value: i32) -> Option<&'static [u8]> {
    match value {
        0 => Some(b"[CLS]"),
        1 => Some(b"[SEP]"),
        2 => Some(b"[BOS]"),
        3 => Some(b"[MASK]"),
        4 => Some(b"[PAD]"),
        5 => Some(b"[RESERVED]"),
        6 => Some(b"[UNK]"),
        7 => Some(b"aaa"),
        8 => Some(b"aac"),
        9 => Some(b"aag"),
        10 => Some(b"aat"),
        11 => Some(b"aca"),
        12 => Some(b"acc"),
        13 => Some(b"acg"),
        14 => Some(b"act"),
        15 => Some(b"aga"),
        16 => Some(b"agc"),
        17 => Some(b"agg"),
        18 => Some(b"agt"),
        19 => Some(b"ata"),
        20 => Some(b"atc"),
        21 => Some(b"atg"),
        22 => Some(b"att"),
        23 => Some(b"caa"),
        24 => Some(b"cac"),
        25 => Some(b"cag"),
        26 => Some(b"cat"),
        27 => Some(b"cca"),
        28 => Some(b"ccc"),
        29 => Some(b"ccg"),
        30 => Some(b"cct"),
        31 => Some(b"cga"),
        32 => Some(b"cgc"),
        33 => Some(b"cgg"),
        34 => Some(b"cgt"),
        35 => Some(b"cta"),
        36 => Some(b"ctc"),
        37 => Some(b"ctg"),
        38 => Some(b"ctt"),
        39 => Some(b"gaa"),
        40 => Some(b"gac"),
        41 => Some(b"gag"),
        42 => Some(b"gat"),
        43 => Some(b"gca"),
        44 => Some(b"gcc"),
        45 => Some(b"gcg"),
        46 => Some(b"gct"),
        47 => Some(b"gga"),
        48 => Some(b"ggc"),
        49 => Some(b"ggg"),
        50 => Some(b"ggt"),
        51 => Some(b"gta"),
        52 => Some(b"gtc"),
        53 => Some(b"gtg"),
        54 => Some(b"gtt"),
        55 => Some(b"taa"),
        56 => Some(b"tac"),
        57 => Some(b"tag"),
        58 => Some(b"tat"),
        59 => Some(b"tca"),
        60 => Some(b"tcc"),
        61 => Some(b"tcg"),
        62 => Some(b"tct"),
        63 => Some(b"tga"),
        64 => Some(b"tgc"),
        65 => Some(b"tgg"),
        66 => Some(b"tgt"),
        67 => Some(b"tta"),
        68 => Some(b"ttc"),
        69 => Some(b"ttg"),
        70 => Some(b"ttt"),
        _ => None,
    }
}

#[inline]
pub const fn int_to_aa(value: i32) -> Option<&'static [u8]> {
    match value {
        0 => Some(b"[CLS]"),
        1 => Some(b"*"), // also reused for Stop, per your earlier collision design
        2 => Some(b"M"), // also reused for Met/start
        3 => Some(b"[MASK]"),
        4 => Some(b"[PAD]"),
        5 => Some(b"[RESERVED]"),
        6 => Some(b"[UNK]"),
        7 => Some(b"A"),
        8 => Some(b"C"),
        9 => Some(b"D"),
        10 => Some(b"E"),
        11 => Some(b"F"),
        12 => Some(b"G"),
        13 => Some(b"H"),
        14 => Some(b"I"),
        15 => Some(b"K"),
        16 => Some(b"L"),
        17 => Some(b"N"),
        18 => Some(b"P"),
        19 => Some(b"Q"),
        20 => Some(b"R"),
        21 => Some(b"S"),
        22 => Some(b"T"),
        23 => Some(b"V"),
        24 => Some(b"W"),
        25 => Some(b"Y"),
        _ => None,
    }
}