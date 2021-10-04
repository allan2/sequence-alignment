use ndarray::Array2;
use std::str;

use self::solution::Solution;

// semantic newtype
pub type Sequence<'a> = &'a [u8];
pub type Coordinates = (usize, usize);

pub struct Problem<'a> {
	match_: i32,
	mismatch: i32,
	gap_extend: i32,
	seq_x: Sequence<'a>,
	seq_y: Sequence<'a>,
}

impl<'a> Problem<'a> {
	pub fn new(
		match_: i32,
		mismatch: i32,
		gap_extend: i32,
		seq_x: Sequence<'a>,
		seq_y: Sequence<'a>,
	) -> Self {
		assert!(match_ > mismatch, "Match score must be greater than mismatch penalty");
		assert!(match_ > gap_extend, "Match score must be greater than gap penalty");
		assert!(!seq_x.is_empty(), "Sequence X cannot be empty");
		assert!(!seq_y.is_empty(), "Sequence Y cannot be empty");

		Self {
			match_,
			mismatch,
			gap_extend,
			seq_x,
			seq_y,
		}
	}

	// This is far from optimal.
	// It works like this:
	// 1. Construct the matrix.
	// 2. Find possible paths.
	// 3. Traceback to find optimal paths.
	//
	// In step 2, there is some elimination of paths based on position.
	pub fn solve(&self) -> Solution {
		let m_len = self.seq_x.len() + 1;
		let n_len = self.seq_y.len() + 1;

		// scoring matrix
		let m = self.matrix();

		// path construction
		let mut paths: Vec<Vec<Coordinates>> = vec![vec![(0, 0)]];
		for i in 0..m_len {
			for j in 0..n_len {
				if (i, j) == (0, 0) {
					continue;
				}

				// Iterate over a copy since we are mutating paths.
				for (idx, mut p) in paths.clone().into_iter().enumerate() {
					let last = p.last().unwrap();

					if i >= last.0 && j >= last.1 {
						let step_x = j - last.1;
						let step_y = i - last.0;

						let add = || {
							p.push((i, j));
							paths.push(p);
						};

						// Add the candidate path if it's connected, i.e., right, down, or diagonal.
						match (step_x, step_y) {
							(0, 1) => add(),
							(1, 0) => add(),
							(1, 1) => add(),
							(_, _) => (),
						}

						// positional path elimination
						let mut elim = || {
							paths.remove(idx);
						};
						if step_y == 1 {
							match (n_len - j, step_x) {
								(0, 0) => elim(), // end of column
								(1, 1) => elim(), // penultimate column
								(_, 2) => elim(), // paths have been constructed
								(_, _) => (),
							}
						}
					}
				}
			}
		}
		println!("Total number of paths: {}", paths.len());
		paths.retain(|p| {
			let last = p.last().unwrap();
			last.0 == m_len - 1 && last.1 == n_len - 1
		});
		println!("Total number of complete paths: {}", paths.len());
		let top_score = m[[m_len - 1, n_len - 1]];
		let mut optimal_paths = Vec::<Vec<Coordinates>>::new();
		for p in paths {
			if self.traceback(&p) == top_score {
				optimal_paths.push(p);
			};
		}
		Solution::new(optimal_paths, top_score, m)
	}

	/// Construct the scoring matrix.
	fn matrix(&self) -> Array2<i32> {
		let m_len = self.seq_x.len() + 1;
		let n_len = self.seq_y.len() + 1;
		let mut m = ndarray::Array::<i32, _>::zeros((m_len, n_len));

		for i in 0..m_len {
			for j in 0..n_len {
				// Fill in the scoring matrix.
				if i == 0 && j == 0 {
					()
				} else if i == 0 {
					m[[0, j]] = self.gap_extend * j as i32;
				} else if j == 0 {
					m[[i, 0]] = self.gap_extend * i as i32;
				} else {
					// Compute candidate scores.
					let x = if i == 0 { b'0' } else { self.seq_x[i - 1] };
					let y = if j == 0 { b'0' } else { self.seq_y[j - 1] };
					let align = m[[i - 1, j - 1]] + align_score(self.match_, self.mismatch, x, y);
					let gap_down = m[[i - 1, j]] + self.gap_extend;
					let gap_right = m[[i, j - 1]] + self.gap_extend;
					let mut a = [align, gap_down, gap_right];
					a.sort();
					// Take the highest.
					m[[i, j]] = *a.last().unwrap();
				}
			}
		}
		m
	}

