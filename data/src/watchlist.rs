#![allow(dead_code)]
use std::collections::HashSet;

// XLF
pub const FINANCE_SECTOR: [&str; 29] = [
    "JPM", "WFC", "C", "GS", "MS", "AXP", "V", "MA", "BLK", "SCHW", "PRU", "MET", "AIG", "COF",
    "TD", "RY", "BMO", "BNS", "CM", "ITUB", "BBD", "LYG", "BCS", "ESQ", "KEY", "ACGL", "ARES",
    "RF", "XLF",
];

// XLK
pub const TECH_SECTOR: [&str; 42] = [
    "AAPL", "MSFT", "GOOG", "META", "AMZN", "TSLA", "NVDA", "INTC", "CRM", "ORCL", "CSCO", "ADBE",
    "IBM", "TXN", "QCOM", "SAP", "SHOP", "ASML", "INTU", "AMD", "MU", "SQ", "ADSK", "TEAM", "WDAY",
    "DELL", "HPQ", "ARRY", "TTMI", "MXL", "NTCT", "PLTR", "SOFI", "PINS", "COIN", "ARLO", "UI",
    "Z", "SE", "SONY", "TTWO", "XLK",
];

// XLE
pub const ENERGY_SECTOR: [&str; 32] = [
    "XOM", "CVX", "COP", "BP", "SLB", "EOG", "OXY", "VLO", "MPC", "PSX", "KMI", "WMB", "BKR",
    "HES", "APA", "FANG", "MRO", "CNQ", "PBR", "YPF", "ENB", "KOS", "MPLX", "BTU", "PBF", "CEG",
    "NRG", "CVI", "PTEN", "WDS", "FSLR", "XLE",
];

// XLU
pub const UTILITY_SECTOR: [&str; 31] = [
    "NEE", "DUK", "D", "SO", "EXC", "AEP", "SRE", "XEL", "PEG", "WEC", "DTE", "ED", "EIX", "AEE",
    "CNP", "CMS", "AWK", "ETR", "EVRG", "AES", "LNT", "NI", "PNW", "OGE", "NWE", "IDA", "POR",
    "AVA", "MGEE", "BEP", "XLU",
];

// XLRE
pub const REAL_ESTATE_SECTOR: [&str; 20] = [
    "AVB", "EQR", "DLR", "VTR", "SBAC", "O", "REG", "EXR", "ARE", "UDR", "ESS", "ELS", "MAA",
    "CPT", "IRM", "RHP", "KRC", "NHI", "OHI", "XLRE",
];

// XLC
pub const COMMUNICATION_SECTOR: [&str; 20] = [
    "DIS", "CMCSA", "VZ", "T", "TMUS", "FOX", "OMC", "TME", "MTCH", "IAC", "NTES", "BIDU", "ROKU",
    "LYV", "IQ", "VIAV", "EA", "SPOT", "NFLX", "XLC",
];

// XME
pub const METALS_MINING_SECTOR: [&str; 31] = [
    "AEM", "WPM", "TECK", "SCCO", "FNV", "KGC", "PAAS", "AG", "IAG", "BTG", "HBM", "CCJ", "AA",
    "X", "CLF", "STLD", "RS", "CMC", "WOR", "SAND", "ARCH", "HL", "SGML", "SMID", "EQT", "SIL",
    "PSLV", "XME", "LIT", "REMX", "SLX",
];

// XLP
pub const CONSUMER_STAPLES_SECTOR: [&str; 30] = [
    "PG", "KO", "PEP", "PM", "WMT", "MO", "COST", "MDLZ", "CL", "KMB", "STZ", "EL", "GIS", "KR",
    "ADM", "MNST", "TSN", "SYY", "DEO", "CPB", "CLX", "HSY", "CAG", "HRL", "MKC", "CHD", "CCEP",
    "SJM", "LW", "XLP",
];

// XLB
pub const MATERIALS_SECTOR: [&str; 17] = [
    "LIN", "APD", "ECL", "PPG", "SHW", "DOW", "LYB", "CTVA", "DD", "AVY", "IP", "CHX", "GPRE",
    "FCX", "NUE", "NEM", "XLB",
];

// XLI
pub const INDUSTRIAL_SECTOR: [&str; 42] = [
    "MMM", "GE", "HON", "UNP", "CAT", "RTX", "DE", "EMR", "ITW", "NSC", "LMT", "GD", "ADP", "CSX",
    "CMI", "NOC", "WM", "ETN", "PCAR", "IR", "ROP", "PH", "TDG", "DOV", "SWK", "TXT", "CTAS",
    "CPRT", "GWW", "RSG", "AVY", "XYL", "ALLE", "MAS", "AME", "WAB", "WFRD", "ACA", "CLH", "FIX",
    "SYM", "XLI",
];

