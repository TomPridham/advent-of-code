//You barely reach the safety of the cave when the whale smashes into the cave mouth, collapsing it. Sensors indicate another exit to this cave at a much greater depth, so you have no choice but to press on.

//As your submarine slowly makes its way through the cave system, you notice that the four-digit seven-segment displays in your submarine are malfunctioning; they must have been damaged during the escape. You'll be in a lot of trouble without them, so you'd better figure out what's wrong.

//Each digit of a seven-segment display is rendered by turning on or off any of seven segments named a through g:

//0:      1:      2:      3:      4:
//aaaa    ....    aaaa    aaaa    ....
//b    c  .    c  .    c  .    c  b    c
//b    c  .    c  .    c  .    c  b    c
//....    ....    dddd    dddd    dddd
//e    f  .    f  e    .  .    f  .    f
//e    f  .    f  e    .  .    f  .    f
//gggg    ....    gggg    gggg    ....

//5:      6:      7:      8:      9:
//aaaa    aaaa    aaaa    aaaa    aaaa
//b    .  b    .  .    c  b    c  b    c
//b    .  b    .  .    c  b    c  b    c
//dddd    dddd    ....    dddd    dddd
//.    f  e    f  .    f  e    f  .    f
//.    f  e    f  .    f  e    f  .    f
//gggg    gggg    ....    gggg    gggg

//So, to render a 1, only segments c and f would be turned on; the rest would be off. To render a 7, only segments a, c, and f would be turned on.

//The problem is that the signals which control the segments have been mixed up on each display. The submarine is still trying to display numbers by producing output on signal wires a through g, but those wires are connected to segments randomly. Worse, the wire/segment connections are mixed up separately for each four-digit display! (All of the digits within a display use the same connections, though.)

//So, you might know that only signal wires b and g are turned on, but that doesn't mean segments b and g are turned on: the only digit that uses two segments is 1, so it must mean segments c and f are meant to be on. With just that information, you still can't tell which wire (b/g) goes to which segment (c/f). For that, you'll need to collect more information.

//For each display, you watch the changing signals for a while, make a note of all ten unique signal patterns you see, and then write down a single four digit output value (your puzzle input). Using the signal patterns, you should be able to work out which pattern corresponds to which digit.

//For example, here is what you might see in a single entry in your notes:

//acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
//cdfeb fcadb cdfeb cdbaf

//(The entry is wrapped here to two lines so it fits; in your notes, it will all be on a single line.)

//Each entry consists of ten unique signal patterns, a | delimiter, and finally the four digit output value. Within an entry, the same wire/segment connections are used (but you don't know what the connections actually are). The unique signal patterns correspond to the ten different ways the submarine tries to render a digit using the current wire/segment connections. Because 7 is the only digit that uses three segments, dab in the above example means that to render a 7, signal lines d, a, and b are on. Because 4 is the only digit that uses four segments, eafb means that to render a 4, signal lines e, a, f, and b are on.

//Using this information, you should be able to work out which combination of signal wires corresponds to each of the ten digits. Then, you can decode the four digit output value. Unfortunately, in the above example, all of the digits in the output value (cdfeb fcadb cdfeb cdbaf) use five segments and are more difficult to deduce.

//For now, focus on the easy digits. Consider this larger example:

//be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
//fdgacbe cefdb cefbgd gcbe
//edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
//fcgedb cgb dgebacf gc
//fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
//cg cg fdcagb cbg
//fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
//efabcd cedba gadfec cb
//aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
//gecf egdcabf bgf bfgea
//fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
//gebdcfa ecba ca fadegcb
//dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
//cefg dcbef fcge gbcadfe
//bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
//ed bcgafe cdgba cbgef
//egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
//gbdfcae bgc cg cgb
//gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
//fgae cfgab fg bagce

//Because the digits 1, 4, 7, and 8 each use a unique number of segments, you should be able to tell which combinations of signals correspond to those digits. Counting only digits in the output values (the part after | on each line), in the above example, there are 26 instances of digits that use a unique number of segments (highlighted above).

//In the output values, how many times do digits 1, 4, 7, or 8 appear?
//
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl Digit {
    fn match_string(s: &str) -> Digit {
        match s {
            "abcefg" => Digit::Zero,
            "cf" => Digit::One,
            "acdeg" => Digit::Two,
            "acdfg" => Digit::Three,
            "bcdf" => Digit::Four,
            "abdfg" => Digit::Five,
            "abdefg" => Digit::Six,
            "acf" => Digit::Seven,
            "abcdefg" => Digit::Eight,
            "abcdfg" => Digit::Nine,
            _ => Digit::Zero,
        }
    }
}

