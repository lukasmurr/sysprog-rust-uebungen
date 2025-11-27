use std::str::FromStr;

fn main() {
    let input = "1, 2; 3, 4";
    match parse_list(input) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("Error: {:?}", e),
    }
}

// 1. Was macht die Funktion?
// Die Funktion parst einen String, der Listen von Zahlen enthält.
// Die Listen sind durch Semikolons ';' getrennt.
// Die Zahlen innerhalb einer Liste sind durch Kommas ',' getrennt.
// Whitespace wird ignoriert.
// Leere Einträge (z.B. durch doppelte Trennzeichen) werden ignoriert.
// Das Ergebnis ist ein Result, das im Erfolgsfall einen Vektor von Vektoren von u32 enthält (Vec<Vec<u32>>).
// Wenn das Parsen einer Zahl fehlschlägt, wird der Fehler zurückgegeben.

fn parse_list(input: &str) -> Result<Vec<Vec<u32>>, <u32 as FromStr>::Err> {
    input
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse())
                .collect()
        })
        .collect()
}

// 3. Kann man die Funktion noch kürzer schreiben?
// Ja, durch Verwendung von Funktionszeigern statt Closures wo möglich (z.B. str::trim).
#[allow(dead_code)]
fn parse_list_short(input: &str) -> Result<Vec<Vec<u32>>, <u32 as FromStr>::Err> {
    input
        .split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::parse)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 2. Testfälle
    #[test]
    fn test_parse_list_ok() {
        let input = "1, 2; 3, 4";
        let expected = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(parse_list(input), Ok(expected));
    }

    #[test]
    fn test_parse_list_ok_whitespace() {
        let input = "  10 , 20  ;  30  ";
        let expected = vec![vec![10, 20], vec![30]];
        assert_eq!(parse_list(input), Ok(expected));
    }

    #[test]
    fn test_parse_list_ok_empty_parts() {
        let input = "1,2;;3,4,";
        let expected = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(parse_list(input), Ok(expected));
    }

    #[test]
    fn test_parse_list_err() {
        let input = "1, 2; 3, a";
        assert!(parse_list(input).is_err());
    }

    #[test]
    fn test_parse_list_short_ok() {
        let input = "1, 2; 3, 4";
        let expected = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(parse_list_short(input), Ok(expected));
    }
}
