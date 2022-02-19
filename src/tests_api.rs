// !!!cmk test slicing macro s! https://docs.rs/ndarray/latest/ndarray/macro.s.html
// !!!cmk use read_all or new macros to make reading all easier.
// !!!cmk is there a way to set default value based on the result type (if given)
use ndarray as nd;

#[test]
fn rusty_bed1() {
    let file = "bed_reader/tests/data/plink_sim_10s_100v_10pmiss.bed";
    // !!! cmk how come this can't return an error?
    let bed = Bed::builder().filename(file.to_string()).build();
    let val = bed
        .read(ReadArg::builder().missing_value(-127).build())
        .unwrap();
    let mean = val.mapv(|elem| elem as f64).mean().unwrap();
    assert!(mean == -13.142); // really shouldn't do mean on data where -127 represents missing

    let bed = Bed::builder()
        .filename(file.to_string())
        .count_a1(false)
        .build();
    let val = bed
        .read(ReadArg::builder().missing_value(-127).build())
        .unwrap();
    let mean = val.mapv(|elem| elem as f64).mean().unwrap();
    assert!(mean == -13.274); // really shouldn't do mean on data where -127 represents missing
}

#[test]
fn rusty_bed2() {
    // !!!cmk reading one iid is very common. Make it easy.
    let file = "bed_reader/tests/data/plink_sim_10s_100v_10pmiss.bed";
    // !!! cmk how come this can't return an error?
    let bed = Bed::builder().filename(file.to_string()).build();
    let val = bed
        .read(
            ReadArg::builder()
                .missing_value(-127)
                // !!!cmk could it be any slice of usize?
                .iid_index(Index::Full([0].to_vec()))
                .sid_index(Index::Full([1].to_vec()))
                .build(),
        )
        .unwrap();
    let mean = val.mapv(|elem| elem as f64).mean().unwrap();
    println!("{:?}", mean);
    assert!(mean == 0.001); // really shouldn't do mean on data where -127 represents missing
}

// !!!cmk ask reddit help (mention builder library creator)
// macro_rules! read {
//     ($b:expr,$x:expr) => {
//         $b.read(ReadArg::builder().$x.build())
//     }; // () => {
//        //      ReadArg::builder().build()
//        //};
// }

use std::{collections::HashSet, f64::NAN};

use crate::api::{Bed, Index, ReadArg};

#[test]
fn rusty_bed3() {
    // !!!cmk how come fields of bed don't need 'pub' to be accessible here?
    // !!!cmk also show mixing bool and full and none
    // !!!cmk remove the need for wrapping with Bool(), Full(), None()
    let file = "bed_reader/tests/data/plink_sim_10s_100v_10pmiss.bed";
    // !!! cmk how come this can't return an error?
    let mut bed = Bed::builder().filename(file.to_string()).build();
    let iid_bool: Vec<bool> = (0..bed.get_iid_count())
        .map(|elem| (elem % 2) != 0)
        .collect();
    let sid_bool: Vec<bool> = (0..bed.get_sid_count())
        .map(|elem| (elem % 8) != 0)
        .collect();
    // let val = read!(
    //     bed,
    //     ReadArg::missing_value(-127)
    //         .iid_index(Index::Bool(iid_bool))
    //         .sid_index(Index::Bool(sid_bool))
    // )
    // .unwrap();
    let val = bed
        .read(
            ReadArg::builder()
                .missing_value(-127)
                // !!!cmk could it be any slice of usize?
                .iid_index(Index::Bool(iid_bool))
                .sid_index(Index::Bool(sid_bool))
                .build(),
        )
        .unwrap();
    let mean = val.mapv(|elem| elem as f64).mean().unwrap();
    println!("{:?}", mean);
    assert!(mean == -6.309); // really shouldn't do mean on data where -127 represents missing
}