pub fn find_easy_digits() {
    let count = DISPLAY_OUTPUTS
        .into_iter()
        .fold(0, |total_digits, display_output| {
            let output_values = display_output.split(" | ").nth(1).unwrap();
            let count = output_values.split(' ').fold(0, |acc, val| {
                let len = val.len();
                if len == 7 || len <= 4 {
                    return acc + 1;
                };
                acc
            });
            count + total_digits
        });
    println!("{}", count);
}

//Through a little deduction, you should now be able to determine the remaining digits. Consider again the first example above:

//acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
//cdfeb fcadb cdfeb cdbaf

//After some careful analysis, the mapping between signal wires and segments only make sense in the following configuration:

//dddd
//e    a
//e    a
//ffff
//g    b
//g    b
//cccc

//So, the unique signal patterns would correspond to the following digits:

//acedgfb: 8
//cdfbe: 5
//gcdfa: 2
//fbcad: 3
//dab: 7
//cefabd: 9
//cdfgeb: 6
//eafb: 4
//cagedb: 0
//ab: 1

//Then, the four digits of the output value can be decoded:

//cdfeb: 5
//fcadb: 3
//cdfeb: 5
//cdbaf: 3

//Therefore, the output value for this entry is 5353.

//Following this same process for each entry in the second, larger example above, the output value of each entry can be determined:

//fdgacbe cefdb cefbgd gcbe: 8394
//fcgedb cgb dgebacf gc: 9781
//cg cg fdcagb cbg: 1197
//efabcd cedba gadfec cb: 9361
//gecf egdcabf bgf bfgea: 4873
//gebdcfa ecba ca fadegcb: 8418
//cefg dcbef fcge gbcadfe: 4548
//ed bcgafe cdgba cbgef: 1625
//gbdfcae bgc cg cgb: 8717
//fgae cfgab fg bagce: 4315

//Adding all of the output values in this larger example produces 61229.

//For each entry, determine all of the wire/segment connections and decode the four-digit output values. What do you get if you add up all of the output values?

struct SevenSegmentMap {
    zero: String,
    one: String,
    two: String,
    three: String,
    four: String,
    five: String,
    six: String,
    seven: String,
    eight: String,
    nine: String,
}

impl SevenSegmentMap {
    pub fn new(input: &str) -> Self {
        let digits: Vec<&str> = input.split(' ').collect();

        let one = digits.iter().find(|digit| digit.len() == 2).unwrap();
        let seven = digits.iter().find(|digit| digit.len() == 3).unwrap();
        let four = digits.iter().find(|digit| digit.len() == 4).unwrap();
        let eight = digits.iter().find(|digit| digit.len() == 7).unwrap();
        let nine = digits
            .iter()
            .find(|digit| digit.len() == 6 && four.chars().all(|c| digit.contains(c)))
            .unwrap();
        let zero = digits
            .iter()
            .find(|&digit| {
                digit.len() == 6 && seven.chars().all(|c| digit.contains(c)) && digit != nine
            })
            .unwrap();
        let six = digits
            .iter()
            .find(|&digit| digit.len() == 6 && digit != zero && digit != nine)
            .unwrap();
        let three = digits
            .iter()
            .find(|&digit| digit.len() == 5 && one.chars().all(|c| digit.contains(c)))
            .unwrap();
        let five = digits
            .iter()
            .find(|&digit| digit.len() == 5 && digit.chars().all(|c| six.contains(c)))
            .unwrap();
        let two = digits
            .iter()
            .find(|&digit| digit.len() == 5 && digit != three && digit != five)
            .unwrap();
        Self {
            one: one.to_string(),
            two: two.to_string(),
            three: three.to_string(),
            four: four.to_string(),
            five: five.to_string(),
            six: six.to_string(),
            seven: seven.to_string(),
            eight: eight.to_string(),
            nine: nine.to_string(),
            zero: zero.to_string(),
        }
    }

