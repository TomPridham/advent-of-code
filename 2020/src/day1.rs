use std::iter;
//After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

//The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.

//To save your vacation, you need to get all fifty stars by December 25th.

//Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

//Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.

//Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

//For example, suppose your expense report contained the following:

//1721
//979
//366
//299
//675
//1456

//In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.

//Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
//
pub fn fix_expense_report() -> u32 {
    let mut report = EXPENSE_REPORT.clone();
    report.sort();
    let rev_report: Vec<_> = report.iter().rev().collect();
    let final_value = report.iter().try_fold(0, |_, first| {
        let val = rev_report.iter().find(|&&second| first + second == 2020);
        if let Some(second) = val {
            return Err(*second * first);
        } else {
            return Ok(0);
        }
    });
    if let Err(f) = final_value {
        return f;
    } else {
        0
    }
}
//The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.

//Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

//In your expense report, what is the product of the three entries that sum to 2020?

pub fn fix_expense_report_triple() -> u32 {
    let mut report = EXPENSE_REPORT.clone();
    report.sort();
    let final_value = report.iter().try_fold(0, |_, first| {
        let first_iter = iter::once(first).cycle();

        let val = first_iter.zip(&report).try_fold(0, |_, (&first, second)| {
            if first + second > 2020 {
                return Ok(0);
            }
            let final_value = report.iter().find(|&third| first + second + third == 2020);

            if let Some(third) = final_value {
                return Err(first * second * third);
            }
            Ok(0)
        });
        if let Err(ret) = val {
            return Err(ret);
        } else {
            return Ok(0);
        }
    });
    if let Err(f) = final_value {
        return f;
    } else {
        0
    }
}

const EXPENSE_REPORT: [u32; 200] = [
    1914, 1931, 1892, 1584, 1546, 1988, 1494, 1709, 1624, 1755, 1849, 1430, 1890, 1675, 1604, 1580,
    1500, 1277, 1729, 1456, 2002, 1075, 1512, 895, 1843, 1921, 1904, 1989, 1407, 1552, 1714, 757,
    1733, 1459, 1777, 1440, 1649, 1409, 1662, 1968, 1775, 1998, 1754, 1938, 1964, 1415, 1990, 1997,
    1870, 1664, 1145, 1782, 1820, 1625, 1599, 1530, 1759, 1575, 1614, 1869, 1620, 1818, 1295, 1667,
    1361, 1520, 1555, 1485, 1502, 1983, 1104, 1973, 1433, 1906, 1583, 1562, 1493, 1945, 1528, 1600,
    1814, 1712, 1848, 1454, 1801, 1710, 1824, 1426, 1977, 1511, 1644, 1302, 1428, 1513, 1261, 1761,
    1680, 1731, 1724, 1970, 907, 600, 1613, 1091, 1571, 1418, 1806, 1542, 1909, 1445, 1344, 1937,
    1450, 1865, 1561, 1962, 1637, 1803, 1889, 365, 1810, 1791, 1591, 1532, 1863, 1658, 1808, 1816,
    1837, 1764, 1443, 1805, 1616, 1403, 1656, 1661, 1734, 1930, 1120, 1920, 1227, 1618, 1640, 1586,
    1982, 1534, 1278, 1269, 1572, 1654, 1472, 1974, 1748, 1425, 1553, 1708, 1394, 1417, 1746, 1745,
    1834, 1787, 1298, 1786, 1966, 1768, 1932, 1523, 1356, 1547, 1634, 1951, 1922, 222, 1461, 1628,
    1888, 1639, 473, 1568, 1783, 572, 1522, 1934, 1629, 1283, 1550, 1859, 2007, 1996, 1822, 996,
    1911, 1689, 1537, 1793, 1762, 1677, 1266, 1715,
];
