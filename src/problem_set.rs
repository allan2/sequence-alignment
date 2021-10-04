use super::problem::Problem;

#[allow(dead_code)]
pub fn eddy<'a>() -> Problem<'a> {
	Problem::new(5, -2, -6, b"TTCATA", b"TGCTCGTA")
}

#[allow(dead_code)]
pub fn lec_17_slide19<'a>() -> Problem<'a> {
	Problem::new(3, -1, -3, b"GTAC", b"GATCA")
}

#[allow(dead_code)]
pub fn in_class_a<'a>() -> Problem<'a> {
	Problem::new(3, -1, -3, b"TCA", b"ATCG")
}

#[allow(dead_code)]
pub fn in_class_b<'a>() -> Problem<'a> {
	Problem::new(3, -10, -1, b"TCA", b"ATCG")
}

#[allow(dead_code)]
pub fn in_class_c<'a>() -> Problem<'a> {
	Problem::new(3, -1, -1, b"TCA", b"ATCG")
}