    pub fn find_match(&self, digit: &str) -> u64 {
        match digit.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            5 => {
                if digit.chars().all(|c| self.five.contains(c)) {
                    5
                } else if digit.chars().all(|c| self.two.contains(c)) {
                    2
                } else {
                    3
                }
            }
            6 => {
                if digit.chars().all(|c| self.six.contains(c)) {
                    6
                } else if digit.chars().all(|c| self.nine.contains(c)) {
                    9
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}
pub fn find_display_sum() {
    let count =
        DISPLAY_OUTPUTS
            .into_iter()
            .fold(0u64, |total_digits, display_output| {
                let (input, display_values) =
                    match display_output.split(" | ").collect::<Vec<&str>>()[..] {
                        [first, second, ..] => (first, second),
                        _ => unreachable!(),
                    };
                let segment_map = SevenSegmentMap::new(input);
                let reading = display_values
                    .split(' ')
                    .map(|digit| segment_map.find_match(digit).to_string())
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();

                reading + total_digits
            });
    println!("{}", count);
}
const ADISPLAY_OUTPUTS: [&str; 10] = [
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
    "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
    "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
    "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
    "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
    "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
    "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
    "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
    "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
    "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
];
const DISPLAY_OUTPUTS: [&str; 200] = [
    "dfgabce cadfgb cefa ca aecbg dfcegb geabd ecbfg cab agcfbe | egbfadc dbgae gcfeb abgdfc",
    "fbcag eg dbge gcbfe acdgfe gec fdcegba efbdc fbedgc efabdc | ge fagbc dfebc eg",
    "da gdbcea gdcfbe acgbd dbfcgea ebad aecdfg afcbg adc ecdbg | afcegd dacebg ad cdbafeg",
    "gafeb ag acfbed ega ebadfg bfcge dgfa begcda edfba cfgedba | ag dafg dfcbgea feacdb",
    "cefdag gcea fag defca gbdaef afcdg dcbgf ga adfecbg fcbdea | geac cfedba cgea fadecg",
    "fcgaed abdecgf cfaed ceb febacd dbecf bc cbfa gecbad fbdeg | facde cfgade acdefb cbe",
    "bcgae cfedba efcbadg dafbc dfbe fcgbad ef aefgcd acfbe fea | abfcd cafbd fbde dcgbaf",
    "geabdc fedga cafdbg cd aefcbg adcge dac bdegafc cegab dceb | dcfgba cad cad deabcg",
    "gdecab fde aecf adbgfe egdcaf dgaebcf decfg bcfdg ef ceadg | cefdabg fcea efac edf",
    "cbgaf edfa bfgecd fcdbae debca dfb bcdaf bdcaeg fecgadb fd | cafdgeb gabcf df afdebc",
    "fdebgc egdfb bfcdg cfedgab fdeagb egcb fcb cfdeba cb fagcd | gebfd afdbgec bc gdcafbe",
    "cb gcdefa cdb bacg eabfd ebdac gdabec gcbefd bgdecaf caedg | cgdeab ebfdgca edgbca cbefadg",
    "efbc aecgbd ebgdfa ecbag afb cfgeadb bf cadgf bgfca egcabf | bf agdbce fab bfcdeag",
    "cedafg aecdbg fcd cgeda cdfag efcg agfdb defcab fagcebd cf | adgcfe daecg abcged fgedbca",
    "bcgaed cfdgea caebd gdebc bega fbecadg ge fcdaeb gde gfbdc | gabe eg cadebf ecgadf",
    "aefc abefg acfbge agedbf dbgce cab abgdfc ecbga ac bfdceag | beafgd ca ac ac",
    "afbde fedag becda dacegf baf gacdbfe fcabdg fgeb fgdabe fb | fb afbdge bcdea fedacbg",
    "bfd cdgbfa dgefca fabdeg cgebdfa cabd gfcda cbgef db dgfcb | bd badegcf acdb adegcf",
    "ef cfbgd cdbfe bef ceadb bfcgead aedf edbagc abgcef aecbdf | bcfegad bef bef fe",
    "cbagef beadgc fae deacb fcagd cgdebfa dfeb bdecaf fe fcdae | dfaceb efdb beafcg bdef",
    "gdfeacb bca gfeacb gfeca egcb bc dgeacf fabgd fbcdae gcbfa | bfdeca bca adbgf abdgf",
    "faedbc egcaf bdgaf agdebcf fagcb cab bcdgaf dbgc fbgead cb | bdgafc cb dgbfa bgcd",
    "dfabc bgacde ad dab edfa bdfec dbgefca facgb eacfbd fcgdeb | ecdbfag bedfc ebfcad bfcdge",
    "dfegbc cfagb ba gefbc abec bfcdega bfa egadbf fgbcea fgcda | eafbdg ebfgc fgbdcea ba",
    "fgdebca dgbcea adbce cfaedb acebf ef efc cfabg edgafc fedb | fdeb fce gcfab bafgc",
    "cfdg daefgcb caedgb aefbc cbdag fg fdbgac fbg beagdf fbcga | dgfc fecgabd fgcd gbf",
    "cbadef abgdef cfea ac gedbc dcbafg dca fdbecag dceab deafb | eafc ebfdga cefa abcde",
    "fcbged ebdca bde ceagb abcdfeg bd cdaef cgabed ecgfab bgda | baecd dgbcea ecafd dbag",
    "bfeag cdabf dbeacg dgbcefa eda fdabge bafde ed egfd ebcafg | ebfcadg defg fadbe cfbgea",
    "bfd dgbec gfcadb ecdf aefbg cfbgead fd gcbdfe debgf gcdeab | ecdbga aebgf abcfgde fd",
    "ecgfda bcafe dgbefca dagcb dfc acdfb cdabfe cebfag fbde fd | bgdefac cfd febgac df",
    "dcafb dgafcb dbeacf bdga degbcfa cgbaf ga fga efdgca cegfb | ag beagdfc abcfg gcdeaf",
    "cgbfd fegcba ecgfb cbe dafebg bdface gace ec fcgdeab eagfb | gcdfb acge ecb cebgf",
    "adbcgf ef fged feb dabfgce bdgfc becfgd dcabe febgac bdfec | cgdbf fdcgeb afcbge fbdgc",
    "cg dagfc dfagbc cgab dagbf gbdcfe dgc aefgdbc aecdf bafegd | cg gfacbed bfgad bgacfed",
    "cadfeg cfdegba edagcb fae faebc decfb fa aebcg fgba egafcb | baegc fa acfbe af",
    "fed bcdge fcbae gbfd gfabecd cdaegb df egdfac egcfbd becdf | cadegf edcgbf fcbde dbgf",
    "fadcg beafcd cedbag edabg abdfge bdagf fba dgcbaef fb fbge | deafcgb dfcaeb geadfb ecfgdba",
    "bec fbcea cgbf bfgcea cdefa cgadeb cb fdbeag bdaegcf fgabe | ebfacg bgcf cb bc",
    "gb agdbce efacgd cgdfbe agdec afcdb ebag gcb eadfgcb bdgac | efcbagd bedcag bgc baeg",
    "fbe dcbgae cedgfba bcgefa bfdge bf dafge edbgc fcdb bcfdeg | bdfc fdabgce ebgfd fegbd",
    "fb abfdge fgdca fedcag bafcgd bfg gdeabfc fbgac eagcb bcdf | gacfedb fb bf fcadg",
    "bg eadgf bcdea gbd gebda edgbac cagb dfecab fgcbead ecfbgd | gbedca adfebgc acgb bdg",
    "adgfce begafc gfeac cde cgabd ed becdfa gfed egfbcda egacd | ed cgeda dfeacb dec",
    "agefcb gadec gdbafe dg cebafdg dcfg ecagf edg gdeacf ceadb | dgaefc egcadbf cdfg faecg",
    "fdec gcbdaf bafedc cf dbegac ceabd gbadecf fca gbefa bacfe | fc fdce acf afc",
    "fdecg gdefcba cg dbcfe dfbeac gcfedb daefg facdgb gebc cfg | begafdc fcdebag dbcgaef gfc",
    "adcgef agbdfec dc ecad cgeabf edfbg bcdgaf fdgce ceagf cdg | cdgef ecda cade cd",
    "abdefc bdaecfg dgeacb fb gfade bacde bdagcf fdb becf adbef | fcbadg cefabd fdb edfga",
    "efcgab bead fcdeg ae cgdab dgcbae aeg edgca bcegdaf gdcfab | gdafceb ea eag aeg",
    "dceafb befac fca cf fdcb cadgef bedaf ebagc aedfbg acgdbef | beadfc cf cfeba gbeca",
    "gce eacgbd afcdbe eg dfcebga bedca gfbca agfedc gcaeb gebd | gdbe ge ge gbde",
    "bcgae bedgfca gde ed gdaec gbfcad cgbdef gdaecf edfa gfadc | cdbfeg bgdfac cfdeag adgfbc",
    "gebca dbacfeg dbfeag bagfe eadfb gf fdge fbg dfceba gafbdc | egfd fedg egafb gf",
    "begfadc bdcae dgbca edbf acbfeg fcgead be eba afdce aedcbf | bfedac edcab eba fdbace",
    "fdbgc cegabf ac dfbage bgfae faec gac caedbg bfgedca bfagc | acef dfgcb face gbafed",
    "cdbfge bfceg dgacbe baegfd ecb dfceabg ebdfg cb acfge bdfc | cbe gcbefd egdcba eafbdg",
    "fecdb dgbcfae bedca edf ef cfebgd ecgf gfdbc fdbagc afbged | aedbgf ef fecg fagcbd",
    "gdecfb fbadge cabe adfcg efcbd ea gbeacfd fcedba dea fdeac | acbe acbe dea fcgedba",
    "bg dgfac gabdec dgcabef beafc gfceab cgb bafcde fegb cgafb | bcg bg fdgca faebcg",
    "adfgc dfcbge dgafbc gaf acfb dgcea gebcdaf fbgdea fbgdc af | fa agcdf fga gfebda",
    "defag begdf fa becgda aef afcg dfaceg cgdae eacdfb acefbgd | cadfeb edgaf fa ecadbf",
    "fec edfbc fagecd cgdfb ef fcabdg cedba fbecdg bfge faegbcd | befdc dbfecga acgdbf gcbedfa",
    "ed cedbfg bdfac fdeac baed fde eagcbdf abgfdc feadbc gaefc | fdbca de cgbdfe de",
    "fcbga ge cagbef acebfdg egbf acbge ceg egcdfa bagdcf adcbe | cgfab ecg ebcda afgcde",
    "df fadcbeg gcfbde dcagb fbd afgdbc cbeagd dfcab cafbe adfg | eagdbcf fgda cgbda gbfced",
    "agef bgdcaef ade adcbf gdbace cefdg febgdc cgefda fedca ae | ae afdgec febgcd agcdeb",
    "gfbc fdg dbaef fg aefgdcb gefdac gebcd dbegac fegcbd fbedg | cfgb gdebc cbaefdg fgd",
    "ebfgac gcf geabdfc cegbda fgade bcaeg fcba cf fdecgb ecafg | cfg fgc acfgdeb gdabfec",
    "gafeb ebdgac ed cgbfad cgeadfb bdecfa bfcda fabde aed ecdf | bgfdaec fedcbag de fedc",
    "age ebcdfg afcdeg dcfeg efadgbc fgad becad ag eacgd gabecf | ag gcdfbe agecd fdgce",
    "aecfbd geb afbde fdgeab gfeba ebgcadf bg gdbf fgeac bceagd | eacdgfb bge edabcf ebdaf",
    "cgabe aefbcdg fagdce abcdfg dcbe aecgbd cge fbaeg ce dcbga | bgacfed gafcdb egc egc",
    "bdefa fagbd fabdce cdae fceagb deb fbdecg fcbea de agedbcf | fgdcabe afbde bde gfcbde",
    "dgfcea dcbea dabcf edcbga cedafb gfabc cgdabfe afd fd dbfe | debf egdacb dbef edabcg",
    "bfdea adfceb fgcbae feg fdebg bagfdce egcdb fedagb dgaf gf | bgdfea gef cdebfag edcbgaf",
    "ade aefgdc ea feac gefdc eacbdfg eabfgd abcdg efbdgc geacd | eafgdc fgebad cbdefg cdefg",
    "egfdab edfb agfec dbcage ade ed badcfg gfcdeba afedg fgadb | aedbcg bgfda aed dae",
    "cfdbge abgc afgbe cb bcf ebfdga adecf efbacdg agfbce bacfe | bfc fbc cfb bfc",
    "gd bgd gcafeb dfcbage ebfgcd egfbd decg gcafdb aedfb ebgfc | gdec gefbacd gdce dg",
    "gf dbegac becgadf dfge facbe gfa acdgfe ecgfa dcfbag gceda | gfed cefba cabfe acedfg",
    "fd cdbagef fdgbec daebcg dfba fbacdg cdf cfadg gabdc gfcae | agecf cfd cdf egcbad",
    "dfca adgefb fbced cfdbge fa egcabfd eabcf eabfcd cabeg afe | acfd dbecf af af",
    "agfdec dfcag edgacfb baegfd fd gfaecb dacgb ecfga dfec daf | gabcd ebcgadf fbegac fgace",
    "edafg gcfea cbaeg afcdgbe fcde fc acf cedagf begfda cdbfag | cf daecbfg bgace dcgabef",
    "bdefagc gcba bfacde adbce ga dgefc cagdeb dgeabf aegdc adg | faecdgb gda ag dga",
    "becdag bceadf edcab dgacb bdgcaef gd dga cedg agbfc dafebg | gd gced gad cgabd",
    "gbfaed bdacf db fcabg ecfda edcb adcfge adb abcfed cgedfba | bad dba fdecgab edacf",
    "ae cabe ecgafb dfgce afbdge gdacfb bdegfca afe agfcb cegfa | ae fgdeab gcfed cgdfabe",
    "ad fdbcg cbfea fdabgec dca cgadfe fbdac dbag gbcfde bgcfad | dbagfc dac ad cbfdge",
    "fdgbaec ab dabe cabegd cgaed bcdag bgfdc gebcfa agb cadefg | efbgac dcgafe dagecb gba",
    "cafbge gdbfac aefcb adfcegb daefc bacge gcabed febg fb bfc | bfcgae gebf bfc abgcdf",
    "dfbgca bfcae ebc ebfdac eb eadfgcb dfeb edbagc fdcba gfcea | becadf ecb bce badcf",
    "cdefa adcgfe fdceb eacdfbg adf ad fcgea deag gacbfd aecfgb | da dega adcebfg acdbefg",
    "dace dcebg adbcfg abdcg de dceagb fcbeg fbacedg bdeafg deb | bed cabgd ed fecbg",
    "edabgc fdbaegc gcadfe afed fcd gecbf fd fgdcab fedcg dgace | gaecd bdgcaf gedfcab fead",
    "gfbae bgefdc cbfgdae dbgfea abgc bc aecbgf adfce bce afbce | aedgfbc bacgfe bcdefg gfbea",
    "cadeg cagdfb bdc bfcedga cgbafe facbe efbd edcba bafdec bd | dbc agdce efdacb bcd",
    "ac fegacb bac afdc aedfbc defba dgefba gcdeb edacb dcbeagf | egdbc egcfbad decba adfc",
    "bcfdag cgfdbea dce gbcfd ed bgced dfbgec gcaeb fcbead dgfe | defg edc gecdb de",
    "eb fbe afdec bfgda cdfbgae fbgcde aedbfc deafgc ecab efdba | aegcdf fedgcab edcfbag be",
    "cfabg aceg gba fcebda egabfc cbaef acfbdeg ga gcfdb adbfge | cgae abg gace dfabec",
    "fabe af fdaeg bdega fdceg adbfge fad gcedbfa dfgbac aedgbc | afdgceb aebf dfa cgbaed",
    "ecgdf bfcagd dcegb gafdc efac feadbgc agcdef eagbfd fge ef | ef egf dcgfa decgf",
    "ag ecbga bga fdegab gbdcfe gbecf afbecg ebdac edacgfb cagf | gafc ecgbf bag eadgbf",
    "dafe fe cef agdebfc fecdg bedcg bdcafg beafcg dcegaf fdcag | dafe fcdag afdgce adef",
    "gefac dagcfe age facdegb acfbg edfa ea edfcg egdcab fcbgde | gcefa ea aebdgc ecfga",
    "cdfbea debgca adbec bgfadc cdg ceadg bgce gc gfaed fcdgeab | cg cgd afdcbg cdg",
    "gcbefda fdcga cfdebg fgdabc abgd fdbcg cabfde fda efagc ad | dgcfa bacfed cfbegd da",
    "cgbaed bcdefg ebdca dcabf feabcdg cgae deabg ced ec efgdba | bgdfce ceag cde dbegac",
    "debgfa gbdce ecdgbf ag agdc bga acdgbfe bceag cbaef cebagd | bga ga gaceb abecgd",
    "dbgcef eafgb fcdeb bdfcae fadc ac degabc agbcefd abfce cba | fcedb cebdga dacf dafcbeg",
    "bag gceb cbafd bg bgfaec dfgcae cbagf gceaf dfageb bcafegd | agb gb gb fedbagc",
    "gadce gecdf cgf egbfd beagfc afdc dfcageb cf bcgdae dgefca | fc gfc eacfdg cgf",
    "dabcfg ebadf cf cebag fbc eagfcb bcegad fgce eabcf ebcgdaf | feacbdg ebadf adfbe fc",
    "ad fgeac afd bfagcde afbcde gdca debgf adgfe fbagec cfaged | adcg adgefc ad adgcef",
    "fcdbeag fdbage cadf daecb bgecd bafce bda ad cdbaef eacbgf | ad caebdf gbcefa bcade",
    "efbgca cabdgef fabcd dafec bf afdgcb fab agcbd fgbd dgceab | fba fb dcagb dbfcag",
    "bea cbfag agefdc cdeb gbadcef aefgdb eagdc cgdeba egbac be | bgace eb be cbde",
    "acbdeg cefda fb cfbaeg ebf aefcb gbfc ceagb cegbfda gebafd | agcfbe afdec fbgc dfcegab",
    "bc cgdab gbfcad ecbdfa cbd fgdac edfacgb dgaeb afecdg bcgf | fcbg dbc dcafgb bcgf",
    "aedf dga cgfbde fgbde cfbgad dabegf cfgabde geacb bdaeg da | dfea agd baecg gcaeb",
    "gc cefga dbefac bacedg afedg afcbe becgaf bcgf bdgfeac cag | cgbf bgcf dabceg gc",
    "db cdegf gcbfd afgbc dgcfba cbgaef dgba fgebacd fadcbe fdb | fgebac bd fdegc db",
    "debgafc dacbeg fdg cgdba fdagcb eacfg bdcf gafbde fd dcagf | fbcd bgcefda cafbged dfbc",
    "fcab gdbef begadc fdecga abdcefg bafgec cb cbe efbgc egacf | bce gcfeb cgbfade cbaf",
    "aedg agfebcd abfge fge cdbefg eg dgcfba gdebfa acbef gafbd | agdfbc baefc efg gbfea",
    "cdgab cbfedga eacbgd cf dceafg cadgfb fdc efbdg bcfa fdbgc | dfc fc cdgbf gcbadf",
    "gce eg edcfagb eacgdb bfegdc cdgaf ecgad cebadf eagb cedab | dcbae cdafg gedacfb ge",
    "dbeagf gfbda fdebgc gabfdc afbgc cgbea fc cdaf ecadbgf fcb | cfgedb bdagf fc gbecfd",
    "bdcefg egcfab acedg fcdb fbgeda bec cebdg deacbgf cb ebdfg | bc decbg eacgd bdcf",
    "dfgeab cegba facbegd edg dfgceb gafcbd edbgc gfcbd dcfe de | bdceg fecd cedf defgcb",
    "egfb dfgace cfgaebd befac adcgb aeg ge cebag bgface bdacfe | cgbea cgbae cbdgaef afebgc",
    "de efd efdbg efabdcg fgaceb bfgdc fgabe edgfab ebda degacf | fecagd ed adbe de",
    "fedbga gc gec gaefd ecagf gecbda bafce cgfead cgdf cadfbeg | gafed gcfd cgfd gcfd",
    "bedfgca ebfdc acgbfd cfb dfbeg decaf bc dabefc beac aedcgf | debgf efacd cb bc",
    "edgcba eg dbfga acfgeb cdbea bcfdaeg dcge gdeba dacefb egb | efgdacb cgeafbd gfadb gedc",
    "bacfde dgcefa eabdf acfebdg afgdbc dba aedfc aebgf db bdce | db afecd agcdfe agfedbc",
    "dga cgadb cdbge bfgca edab ad cbgade efbcdga afedgc ecfgbd | ad dgecb ad bdgac",
    "abfdec dacfegb faecg cbge acgfb fcgaeb bgfda bc bfc fgcdae | cgeb fgaecb cbeg gadbefc",
    "ea ecgbfda eacb efa facegb bgfea ebfdg gfbac edcgaf gcdfba | gabfe bgfeca ae cabe",
    "ecf bfged cebfgd adfebg dfabec cgdf gbeca cf bdfcgea bgfce | cdfgeab gdbfe daebcf bcagdef",
    "ebf efbdc bf bfcdge dgcfea bgfc cedab begdaf egcdf befdcag | ebgcfda gdfaeb bf ecfgd",
    "egcdf ecafg afebdc df dgfa agfecd cfbdgea ecgafb gdceb cfd | cdf cgadef acfgbde defgc",
    "gbdeacf bfcdg cag acdgf efdbcg ac fagdcb gbfeac bcda edfga | agc gaedf abdc cag",
    "cfgd edbcfg gfebc begcdfa acebgf ged dbace faedbg gecbd gd | bagefc dgefabc defabg becgf",
    "gcbe ce ebdfa cae bdcagfe bgdac gdeacb cbdagf dbace caegfd | ec acedb gabcd bcagfed",
    "efdgba cgabfed ec caef ceagb fbega cbgda ebc afbgec ecbdgf | afegdb gacbd dgefab edafgb",
    "ecagd bfdec fabdce bda beadc gfeabd bdfeacg afbc gecbfd ba | becdf dcaeg gcdea adb",
    "cefb cfabdg dbf bfeacd baged bf baefd gaecfd cbfaedg fecda | bf fedac fdgecba bdf",
    "gce gdbcae fecd efcbgd fbdacg eafbg cefgb cbdfg ec fgdacbe | cge ec bgcfead ceg",
    "deagfb af efca gfecad dfa bacdg cfdegb fecdg egdacfb dfcag | fdgce af edbgafc fdgce",
    "fagdce ebd abdegf defba cafebdg eb daefg dfbca ebga defcbg | afdeb eb bde egba",
    "bafed aecdb fbce cbd gdebafc cfbadg badegf bc ecdfab gecad | dbc dbc badgcfe fdgeba",
    "fgbec bfacde ge edgcfb cgdbfae fdecga cfbga efg dbge cdfeb | bedfcg eg ge afebdcg",
    "dcaef ag gcab aeg acedg edgcb efgdacb cgfbed dafgeb agdbec | bedagc bdegc dfcebg adecf",
    "dcfg abecg bgadc dg fcgbda gadebf cdabfe badcf agd fcdbega | abdfeg gd egbac cabgdf",
    "dfgebc eabcd efcdb dbecfa dac aegdb ca bcgafed fbac fcadge | ca egcdafb afcbdge eagdb",
    "ebgdc cbgdfe dcbfae bfagcd gdef cebga bceagfd ecd dfbgc ed | de ed fdegcba edc",
    "eb feb begdcf efgacb gdbe daecf dgcfb fbdcega cebfd gabcdf | efdbc be afdec efb",
    "begac fbec bcdafg acb cb fgaebd afegbc gebadcf dagec beafg | adgce fceb cgdeafb cab",
    "ca cdfagb ebfca ebdcgaf afebg dfbgae cega cebfd fbagce fac | bgedaf cgae gdaecfb egdabcf",
    "gabfd agfc ecdbfag aefcbd afd af gdabfc bgcfed abedg dcgfb | af bdgcef adgbf gcdafb",
    "egfda fed dbfga adgefb de gacbdf aefgc aecbfd bdge fdbecga | ed de eacbgdf bdeg",
    "dbfag cdbfea gfdbec agfedbc gfb bg gabfcd fgeda dbfac acbg | dfbac gcba gb fbgda",
    "dbcgf gba gdcaeb dgbacfe edfba debgcf cbdgaf afcg ag abfdg | bgafecd edfcgb ag dgfba",
    "bdfcg acebfg egfbacd gfdcba fbg dagb bg cbdaf cgdfe edfcab | bagd gfebacd efbcda gfb",
    "cedbf fcabdge cge feagbc dfcge gedb adcgf bdcefg edacfb eg | cbdfe cge efbcad cfagdeb",
    "cga fgeda ceagd ebcg beadc bgfecad fgdcba gc cagebd dfeabc | cag daebgc cg fcdageb",
    "fgdc fda aebgd cabfde fd agbecf fdacgbe dbfga cfgba bdgacf | facbg fbdagec fdcg ebafcgd",
    "gbced efbgca gfebd bdc baecg fcabed dcbgeaf gbedac dc acdg | gfbace cdafbe cbaeg bcd",
    "afgdec ba dbcge daefg dabeg dfbcage afbd fgdaeb gab cfegab | bcfage bgead ba fgdbcea",
    "ace dgcaef daegbc bfgeda ecbd degab cgabf ce eagbdfc bcgea | cdgeaf bdgea edcb cae",
    "cd cadf gfead gdefab dce adcfge degcf cgefb gdfbcae aebcgd | efgdc dce ecd dc",
    "afdbe ad fgdeb fda aebdcgf bcaefg acbfe fdceab bacd dcagef | afd begfd acfeb fda",
    "dfg afgedbc cbfga gbdace dbagf fdegab edgacf df befd edgba | dgeba df efbd cfadge",
    "bdcgaf aebg badfce efagc gdefc cabfeg abgcf ecafbdg afe ae | aegdbcf ageb eagb ebdfgca",
    "gdceab fb bfe cgbfea edbagfc cafed ebacg ebafc defgcb gbfa | efb efcba bf afgb",
    "bafcg cdfgb ac caf fcdabe adgc dbgefc bedafcg bgacdf bfaeg | bcfegad afegb gabfdce bgcfa",
    "dafgb gca fcab fcabgd dcfaebg gfcdae cgdba ac gbadef egcbd | bcedg aebgfcd gfeadb afbc",
    "adebg fcgaedb dagfe agfc fdbaec fgaecd feg acdfe gf becdgf | feg fge gfe egf",
    "gf gfda gfb gfcebd gefab afcbged eabdg adgbfe fbace bgcade | dbafgce fbdaeg dagf fcbae",
    "fbegcd ecg cefdbag fcgd ebdac gbecd cg bfcgea bafdeg edfgb | fcbegd acbde bcgaef cge",
    "gbcadf cf gfbc defbca acgfd bcaedg afegd cfbgeda caf gcdba | begcad cfa bagfedc gbcdeaf",
    "begacdf dcafbg gb cgfbed gbcde cedfab dfcbe gbc cadge ebfg | gbfe gb ebcdfga bfeg",
    "fe bgadefc cbgdf gbfade cefgd ecdgaf efg afce eacgd dcgeab | gdebac fcae egbdfac fgdeab",
    "ebfca fdac fdb bgdecaf dbcgef cedbfa bgcefa fd eadfb baegd | cgfaeb dgbecf dfb bagecf",
    "ebda eagcf gcbea ba bfcegad acebgd gdecb cdafgb acb gfcdbe | fgcebd cedbgf gbfdac cefag",
    "edacf fed eacgd febadc ebafc bfcgea ecafgdb adbf fd gbfdec | fabce egfcbd deafgbc bafd",
    "gfbe fdbga ged ge aegdf baegcd afecd ebgcfda fgacbd gdeafb | cdfae fdaec ge aefdg",
    "gebaf afdbcg cfged dga fgead febdcga ad fbaedg bade gbfaec | cgbfdea gadfecb agfde gad",
    "fdg cfgdbe cfbeadg cgbf gedbc dcfage dfgeb fg agcbed adefb | gdf bdecg faedb gfd",
    "acdgbe cfabg gfbacd cbf dfbcea bcfegad cf gcdab efbag fdcg | dbfagc fgbdcea cbdga fc",
    "afbcg dgbfac dgaebf gcfea gbceadf afb fb abgcd abcdge fdcb | gbceda cgdabe bcgdae fbgdea",
    "cgbedf dbgfc cbef cf bgfda gcedb gadceb efdcag fdc bcfdaeg | fecb ebdcfg cf badgf",
    "fg cafdg fegadbc aefcdb bcagd degf gfecba adegfc cfg fdeac | gdcafe cgfdae gebfca dfcgae",
    "fdbecg fcagbd fgeabd cadbe bcdag dafgb facg gdc bdgeacf cg | cedab cbdga gcd gc",
    "gdeaf edbcag fbac fgecdba dfbcea afdce ac fbecd dbefcg cea | efcbd gdeaf dfceagb ace",
    "abgdc ebcgd ecgdf ebagcd dabe edgbcfa be cgeabf ceb cdfgba | fdacbg abegcd abdgc cabfdge",
    "fbgcd feacb ed eadf becfd bde fdabce cgdbea fadbcge gbefac | dcgbf fcebd ebcfd gbface",
];
