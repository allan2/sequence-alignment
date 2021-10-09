mod problem;
mod problem_set;

fn main() {
	let eddy = problem_set::eddy();
	let soln = eddy.solve();
	println!("{}", soln);

	let ica = problem_set::in_class_a();
	let soln = ica.solve();
	println!("{}", soln);

	let icb = problem_set::in_class_b();
	let soln = icb.solve();
	println!("{}", soln);
}
