use itertools::Itertools;

fn main() {
    let numbers = vec![1, 1, 2, 2, 2, 3];
    println!("Most frequent: {}", most_frequent(&numbers));
}

fn most_frequent(numbers: &[i32]) -> i32 {
    numbers
        .iter() // Erstellt einen Iterator über die Referenzen der Zahlen (&i32)
        .copied() // Kopiert die Werte, um i32 statt &i32 zu erhalten
        .chunk_by(|x| *x) // Gruppiert aufeinanderfolgende gleiche Elemente. Da der Vektor sortiert ist, werden alle gleichen Zahlen gruppiert.
        .into_iter() // Wandelt die GroupBy-Struktur in einen Iterator um, der (Key, Group) Paare liefert
        .map(|(k, v)| (k, v.count())) // Wandelt jedes Paar in (Zahl, Anzahl) um. v.count() konsumiert den Gruppen-Iterator.
        .max_by_key(|(_, v)| *v) // Sucht das Element mit der höchsten Anzahl (v)
        .unwrap() // Entpackt das Option-Ergebnis (panikt bei leerem Input)
        .0 // Gibt den Key (die Zahl selbst) zurück
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_frequent() {
        let numbers = vec![1, 1, 2, 2, 2, 3];
        assert_eq!(most_frequent(&numbers), 2);
    }

    #[test]
    fn test_single_element() {
        let numbers = vec![5];
        assert_eq!(most_frequent(&numbers), 5);
    }

    #[test]
    fn test_all_same() {
        let numbers = vec![7, 7, 7, 7];
        assert_eq!(most_frequent(&numbers), 7);
    }

    #[test]
    fn test_tie() {
        // Bei Gleichstand liefert max_by_key das letzte Element
        let numbers = vec![1, 1, 2, 2];
        assert_eq!(most_frequent(&numbers), 2);
    }
}