	// Evaluate the score of a path. The path is assumed to be valid, complete, and ordered from beginning to end.
	fn traceback(&self, path: &Vec<Coordinates>) -> i32 {
		let mut score = 0;
		let mut alignment = (Vec::new(), Vec::new());
		// count
		let mut x_gaps = 0;
		let mut y_gaps = 0;

		for (idx, cell) in path.into_iter().rev().enumerate() {
			if idx == path.len() - 1 {
				break;
			}
			//println!("idx: {}, still kicking", idx);
			let prev = &path[path.len() - idx - 2];
			let x_step = cell.1 - prev.1;
			let y_step = cell.0 - prev.0;

			// closure for any residue, x or y
			let r = |seq_r: &[u8], algnmnt: &Vec<u8>, n_gaps: usize| {
				if seq_r.len() + n_gaps >= algnmnt.len() + 1 {
					seq_r[seq_r.len() + n_gaps - 1 - algnmnt.len()]
				} else {
					// this is for the last letter. No need to append to the gap count;
					b'-'
				}
			};

			let x = r(self.seq_x, &alignment.0, x_gaps);
			let y = r(self.seq_y, &alignment.1, y_gaps);

			match (x_step, y_step) {
				(1, 0) => {
					// gap right
					//println!("Gap right");
					x_gaps += 1;
					alignment.0.push(b'-');
					alignment.1.push(y);
					score += self.gap_extend;
				}
				(0, 1) => {
					// gap down
					//println!("Gap down");
					y_gaps += 1;
					alignment.0.push(x);
					alignment.1.push(b'-');
					score += self.gap_extend;
				}
				(1, 1) => {
					// match or mismatch
					alignment.0.push(x);
					alignment.1.push(y);
					if x == y {
						score += self.match_;
					} else {
						score += self.mismatch;
					}
				}
				(_, _) => panic!("Invalid path"),
			}
		}

		// check that the whole sequence is there
		assert_eq!(
			str::from_utf8(self.seq_x).unwrap(),
			str::from_utf8(&alignment.0)
				.unwrap()
				.replace("-", "")
				.chars()
				.rev()
				.collect::<String>()
		);
		assert_eq!(
			str::from_utf8(self.seq_y).unwrap(),
			str::from_utf8(&alignment.1)
				.unwrap()
				.replace("-", "")
				.chars()
				.rev()
				.collect::<String>()
		);
		score
	}
}

mod solution {
	use super::Coordinates;
	use ndarray::Array2;
	use std::fmt;

	pub struct Solution {
		pub paths: Vec<Vec<Coordinates>>,
		pub score: i32,
		pub matrix: Array2<i32>,
		_secret: ()
	}

	impl Solution {
		pub fn new(paths: Vec<Vec<Coordinates>>, score: i32, matrix: Array2<i32>) -> Self {
			assert!(paths.len() > 0, "A solution must have at least one path");
			Self {
				paths,
				score,
				matrix,
				_secret: ()  // mandate usage of the new constructor
			}
		}
	}

	impl fmt::Display for Solution {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			fn path_str(p: &Vec<Coordinates>) -> String {
				let mut s = String::new();
				for c in p {
					s += &format!("({}, {}), ", c.0, c.1);
				}
				let mut chars = s.chars();
				chars.next_back(); // remove the trailing space
				chars.next_back(); // remove the comma
				chars.as_str().to_owned()
			}

