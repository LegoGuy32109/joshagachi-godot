use rand::seq::IndexedRandom;
use rand::{Rng, rng};
use std::fs::{File, canonicalize};
use std::io::Write;

fn main() {
    // generate name for build
    let build_dwarf_name = get_dwarf_name();

    let now = chrono::Local::now();
    let build_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let build_time_nice = now.format("%b%e %-I:%M:%S %P").to_string();
    let build_time_epoch = now.timestamp();
    let build_info_filename = "build_info.json";
    let build_info_path = format!("../../{build_info_filename}");

    let content = format!(
        r#"{{
    "build_time": "{build_time}",
    "build_time_nice": "{build_time_nice}",
    "build_time_epoch": "{build_time_epoch}",
    "build_dwarf_name": "{build_dwarf_name}"
}}"#
    );

    let mut file = File::create(&build_info_path).expect(
        format!("Failed to create file '{build_info_filename}' at '{build_info_path}'").as_str(),
    );

    file.write_all(content.as_bytes())
        .expect(format!("Failed to write '{build_info_filename}'").as_str());

    let abs_path = canonicalize(&build_info_path)
        .expect(format!("failed to determine location of '{build_info_path}'").as_str());

    println!("⛏️ {build_dwarf_name} ⚒️");
    println!(
        "{build_time_nice} ✅ Wrote {build_info_filename} to {}",
        abs_path.display()
    );
    std::process::exit(0);
}

