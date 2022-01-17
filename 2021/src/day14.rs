use std::collections::HashMap;
//The incredible pressures at this depth are starting to put a strain on your submarine. The submarine has polymerization equipment that would produce suitable materials to reinforce the submarine, and the nearby volcanically-active caves should even have the necessary input elements in sufficient quantities.

//The submarine manual contains instructions for finding the optimal polymer formula; specifically, it offers a polymer template and a list of pair insertion rules (your puzzle input). You just need to work out what polymer would result after repeating the pair insertion process a few times.

//For example:

//NNCB

//CH -> B
//HH -> N
//CB -> H
//NH -> C
//HB -> C
//HC -> B
//HN -> C
//NN -> C
//BH -> H
//NC -> B
//NB -> B
//BN -> B
//BB -> N
//BC -> B
//CC -> N
//CN -> C

//The first line is the polymer template - this is the starting point of the process.

//The following section defines the pair insertion rules. A rule like AB -> C means that when elements A and B are immediately adjacent, element C should be inserted between them. These insertions all happen simultaneously.

//So, starting with the polymer template NNCB, the first step simultaneously considers all three pairs:

//The first pair (NN) matches the rule NN -> C, so element C is inserted between the first N and the second N.
//The second pair (NC) matches the rule NC -> B, so element B is inserted between the N and the C.
//The third pair (CB) matches the rule CB -> H, so element H is inserted between the C and the B.

//Note that these pairs overlap: the second element of one pair is the first element of the next pair. Also, because all pairs are considered simultaneously, inserted elements are not considered to be part of a pair until the next step.

//After the first step of this process, the polymer becomes NCNBCHB.

//Here are the results of a few steps using the above rules:

//Template:     NNCB
//After step 1: NCNBCHB
//After step 2: NBCCNBBBCBHCB
//After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
//After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB

//This polymer grows quickly. After step 5, it has length 97; After step 10, it has length 3073. After step 10, B occurs 1749 times, C occurs 298 times, H occurs 161 times, and N occurs 865 times; taking the quantity of the most common element (B, 1749) and subtracting the quantity of the least common element (H, 161) produces 1749 - 161 = 1588.

//Apply 10 steps of pair insertion to the polymer template and find the most and least common elements in the result. What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?
//
//--- Part Two ---

//The resulting polymer isn't nearly strong enough to reinforce the submarine. You'll need to run more steps of the pair insertion process; a total of 40 steps should do it.

//In the above example, the most common element is B (occurring 2192039569602 times) and the least common element is H (occurring 3849876073 times); subtracting these produces 2188189693529.

//Apply 40 steps of pair insertion to the polymer template and find the most and least common elements in the result. What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?

pub fn compute_polymerization(num_insertions: usize) {
    let insertion_map: HashMap<String, char> =
        INSERTIONS
            .iter()
            .fold(HashMap::new(), |mut map, (key, val)| {
                map.insert(key.to_string(), *val);
                map
            });
    let polymer_map =
        TEMPLATE
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .fold(HashMap::new(), |mut map, pair| {
                let pair = &pair.iter().collect::<String>();
                let e = map.entry(pair.clone()).or_insert(0u64);
                *e += 1;
                map
            });
    let polymer = (0..num_insertions).fold(polymer_map, |map, _| {
        map.iter()
            .fold(HashMap::new(), |mut new_map, (pair, &count)| {
                if let Some(&c) = insertion_map.get(pair) {
                    let (left, right) = match pair.chars().collect::<Vec<char>>()[..] {
                        [first, second, ..] => (first, second),
                        _ => unreachable!(),
                    };

                    let left_pair = new_map.entry(format!("{}{}", left, c)).or_insert(0);
                    *left_pair += count;
                    let right_pair = new_map.entry(format!("{}{}", c, right)).or_insert(0);
                    *right_pair += count;
                } else {
                    let e = new_map.entry(pair.clone()).or_insert(0);
                    *e += count;
                }
                new_map
            })
    });

    let mut char_map = polymer
        .into_iter()
        .fold(HashMap::new(), |mut m, (pair, count)| {
            let (left, right) = match pair.chars().collect::<Vec<char>>()[..] {
                [first, second, ..] => (first, second),
                _ => unreachable!(),
            };

            let left_char = m.entry(left).or_insert(0f64);
            *left_char += count as f64;
            let right_char = m.entry(right).or_insert(0f64);
            *right_char += count as f64;
            m
        })
        .into_iter()
        .collect::<Vec<(char, f64)>>();
    char_map.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!(
        "{}",
        (char_map.last().unwrap().1 / 2f64 - char_map[0].1 / 2f64).ceil()
    );
}
const ATEMPLATE: &str = "NNCB";