			fn multi_path_str(paths: &Vec<Vec<Coordinates>>) -> String {
				let mut m = String::new();
				for (idx, p) in paths.iter().enumerate() {
					m += &format!("Solution {}: {}\n", idx + 1, &path_str(p));
				}
				m
			}

			let score_msg = format!("Optimal score: {}", self.score);
			let msg = match self.paths.len() {
				0 => panic!("No solution found"),
				1 => format!("One solution\n{}", path_str(&self.paths[0])),
				n => format!("{} solutions\n{}", n, multi_path_str(&self.paths)),
			};
			write!(
				f,
				"\n=== Solution ===\n\n{}\n{}\n{}\nEND\n",
				score_msg, msg, self.matrix
			)
		}
	}
}

fn align_score(match_: i32, mismatch: i32, x: u8, y: u8) -> i32 {
	if x == y {
		match_
	} else {
		mismatch
	}
}

#[cfg(test)]
mod tests {
	use super::{super::problem_set, align_score};
	use ndarray::array;

	#[test]
	fn align_score_match() {
		assert_eq!(align_score(3, -1, b'A', b'A'), 3)
	}

	#[test]
	fn align_score_mismatch() {
		assert_eq!(align_score(3, -1, b'A', b'G'), -1)
	}

	#[test]
	fn eddy_matrix() {
		assert_eq!(
			problem_set::eddy().matrix(),
			array![
				[0, -6, -12, -18, -24, -30, -36, -42, -48],
				[-6, 5, -1, -7, -13, -19, -25, -31, -37],
				[-12, -1, 3, -3, -2, -8, -14, -20, -26],
				[-18, -7, -3, 8, 2, 3, -3, -9, -15],
				[-24, -13, -9, 2, 6, 0, 1, -5, -4],
				[-30, -19, -15, -4, 7, 4, -2, 6, 0],
				[-36, -25, -21, -10, 1, 5, 2, 0, 11]
			]
		);
	}

	#[test]
	fn lec_17_slide_19_matrix() {
		assert_eq!(
			problem_set::lec_17_slide19().matrix(),
			array![
				[0, -3, -6, -9, -12, -15],
				[-3, 3, 0, -3, -6, -9],
				[-6, 0, 2, 3, 0, -3],
				[-9, -3, 3, 1, 2, 3],
				[-12, -6, 0, 2, 4, 1]
			]
		);
	}

	#[test]
	fn in_class_a_matrix() {
		assert_eq!(
			problem_set::in_class_a().matrix(),
			array![
				[0, -3, -6, -9, -12],
				[-3, -1, 0, -3, -6],
				[-6, -4, -2, 3, 0],
				[-9, -3, -5, 0, 2],
			]
		);
	}

	#[test]
	fn in_class_b_matrix() {
		assert_eq!(
			problem_set::in_class_b().matrix(),
			array![
				[0, -1, -2, -3, -4],
				[-1, -2, 2, 1, 0],
				[-2, -3, 1, 5, 4],
				[-3, 1, 0, 4, 3],
			]
		);
	}

	#[test]
	fn in_class_c_matrix() {
		assert_eq!(
			problem_set::in_class_c().matrix(),
			array![
				[0, -1, -2, -3, -4],
				[-1, -1, 2, 1, 0],
				[-2, -2, 1, 5, 4],
				[-3, 1, 0, 4, 4],
			]
		);
	}

	#[test]
	fn eddy_optimal_path() {
		let soln = problem_set::eddy().solve();
		assert_eq!(
			soln.paths,
			vec![vec![
				(0, 0),
				(1, 1),
				(1, 2),
				(1, 3),
				(2, 4),
				(3, 5),
				(4, 6),
				(5, 7),
				(6, 8)
			]]
		);
		assert_eq!(soln.score, 11);
	}
}