fn get_dwarf_name() -> String {
    let male_name_1 = vec![
        "A", "Ara", "Alfo", "Bari", "Be", "Bo", "Bha", "Bu", "Ba", "Bra", "Bro", "Brou", "Bru",
        "Da", "Dalo", "Dare", "De", "Dhu", "Dho", "Do", "Dora", "Dwo", "Dou", "Duri", "Du", "El",
        "Eri", "Fi", "Fo", "Fo", "Ga", "Gi", "Gla", "Glori", "Go", "Gra", "Gro", "Groo", "Gru",
        "Grou", "Ha", "Ha", "He", "He", "Ho", "Hou", "Hu", "Ja", "Jo", "Ka", "Khe", "Khu", "Khou",
        "Ko", "Ku", "Ki", "Kra", "Kro", "Lo", "Lu", "Lo", "Ma", "Mo", "Mu", "Na", "No", "Nu",
        "Nora", "Nura", "Ne", "No", "O", "Ori", "Rei", "Ra", "Ru", "Sa", "Si", "Sna", "Sko", "Ska",
        "Stro", "The", "Thi", "Tho", "Thra", "Tha", "Tore", "Tha", "Thra", "Thro", "Thu", "Tu",
        "U", "Umi", "Va", "Vo", "Whu", "We", "Wera", "Yu", "Yo", "Ya",
    ];
    let male_name_2 = vec![
        "b", "br", "dd", "d", "dr", "dm", "dgr", "f", "fr", "gr", "gg", "gh", "gn", "k", "kh",
        "kgr", "kdr", "kk", "kh", "kr", "l", "lg", "lgr", "ldr", "lm", "md", "mn", "m", "mm", "mr",
        "n", "nd", "ndr", "ngr", "nm", "r", "rr", "rgr", "rdr", "rb", "rg", "rn", "rh", "rd", "rm",
        "rs", "rf", "s", "ss", "sdr", "sgr", "st", "str", "t", "tr", "tm", "th", "tdr", "tgr", "v",
        "vr", "z", "zm", "zn", "zz",
    ];
    let male_name_3 = vec![
        "ac", "aic", "aec", "ec", "eac", "ic", "oc", "oic", "ouc", "ack", "aeck", "eck", "eack",
        "ick", "ock", "oick", "ouck", "uck", "uc", "ad", "aed", "ed", "ead", "id", "od", "oid",
        "oud", "ud", "uid", "ag", "aeg", "eg", "eag", "ig", "og", "oug", "ug", "ak", "aek", "ek",
        "eak", "ik", "ok", "oki", "uk", "uik", "ouk", "uki", "al", "ael", "el", "eal", "il", "ol",
        "oli", "olin", "olim", "olir", "oul", "ul", "uli", "ulim", "ulir", "uil", "am", "ami",
        "amli", "amri", "aem", "em", "eam", "im", "om", "omli", "omri", "omi", "oum", "um", "umi",
        "umir", "umin", "umli", "umlir", "umlin", "umri", "an", "aen", "en", "ean", "in", "on",
        "onlim", "onlir", "oun", "un", "unli", "unri", "ar", "arlum", "arlun", "arlug", "arlig",
        "aer", "er", "erlum", "erlun", "erlug", "erlig", "ear", "ir", "irlum", "irlun", "or",
        "orli", "orlim", "orlum", "orlun", "orlig", "orlug", "oir", "our", "ur", "uri", "urim",
        "urum", "us", "as", "ous", "aes", "eas", "at", "atir", "atum", "atin", "aet", "et", "eat",
        "it", "ot", "otir", "atin", "otum", "out", "ut", "ath", "aeth", "eth", "eath", "ith",
        "oth", "outh", "uth",
    ];
    let female_name_1 = vec![
        "A", "Ara", "Alfo", "Bari", "Be", "Bo", "Bha", "Bu", "Ba", "Bra", "Bro", "Brou", "Bru",
        "Da", "Dalo", "Dare", "De", "Dhu", "Dho", "Do", "Dora", "Dwo", "Dou", "Duri", "Du", "El",
        "Eri", "Fi", "Fo", "Fo", "Ga", "Gi", "Gla", "Glori", "Go", "Gra", "Gro", "Groo", "Gru",
        "Grou", "Ha", "Ha", "He", "He", "Ho", "Hou", "Hu", "Ja", "Jo", "Ka", "Khe", "Khu", "Khou",
        "Ko", "Ku", "Ki", "Kra", "Kro", "Lo", "Lu", "Lo", "Ma", "Mo", "Mu", "Na", "No", "Nu",
        "Nora", "Nura", "Ne", "No", "O", "Ori", "Rei", "Ra", "Ru", "Sa", "Si", "Sna", "Sko", "Ska",
        "Stro", "The", "Thi", "Tho", "Thra", "Tha", "Tore", "Tha", "Thra", "Thro", "Thu", "Tu",
        "U", "Umi", "Va", "Vo", "Whu", "We", "Wera", "Yu", "Yo", "Ya",
    ];
    let female_name_2 = vec![
        "b", "br", "dd", "d", "dr", "dm", "dgr", "dw", "f", "fr", "gr", "gg", "gh", "gn", "k",
        "kh", "kgr", "kdr", "kk", "kw", "kh", "kr", "l", "lg", "lgr", "ldr", "lm", "md", "mw",
        "mn", "m", "mm", "mr", "n", "nd", "ndr", "nw", "ngr", "nm", "r", "rr", "rgr", "rdr", "rb",
        "rg", "rn", "rh", "rd", "rm", "rs", "rf", "s", "ss", "sdr", "sgr", "st", "str", "t", "tr",
        "tm", "th", "tdr", "tgr", "v", "vr", "w", "z", "zm", "zn", "zz",
    ];
    let female_name_3 = vec![
        "abelle", "aebelle", "ebelle", "ibelle", "obelle", "ubelle", "alyn", "aelyn", "elyn",
        "ealyn", "ilyn", "olyn", "oulyn", "ulyn", "uilyn", "alynn", "aelynn", "elynn", "ealynn",
        "ilynn", "olynn", "oulynn", "ulynn", "uilynn", "abelyn", "aebelyn", "ebelyn", "eabelyn",
        "ibelyn", "obelyn", "oubelyn", "ubelyn", "uibelyn", "abelynn", "aebelynn", "ebelynn",
        "eabelynn", "ibelynn", "obelynn", "oubelynn", "ubelynn", "uibelyn", "anelyn", "aenelyn",
        "enelyn", "eanelyn", "inelyn", "onelyn", "ounelyn", "unelyn", "uinelyn", "anelynn",
        "aenelynn", "enelynn", "eanelynn", "inelynn", "onelynn", "ounelynn", "unelynn", "uinelynn",
        "agit", "aegit", "egit", "eagit", "igit", "ogit", "ugit", "uigit", "agith", "aegith",
        "egith", "eagith", "igith", "ogith", "ugith", "uigith", "irgit", "irgith", "uirgit",
        "uirgith", "airgit", "airgith", "arika", "aerika", "erika", "earika", "irika", "orika",
        "urika", "atain", "aetain", "etain", "eatain", "itain", "otain", "utain", "ataine",
        "aetaine", "etaine", "eataine", "itaine", "otaine", "utaine", "ahilda", "aehilda",
        "ehilda", "eahilda", "ohilda", "ihilda", "uhilda", "ahulda", "aehulda", "ehulda",
        "eahulda", "ohulda", "ihulda", "uhulda", "agar", "aegar", "egar", "eagar", "igar", "ogar",
        "ugar", "agaer", "egaer", "igaer", "ogaer", "ugaer", "atrud", "aetrud", "etrud", "eatrud",
        "itrud", "otrud", "utrud", "atrude", "aetrude", "etrude", "eatrude", "itrude", "otrude",
        "utrude", "ada", "aeda", "eda", "eada", "ida", "oda", "uda", "alda", "aelda", "elda",
        "ealda", "ilda", "olda", "oulda", "ulda", "alin", "aelin", "elin", "ealin", "ilin", "olin",
        "oulin", "ulin", "aline", "aeline", "eline", "ealine", "iline", "oline", "ouline", "uline",
        "atalin", "aetalin", "etalin", "eatalin", "italin", "otalin", "outalin", "utalin",
        "atalyn", "aetalyn", "etalyn", "eatalyn", "italyn", "otalyn", "outalyn", "utalyn",
        "atelin", "aetelin", "etelin", "eatelin", "itelin", "otelin", "outelin", "utelin",
        "atelyn", "aetelyn", "etelyn", "eatelyn", "itelyn", "otelyn", "outelyn", "utelyn",
        "angrid", "aengrid", "engrid", "eangrid", "ingrid", "ongrid", "oungrid", "ungrid", "ani",
        "aeni", "eni", "eani", "ini", "oni", "ouni", "uni", "ana", "aena", "ena", "eana", "ina",
        "ona", "ouna", "una", "alsia", "aelsia", "elsia", "ealsia", "ilsia", "olsia", "oulsia",
        "ulsia", "ala", "aela", "ela", "eala", "ila", "ola", "oula", "ula", "abella", "aebella",
        "ebella", "eabella", "ibella", "obella", "oubella", "ubella", "abela", "aebela", "ebela",
        "eabela", "ibela", "obela", "oubela", "ubela", "astr", "aestr", "estr", "eastr", "istr",
        "ostr", "oustr", "ustr", "abo", "aebo", "ebo", "eabo", "ibo", "obo", "oubo", "ubo",
        "abena", "aebena", "ebena", "eabena", "ibena", "obena", "oubena", "ubena", "abera",
        "aebera", "ebera", "eabera", "ibera", "obera", "oubera", "ubera", "adeth", "aedeth",
        "edeth", "eadeth", "ideth", "odeth", "oudeth", "udeth", "adrid", "aedrid", "edrid",
        "eadrid", "idrid", "odrid", "oudrid", "udrid", "abyrn", "aebyrn", "ebyrn", "eabyrn",
        "ibyrn", "obyrn", "oubyrn", "ubyrn", "agrett", "aegrett", "egrett", "eagrett", "igrett",
        "ogrett", "ougrett", "ugrett", "agret", "aegret", "egret", "eagret", "igret", "ogret",
        "ougret", "ugret", "asli", "aesli", "esli", "easli", "isli", "osli", "ousli", "usli",
        "ahilda", "aehilda", "ehilda", "eahilda", "ihilda", "ohilda", "ouhilda", "uhilda",
        "ahilde", "aehilde", "ehilde", "eahilde", "ihilde", "ohilde", "ouhilde", "uhilde", "aginn",
        "aeginn", "eginn", "eaginn", "iginn", "oginn", "ouginn", "uginn", "amora", "aemora",
        "emora", "eamora", "imora", "omora", "oumora", "umora", "alydd", "aelydd", "elydd",
        "ealydd", "ilydd", "olydd", "oulydd", "ulydd", "akara", "aekara", "ekara", "eakara",
        "ikara", "okara", "oukara", "ukara", "aren", "aeren", "eren", "earen", "iren", "oren",
        "ouren", "uren", "arra", "aerra", "erra", "earra", "irra", "orra", "ourra", "urra", "are",
        "aere", "ere", "eare", "ire", "ore", "oure", "ure", "awynn", "aewynn", "ewynn", "eawynn",
        "iwynn", "owynn", "ouwynn", "uwynn", "atryd", "aetryd", "etryd", "eatryd", "itryd",
        "otryd", "outryd", "utryd", "athra", "aethra", "ethra", "eathra", "ithra", "othra",
        "outhra", "uthra", "aserd", "aeserd", "eserd", "easerd", "iserd", "oserd", "ouserd",
        "userd", "tryd",
    ];
    let lastname_part_1 = vec![
        "Ale", "Amber", "Anvil", "Ash", "Axe", "Barbed", "Barrel", "Battle", "Beast", "Bone",
        "Beryl", "Bitter", "Black", "Blazing", "Blessed", "Blood", "Blunt", "Bone", "Bottle",
        "Boulder", "Brew", "Brick", "Bright", "Bristle", "Broad", "Bronze", "Brown", "Cave",
        "Cask", "Chain", "Crag", "Chaos", "Coal", "Coin", "Copper", "Dark", "Deep", "Dim",
        "Dragon", "Drake", "Dusk", "Earth", "Ember", "Fiery", "Flint", "Flask", "Flint", "Flat",
        "Forge", "Frost", "Giant", "Gold", "Golden", "Granite", "Gravel", "Gray", "Great", "Grey",
        "Grim", "Grumble", "Hammer", "Hard", "Heavy", "Hill", "Honor", "Horn", "Ice", "Ingot",
        "Iron", "Jade", "Keg", "Kobold", "Krag", "Lead", "Large", "Lava", "Leather", "Light",
        "Long", "Marble", "Magma", "Merry", "Metal", "Mithril", "Mine", "Mountain", "Mud", "Night",
        "Noble", "Oak", "Oaken", "Onyx", "Opal", "Ore", "Orc", "Plate", "Pebble", "Red", "Rune",
        "Ruby", "Sapphire", "Shadow", "Shatter", "Smelt", "Silver", "Snow", "Steel", "Storm",
        "Strong", "Troll", "Thunder", "Twilight", "Treasure", "Under", "War", "Warm", "Whit",
        "Wind", "Wold", "Wraith", "Wyvern",
    ];
    let lastname_part_2 = vec![
        "arm", "armour", "axe", "back", "bane", "beard", "basher", "belly", "belt", "bender",
        "blade", "born", "bow", "braid", "braids", "branch", "brand", "breaker", "brew", "brewer",
        "bringer", "brow", "buckle", "buster", "chest", "chin", "cloak", "coat", "delver",
        "digger", "foot", "fall", "fury", "finger", "flayer", "feet", "forge", "forged", "grog",
        "grip", "guard", "gut", "granite", "hand", "head", "heart", "helm", "hide", "hood", "horn",
        "jaw", "mace", "mail", "maker", "mantle", "mane", "master", "maul", "miner", "pike",
        "rock", "river", "shield", "shaper", "sword", "shoulder", "stone", "spine", "sunder",
        "thane", "toe", "tank", "view",
    ];

    let mut rng = rng();
    let is_male: bool = rng.random();

    if is_male {
        return format!(
            "{}{}{} {}{} ♂️",
            male_name_1.choose(&mut rng).expect("male_name_1 was empty"),
            male_name_2.choose(&mut rng).expect("male_name_2 was empty"),
            male_name_3.choose(&mut rng).expect("male_name_3 was empty"),
            lastname_part_1
                .choose(&mut rng)
                .expect("lastname_part_1 was empty"),
            lastname_part_2
                .choose(&mut rng)
                .expect("lastname_part_2 was empty"),
        );
    }
    return format!(
        "{}{}{} {}{} ♀️",
        female_name_1
            .choose(&mut rng)
            .expect("female_name_1 was empty"),
        female_name_2
            .choose(&mut rng)
            .expect("female_name_2 was empty"),
        female_name_3
            .choose(&mut rng)
            .expect("female_name_3 was empty"),
        lastname_part_1
            .choose(&mut rng)
            .expect("lastname_part_1 was empty"),
        lastname_part_2
            .choose(&mut rng)
            .expect("lastname_part_2 was empty"),
    );
}