// XLV
pub const HEALTHCARE_SECTOR: [&str; 38] = [
    "JNJ", "PFE", "MRK", "ABBV", "ABT", "MDT", "AMGN", "BMY", "GILD", "TMO", "LLY", "DHR", "SYK",
    "BSX", "AZN", "CVS", "ISRG", "ZTS", "BDX", "CI", "VRTX", "HCA", "ALGN", "EW", "CNC", "REGN",
    "IQV", "HUM", "A", "WAT", "BAX", "BIIB", "HSIC", "RMD", "SAVA", "KROS", "AIRS", "XLV",
];

// XLY
pub const CONSUMER_DISCRETIONARY_SECTOR: [&str; 41] = [
    "HD", "MCD", "NKE", "SBUX", "LOW", "BKNG", "TJX", "GM", "F", "EBAY", "ETSY", "YUM", "ORLY",
    "ROST", "DG", "AZO", "DLTR", "RCL", "CCL", "NCLH", "MGM", "LVS", "WYNN", "BBY", "ULTA", "GRMN",
    "PVH", "RL", "GPS", "TPR", "JWN", "DRVN", "CHDN", "RIVN", "DKNG", "BLBD", "Z", "RVLV", "LI",
    "TM", "XLY",
];

// SMH
pub const SEMICONDUCTOR_SECTOR: [&str; 31] = [
    "INTC", "TSM", "QCOM", "ASML", "TXN", "AMD", "MU", "ADI", "NXPI", "KLAC", "MCHP", "LRCX",
    "MRVL", "AMAT", "SWKS", "SNPS", "CDNS", "MSCI", "TER", "QRVO", "ON", "UMC", "SMTC", "MPWR",
    "ENPH", "SEDG", "IPGP", "COHU", "ASX", "GSIT", "SMH",
];

// XTN
pub const TRANSPORTATION_SECTOR: [&str; 30] = [
    "UPS", "FDX", "DAL", "AAL", "LUV", "UAL", "CHRW", "JBHT", "ODFL", "LSTR", "EXPD", "R", "MATX",
    "SAIA", "ALK", "SAVE", "JBLU", "ZTO", "HUBG", "PATK", "ULH", "ARCB", "DSKE", "PTSI", "RLGT",
    "PANL", "UBER", "CVLG", "GLNG", "XTN",
];

// VWO
pub const EMERGING_MARKETS_SECTOR: [&str; 18] = [
    "BABA", "HDB", "GGAL", "BMA", "BBAR", "VALE", "ITUB", "PBR", "GGB", "SBS", "EBR", "SBS", "ASR",
    "AMX", "BAP", "BVN", "BCH", "VWO",
];

pub const EXCHANGE_TRADED_FUNDS: [&str; 35] = [
    "XLF", "XLRE", "XLC", "XME", "XLP", "XLB", "XLE", "XLI", "XLK", "XLV", "XLY", "XTN", "XLU",
    "SMH", "VWO", "QQQ", "SPY", "VTI", "VEA", "SCHF", "VOO", "VCLT", "VNQ", "VDE", "VTV", "VUG",
    "VHT", "VAW", "VIS", "VFH", "VPU", "VB", "VO", "VYM", "XPO",
];

pub fn get_all_unique_stock_symbols() -> Vec<&'static str> {
    let mut all_stock_symbols: Vec<&str> = Vec::new();

    all_stock_symbols.extend(&FINANCE_SECTOR);
    all_stock_symbols.extend(&TECH_SECTOR);
    all_stock_symbols.extend(&ENERGY_SECTOR);
    all_stock_symbols.extend(&UTILITY_SECTOR);
    all_stock_symbols.extend(&REAL_ESTATE_SECTOR);
    all_stock_symbols.extend(&COMMUNICATION_SECTOR);
    all_stock_symbols.extend(&METALS_MINING_SECTOR);
    all_stock_symbols.extend(&CONSUMER_STAPLES_SECTOR);
    all_stock_symbols.extend(&MATERIALS_SECTOR);
    all_stock_symbols.extend(&INDUSTRIAL_SECTOR);
    all_stock_symbols.extend(&HEALTHCARE_SECTOR);
    all_stock_symbols.extend(&CONSUMER_DISCRETIONARY_SECTOR);
    all_stock_symbols.extend(&SEMICONDUCTOR_SECTOR);
    all_stock_symbols.extend(&TRANSPORTATION_SECTOR);
    all_stock_symbols.extend(&EMERGING_MARKETS_SECTOR);
    all_stock_symbols.extend(&EXCHANGE_TRADED_FUNDS);

    let mut set = HashSet::new();

    for symbol in all_stock_symbols {
        set.insert(symbol);
    }

    set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_unique_stock_symbols() {
        let symbols = get_all_unique_stock_symbols();
        dbg!(symbols);
        assert!(false);
    }
}
