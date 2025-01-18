// use std::time::Duration;

// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use fast_array::{fast_array::fast_array_basics::AsFastArray, FastArray}; // Replace with your actual crate

// // fn bench_vec_map(c: Criterion) {
// //     let mut x = c.measurement_time(Duration::from_secs(10));
    
    
// // }

// fn bench_fastarray(c: &mut Criterion) {
//     // let mut x = c.measurement_time(Duration::from_secs(10));
//     // let binding = c.measurement_time(Duration::from_secs(10));
//     let mut group = c.benchmark_group("Group");
    
//     group.measurement_time(Duration::from_secs(10));
    
//     group.bench_function("Vec map", |b| {
//         b.iter(|| {
//             let vec = (0..16_000_000).collect::<Vec<_>>();
//             let mod_vec: Vec<_> = vec.into_iter().map(|x| x + 1).collect();
//             black_box(mod_vec);
//         })
//     });
    
//     // group.bench_function("FastArray map", |b| {
//     //     b.iter(|| {
//     //         let fast_arr: FastArray<usize> = FastArray::new_range(0, 16_000_000);
//     //         let iter = fast_arr.as_fast_iterator().map(|x| x+1).as_fast_array();
//     //         black_box(iter);
//     //     })
//     // });


//     group.bench_function("FastArray simd", |b| {
//         b.iter(|| {
//             // let mut fast_arr: FastArray<usize> = FastArray::new_range(0, 16_000_000);
//             let mut fast_arr: FastArray<usize> = (0..16_000_000).into();
//             fast_arr.simd_add(1);
//             black_box(fast_arr);
//         })
//     });

//     group.finish();
// }

// // fn bench_fastarray_simd(c: &mut Criterion) {
// //     let mut x = c.measurement_time(Duration::from_secs(10));
    
// //     x.bench_function("FastArray simd", |b| {
// //         b.iter(|| {
// //             let mut fast_arr: FastArray<i32> = (0..1600000).into();
// //             fast_arr.simd_add(1);
// //             black_box(fast_arr);
// //         })
// //     });
// // }

// criterion_group!(benches, bench_fastarray);
// criterion_main!(benches);
fn main() {}