const AINSERTIONS: [(&str, char); 16] = [
    ("CH", 'B'),
    ("HH", 'N'),
    ("CB", 'H'),
    ("NH", 'C'),
    ("HB", 'C'),
    ("HC", 'B'),
    ("HN", 'C'),
    ("NN", 'C'),
    ("BH", 'H'),
    ("NC", 'B'),
    ("NB", 'B'),
    ("BN", 'B'),
    ("BB", 'N'),
    ("BC", 'B'),
    ("CC", 'N'),
    ("CN", 'C'),
];
const TEMPLATE: &str = "BNSOSBBKPCSCPKPOPNNK";
const INSERTIONS: [(&str, char); 100] = [
    ("HH", 'N'),
    ("CO", 'F'),
    ("BC", 'O'),
    ("HN", 'V'),
    ("SV", 'S'),
    ("FS", 'F'),
    ("CV", 'F'),
    ("KN", 'F'),
    ("OP", 'H'),
    ("VN", 'P'),
    ("PF", 'P'),
    ("HP", 'H'),
    ("FK", 'K'),
    ("BS", 'F'),
    ("FP", 'H'),
    ("FN", 'V'),
    ("VV", 'O'),
    ("PS", 'S'),
    ("SK", 'N'),
    ("FF", 'K'),
    ("PK", 'V'),
    ("OF", 'N'),
    ("VP", 'K'),
    ("KB", 'H'),
    ("OV", 'B'),
    ("CH", 'F'),
    ("SF", 'F'),
    ("NH", 'O'),
    ("NC", 'N'),
    ("SP", 'N'),
    ("NN", 'F'),
    ("OK", 'S'),
    ("BB", 'S'),
    ("NK", 'S'),
    ("FH", 'P'),
    ("FC", 'S'),
    ("OB", 'P'),
    ("VS", 'P'),
    ("BF", 'S'),
    ("HC", 'V'),
    ("CK", 'O'),
    ("NP", 'K'),
    ("KV", 'S'),
    ("OS", 'V'),
    ("CF", 'V'),
    ("FB", 'C'),
    ("HO", 'S'),
    ("BV", 'V'),
    ("KS", 'C'),
    ("HB", 'S'),
    ("SO", 'N'),
    ("PH", 'C'),
    ("PN", 'F'),
    ("OC", 'F'),
    ("KO", 'F'),
    ("VF", 'V'),
    ("CS", 'O'),
    ("VK", 'O'),
    ("FV", 'N'),
    ("OO", 'K'),
    ("NS", 'S'),
    ("KK", 'C'),
    ("FO", 'S'),
    ("PV", 'S'),
    ("CN", 'O'),
    ("VC", 'P'),
    ("SS", 'C'),
    ("PO", 'P'),
    ("BN", 'N'),
    ("PB", 'N'),
    ("PC", 'H'),
    ("SH", 'K'),
    ("BH", 'F'),
    ("HK", 'O'),
    ("VB", 'P'),
    ("NV", 'O'),
    ("NB", 'C'),
    ("CP", 'H'),
    ("NO", 'K'),
    ("PP", 'N'),
    ("CC", 'S'),
    ("CB", 'K'),
    ("VH", 'H'),
    ("SC", 'C'),
    ("KC", 'N'),
    ("SB", 'B'),
    ("BP", 'P'),
    ("KP", 'K'),
    ("SN", 'H'),
    ("KF", 'K'),
    ("KH", 'B'),
    ("HV", 'V'),
    ("HS", 'K'),
    ("NF", 'B'),
    ("ON", 'H'),
    ("BO", 'P'),
    ("VO", 'K'),
    ("OH", 'C'),
    ("HF", 'O'),
    ("BK", 'H'),
];
