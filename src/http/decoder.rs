use std::collections::HashMap;

pub struct HttpUrlDecoder;

impl HttpUrlDecoder {
    pub fn decode_utf_8(line: &str) -> String {
        let mut decoded = String::new();
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            if c == '%' {
                let mut encoded = String::new();
                encoded.push('%');
                if let Some(c) = chars.next() {
                    encoded.push(c);
                    if let Some(c) = chars.next() {
                        encoded.push(c);
                    }
                }
                match ENCODED_CHARACTERS.get(&encoded[..]) {
                    Some(decoded_char) => decoded.push_str(decoded_char),
                    None => decoded.push_str(&encoded[..]),
                }
            } else {
                decoded.push(c);
            }
        }
        decoded
    }

    pub fn encode_utf_8(line: &str) -> String {
        let mut encoded = String::new();
        while let Some(c) = line.chars().next() {
            let mut encoded_char = String::new();
            encoded_char.push(c);
            match DECODED_CHARACTERS.get(&encoded_char[..]) {
                Some(encoded_char) => encoded.push_str(encoded_char),
                None => encoded.push(c),
            }
        }
        encoded
    }
}

lazy_static! {
    static ref ENCODED_CHARACTERS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("%20", " ");
        m.insert("%21", "!");
        m.insert("%22", "\"");
        m.insert("%23", "#");
        m.insert("%24", "$");
        m.insert("%25", "%");
        m.insert("%26", "&");
        m.insert("%27", "'");
        m.insert("%28", "(");
        m.insert("%29", ")");
        m.insert("%2A", "*");
        m.insert("%2B", "+");
        m.insert("%2C", ",");
        m.insert("%2D", "-");
        m.insert("%2E", ".");
        m.insert("%2F", "/");
        m.insert("%30", "0");
        m.insert("%31", "1");
        m.insert("%32", "2");
        m.insert("%33", "3");
        m.insert("%34", "4");
        m.insert("%35", "5");
        m.insert("%36", "6");
        m.insert("%37", "7");
        m.insert("%38", "8");
        m.insert("%39", "9");
        m.insert("%3A", ":");
        m.insert("%3B", ";");
        m.insert("%3C", "<");
        m.insert("%3D", "=");
        m.insert("%3E", ">");
        m.insert("%3F", "?");
        m.insert("%40", "@");
        m.insert("%41", "A");
        m.insert("%42", "B");
        m.insert("%43", "C");
        m.insert("%44", "D");
        m.insert("%45", "E");
        m.insert("%46", "F");
        m.insert("%47", "G");
        m.insert("%48", "H");
        m.insert("%49", "I");
        m.insert("%4A", "J");
        m.insert("%4B", "K");
        m.insert("%4C", "L");
        m.insert("%4D", "M");
        m.insert("%4E", "N");
        m.insert("%4F", "O");
        m.insert("%50", "P");
        m.insert("%51", "Q");
        m.insert("%52", "R");
        m.insert("%53", "S");
        m.insert("%54", "T");
        m.insert("%55", "U");
        m.insert("%56", "V");
        m.insert("%57", "W");
        m.insert("%58", "X");
        m.insert("%59", "Y");
        m.insert("%5A", "Z");
        m.insert("%5B", "[");
        m.insert("%5C", "\\");
        m.insert("%5D", "]");
        m.insert("%5E", "^");
        m.insert("%5F", "_");
        m.insert("%60", "`");
        m.insert("%61", "a");
        m.insert("%62", "b");
        m.insert("%63", "c");
        m.insert("%64", "d");
        m.insert("%65", "e");
        m.insert("%66", "f");
        m.insert("%67", "g");
        m.insert("%68", "h");
        m.insert("%69", "i");
        m.insert("%6A", "j");
        m.insert("%6B", "k");
        m.insert("%6C", "l");
        m.insert("%6D", "m");
        m.insert("%6E", "n");
        m.insert("%6F", "o");
        m.insert("%70", "p");
        m.insert("%71", "q");
        m.insert("%72", "r");
        m.insert("%73", "s");
        m.insert("%74", "t");
        m.insert("%75", "u");
        m.insert("%76", "v");
        m.insert("%77", "w");
        m.insert("%78", "x");
        m.insert("%79", "y");
        m.insert("%7A", "z");
        m.insert("%7B", "{");
        m.insert("%7C", "|");
        m.insert("%7D", "}");
        m.insert("%7E", "~");
        m.insert("%80", "€");
        m.insert("%0A", "\n");
        m
    };
    // reverse lookup
    static ref DECODED_CHARACTERS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(" ", "%20");
        m.insert("!", "%21");
        m.insert("\"", "%22");
        m.insert("#", "%23");
        m.insert("$", "%24");
        m.insert("%", "%25");
        m.insert("&", "%26");
        m.insert("'", "%27");
        m.insert("(", "%28");
        m.insert(")", "%29");
        m.insert("*", "%2A");
        m.insert("+", "%2B");
        m.insert(",", "%2C");
        m.insert("-", "%2D");
        m.insert(".", "%2E");
        m.insert("/", "%2F");
        m.insert("0", "%30");
        m.insert("1", "%31");
        m.insert("2", "%32");
        m.insert("3", "%33");
        m.insert("4", "%34");
        m.insert("5", "%35");
        m.insert("6", "%36");
        m.insert("7", "%37");
        m.insert("8", "%38");
        m.insert("9", "%39");
        m.insert(":", "%3A");
        m.insert(";", "%3B");
        m.insert("<", "%3C");
        m.insert("=", "%3D");
        m.insert(">", "%3E");
        m.insert("?", "%3F");
        m.insert("@", "%40");
        m.insert("A", "%41");
        m.insert("B", "%42");
        m.insert("C", "%43");
        m.insert("D", "%44");
        m.insert("E", "%45");
        m.insert("F", "%46");
        m.insert("G", "%47");
        m.insert("H", "%48");
        m.insert("I", "%49");
        m.insert("J", "%4A");
        m.insert("K", "%4B");
        m.insert("L", "%4C");
        m.insert("M", "%4D");
        m.insert("N", "%4E");
        m.insert("O", "%4F");
        m.insert("P", "%50");
        m.insert("Q", "%51");
        m.insert("R", "%52");
        m.insert("S", "%53");
        m.insert("T", "%54");
        m.insert("U", "%55");
        m.insert("V", "%56");
        m.insert("W", "%57");
        m.insert("X", "%58");
        m.insert("Y", "%59");
        m.insert("Z", "%5A");
        m.insert("[", "%5B");
        m.insert("\\", "%5C");
        m.insert("]", "%5D");
        m.insert("^", "%5E");
        m.insert("_", "%5F");
        m.insert("`", "%60");
        m.insert("a", "%61");
        m.insert("b", "%62");
        m.insert("c", "%63");
        m.insert("d", "%64");
        m.insert("e", "%65");
        m.insert("f", "%66");
        m.insert("g", "%67");
        m.insert("h", "%68");
        m.insert("i", "%69");
        m.insert("j", "%6A");
        m.insert("k", "%6B");
        m.insert("l", "%6C");
        m.insert("m", "%6D");
        m.insert("n", "%6E");
        m.insert("o", "%6F");
        m.insert("p", "%70");
        m.insert("q", "%71");
        m.insert("r", "%72");
        m.insert("s", "%73");
        m.insert("t", "%74");
        m.insert("u", "%75");
        m.insert("v", "%76");
        m.insert("w", "%77");
        m.insert("x", "%78");
        m.insert("y", "%79");
        m.insert("z", "%7A");
        m.insert("{", "%7B");
        m.insert("|", "%7C");
        m.insert("}", "%7D");
        m.insert("~", "%7E");
        m.insert("€", "%80");
        m.insert("\n", "%0A");
        m
    };
}