#[test]
fn readme_examples() {
    // Read genomic data from a .bed file.

    // >>> import numpy as np
    // >>> from bed_reader import open_bed, sample_file
    // >>>
    // >>> file_name = sample_file("small.bed")
    // >>> bed = open_bed(file_name)
    // >>> val = bed.read()
    // >>> print(val)
    // [[ 1.  0. nan  0.]
    //  [ 2.  0. nan  2.]
    //  [ 0.  1.  2.  0.]]
    // >>> del bed

    // !!!cmk document use statements
    // !!!cmk pull down sample file
    let file_name = "bed_reader/tests/data/small.bed";
    // !!!cmk this should be able to fail is the file is the wrong format and default checking is used.
    let bed = Bed::builder().filename(file_name.to_string()).build();
    //nd let bed = Bed::new(filename)?;
    let val = bed
        .read(ReadArg::builder().missing_value(NAN).build())
        .unwrap();
    //nd val = bed.read(NAN).unwrap();
    //nd val = bed.read!().unwrap();
    println!("{:?}", val);
    // [[1.0, 0.0, NaN, 0.0],
    // [2.0, 0.0, NaN, 2.0],
    // [0.0, 1.0, 2.0, 0.0]], shape=[3, 4], strides=[1, 3], layout=Ff (0xa), const ndim=2

    // Read every second individual and SNPs (variants) from 20 to 30.

    // >>> file_name2 = sample_file("some_missing.bed")
    // >>> bed2 = open_bed(file_name2)
    // >>> val2 = bed2.read(index=np.s_[::2,20:30])
    // >>> print(val2.shape)
    // (50, 10)
    // >>> del bed2

    let file_name2 = "bed_reader/tests/data/some_missing.bed";
    let mut bed2 = Bed::builder().filename(file_name2.to_string()).build();
    let iid_count = bed2.get_iid_count();
    let iid_index = (0..iid_count).step_by(2).collect();
    //nd iid_index = s![..;2];
    let val2 = bed2
        .read(
            ReadArg::builder()
                .iid_index(Index::Full(iid_index))
                .sid_index(Index::Full((20..30).collect()))
                .missing_value(NAN)
                .build(),
        )
        .unwrap();
    //nd val2 = bed2.read!(index(s![..,20..30]).missing_value(NAN)).unwrap();
    println!("{:?}", val2.shape());
    // [50, 10]

    // List the first 5 individual (sample) ids, the first 5 SNP (variant) ids, and every unique chromosome. Then, read every value in chromosome 5.

    // >>> with open_bed(file_name2) as bed3:
    // ...     print(bed3.iid[:5])
    // ...     print(bed3.sid[:5])
    // ...     print(np.unique(bed3.chromosome))
    // ...     val3 = bed3.read(index=np.s_[:,bed3.chromosome=='5'])
    // ...     print(val3.shape)
    // ['iid_0' 'iid_1' 'iid_2' 'iid_3' 'iid_4']
    // ['sid_0' 'sid_1' 'sid_2' 'sid_3' 'sid_4']
    // ['1' '10' '11' '12' '13' '14' '15' '16' '17' '18' '19' '2' '20' '21' '22'
    //  '3' '4' '5' '6' '7' '8' '9']
    // (100, 6)

    let mut bed3 = Bed::builder().filename(file_name2.to_string()).build();
    println!("{:?}", &(bed3.get_iid())[5..]);
    println!("{:?}", &(bed3.get_sid())[5..]);
    let unique: HashSet<_> = bed3.get_chromosome().iter().collect();
    println!("{:?}", unique);
    let is_5 = bed3
        .get_chromosome()
        .iter()
        .map(|elem| elem == "5")
        .collect();
    //nd is_5 = bed3.get_chromosome().mapv(|elem| elem == "5");
    let val3: nd::ArrayBase<nd::OwnedRepr<f64>, nd::Dim<[usize; 2]>> = bed3
        .read(
            ReadArg::builder()
                .sid_index(Index::Bool(is_5))
                .missing_value(NAN)
                .build(),
        )
        .unwrap();
    //nd val3 = bed3.read!(sid_index(is_5).missing_value(NAN)).unwrap();
    println!("{:?}", val3.shape());
    // [100, 6]
}
