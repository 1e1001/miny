//! Example for using [allocative::Allocative]

#[cfg(not(feature = "allocative"))]
fn main() {
	panic!("re-run with --features allocative")
}

#[cfg(feature = "allocative")]
fn main() {
	use allocative::Allocative;
	use allocative::FlameGraphBuilder;
	use miny::Miny;

	#[derive(Allocative)]
	struct Example {
		int: Miny<i32>,
		trait_object_inline: Miny<dyn Allocative + Sync>,
		trait_object_allocated: Miny<dyn Allocative + Sync>,
	}

	let example = Miny::new(Example {
		int: Miny::new(0),
		trait_object_inline: Miny::new_unsized(0i32),
		trait_object_allocated: Miny::new_unsized(String::from("contents here")),
	});
	let mut flamegraph = FlameGraphBuilder::default();
	flamegraph.visit_root(&example);
	let flamegraph_src = flamegraph.finish();
	eprintln!(
		r#"
	# To view this as a flamegraph:
	#
	#     cargo install inferno
	#     cargo run --example allocative --features allocative | inferno-flamegraph > flamegraph.svg
	#
	# open the .svg with your browser
	"#
	);
	print!("{}", flamegraph_src.flamegraph().write());
}
