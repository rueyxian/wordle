// ========================
// ===============================================
// ===============================================================================================

pub mod cli;

pub mod word_pool;

pub mod permutation;

// pub mod search;

pub mod pattern;

pub mod entropy;

pub mod universe;

// ===============================================

#[cfg(test)]

mod tests {
    use crate::universe;

    #[test]
    fn ranking() {
        let words = vec![
            "okays", "loons", "howdy", "likes", "frump", "lures", "hiked", "laird", "sinew",
            "icily", "ewers", "obeys", "dirty", "pesky", "agony", "keels", "apart", "gluey",
            "resin", "skunk", "trait", "tests", "picks", "smirk", "stray", "begat", "fools",
            "corps", "mange", "odour", "ruins", "slurs", "reins", "inked", "rucks", "voile",
            "trips", "honor", "pukka", "yuppy", "bided", "beard", "inset", "alone", "racer",
            "bluff", "cleat", "swats", "ashen", "brats", "sized", "sours", "greed", "flubs",
            "speck", "nooky", "climb", "chows", "shoos", "wrote", "dicey", "equal", "torts",
            "their", "stood", "posse", "gusto", "added", "reedy", "bulgy", "evens", "savvy",
            "laxly", "colds", "crest", "knees", "mutts", "desks", "amity", "mazes", "pubic",
            "gawps", "bijou", "burst", "rearm", "bhaji", "betel", "audio", "blond", "draws",
            "yards", "johns", "civet", "reams", "excel", "yawns", "hokum", "posed", "hands",
            "julep", "puffs", "ledge", "stiff", "width", "quail", "avail", "irked", "pates",
            "ailed", "doffs", "fared", "faced", "nasty", "harps", "slobs", "salts", "start",
            "omits", "karma", "stirs", "boxes", "cacti", "quell", "moose", "tubby", "kilts",
            "cater", "lovey", "loose", "sorry", "surge", "dizzy", "jammy", "soppy", "slink",
            "write", "bonks", "asset", "brave", "capon", "medic", "prism", "quick", "tires",
            "fists", "velar", "scour", "upped", "abler", "raids", "rough", "scaly", "shale",
            "human", "chuck", "lowly", "lurks", "azure", "elder", "fatso", "first", "opals",
            "sofas", "idiom", "sieve", "taxes", "gulps", "crazy", "never", "abode", "bidet",
            "hedge", "views", "winos", "snail", "mocha", "woman", "yarns", "group", "hawed",
            "stark", "buoys", "colts", "mania", "shove", "recto", "irons", "axing", "hallo",
            "wacko", "lathe", "safes", "fazes", "nines", "there", "cable", "honky", "wafts",
            "touts", "sends", "shoes", "cedes", "glitz", "quiff", "ruble", "booby", "cedar",
            "clans", "flyby", "lulls", "spear", "sweet", "tramp", "crags", "krill", "lover",
            "purse", "pouts", "imbed", "muses", "shiny", "sifts", "lobed", "clams", "colic",
            "tonal", "break", "canny", "games", "chore", "livid", "ovary", "tails", "scout",
            "caves", "north", "pause", "shone", "teats", "amass", "egret", "train", "exalt",
            "baits", "stein", "zippy", "flame", "odors", "sicko", "booze", "feuds", "scoot",
            "biffs", "dingo", "fluff", "hefty", "later", "madly", "scrub", "foams", "sniff",
            "midst", "these", "baccy", "darts", "enjoy", "moved", "glass", "panel", "scrum",
            "inter", "whale", "catch", "micks", "polls", "drawn", "impel", "forge", "frail",
            "mould", "huffy", "yogic", "meres", "pivot", "byway", "broth", "oiled", "allot",
            "adorn", "focal", "clods", "necks", "swizz", "hubby", "chest", "oozes", "shelf",
            "mammy", "rerun", "pizza", "surfs", "weeps", "coded", "pages", "dhows", "dries",
            "pokey", "beads", "pixel", "ranee", "gales", "manor", "scrap", "spars", "toads",
            "venom", "gizmo", "panic", "remix", "smogs", "humid",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

        // let wv = WordPool::new();
        // let words = wv.read_pool().unwrap();


        let universe = universe::Universe::new(5, words);
        let v = universe.entropy_ranking(1000);
    println!("");

        println!("{:?}", v);
    }
}
