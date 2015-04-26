extern crate underscore;

use underscore::vec::VecU;
use underscore::hashmap::HashMapU;
use underscore::btreemap::BTreeMapU;
use std::collections::HashMap;
use std::collections::BTreeMap;

#[test]
fn test_vec_first() {
    let v = vec!(1isize, 2, 3);
    assert_eq!(1isize, *v.first().unwrap());
}

#[test]
fn test_vec_without() {
    let v = vec!(1isize, 2, 2);
    for x in v.without(&vec!(1isize)).iter() {
        assert_eq!(2isize, *x);
    }
}

#[test]
fn test_vec_intersection() {
    let v = vec!(1isize, 2, 3);
    assert_eq!(vec!(2isize, 3), v.intersection(&vec!(2isize, 3)));
}

#[test]
fn test_vec_uniq() {
    let v = vec!(0usize, 1, 1, 2, 2, 3);
    assert_eq!(vec!(0usize, 1, 2, 3), v.uniq());
}

#[test]
fn test_vec_index_of() {
    assert_eq!(2usize, vec!(3isize, 2, 1).index_of(&1isize).unwrap());
    assert_eq!(None, vec!(3isize, 2, 1).index_of(&4isize));
}

#[test]
fn test_vec_last_index_of() {
    assert_eq!(3usize, vec!(1isize, 2, 3, 1).last_index_of(&1isize).unwrap());
    assert_eq!(0usize, vec!(1isize, 2, 3).last_index_of(&1isize).unwrap());
    assert_eq!(None, vec!(3isize, 2, 1).last_index_of(&4isize));
}

#[test]
fn test_vec_object() {
    let obj = vec!(0usize, 1, 2, 3).object(vec!(0isize, 1, 2, 3));

    for key in 0usize..obj.len() {
        match obj.get(&key) {
            Some(value) => assert_eq!(*value as usize, key),
            None => assert!(false),
        }
    }
}

#[test]
fn test_vec_reject() {
    let rejected = vec!(1isize, 2, 10).reject(|&v| v < 10);
    assert_eq!(vec!(10isize), rejected);
}

#[test]
fn test_hashmap_invert() {
    let mut sample = HashMap::new();
    sample.insert(1isize, 1usize);
    sample.insert(2isize, 2usize);
    let inverted = sample.invert();

    for x in vec!(1usize, 2).iter() {
        assert!(inverted.contains_key(x));
        assert_eq!(*x as isize, inverted[x]);
    }
}

#[test]
fn test_hashmap_pick() {
    let mut sample = HashMap::new();
    sample.insert(1isize, 1usize);
    sample.insert(2isize, 2usize);
    let picked = sample.pick(vec!(1isize));

    for key in picked.keys() {
        assert_eq!(1usize, picked[key]);
    }
}

#[allow(unused_variables)]
fn sample_filter(x: &isize, y: &usize) -> bool {
    if *x == 1 { return true; }
    return false;
}

#[test]
fn test_hashmap_pick_by_filter() {
    let mut sample = HashMap::new();
    sample.insert(1isize, 1usize);
    sample.insert(2isize, 2usize);
    let picked = sample.pick_by_filter(sample_filter);

    for key in picked.keys() {
        assert_eq!(1usize, picked[key]);
    }
}

#[test]
fn test_hashmap_omit() {
    let mut sample = HashMap::new();
    sample.insert(1isize, 1usize);
    sample.insert(2isize, 2usize);
    let omitted = sample.omit(vec!(1isize));

    for key in omitted.keys() {
        assert_eq!(2usize, omitted[key]);
    }
}

#[test]
fn test_hashmap_omit_by_filter() {
    let mut sample = HashMap::new();
    sample.insert(1isize, 1usize);
    sample.insert(2isize, 2usize);
    let omitted = sample.omit_by_filter(sample_filter);

    let mut expected = HashMap::new();
    expected.insert(2isize, 2usize);
    assert_eq!(expected, omitted);
}

#[test]
fn test_hashmap_defaults() {
    let mut origin = HashMap::new();
    origin.insert(1isize, 1usize);
    origin.insert(2isize, 2usize);

    let mut appends = HashMap::new();
    appends.insert(1isize, 10000usize);
    appends.insert(3isize, 3usize);

    let defaults = origin.defaults(appends);
    for x in vec!(1isize, 2, 3).iter() {
        assert!(defaults.contains_key(x));
        assert_eq!(*x as usize, *defaults.get(x).unwrap());
    }
}

#[test]
fn test_treemap_invert() {
    let mut sample = BTreeMap::new();
    sample.insert(1isize, 1usize);
    sample.insert(2isize, 2usize);
    let inverted = sample.invert();

    for x in vec!(1usize, 2).iter() {
        assert!(inverted.contains_key(x));
        assert_eq!(*x as isize, *inverted.get(x).unwrap());
    }
}

#[test]
fn test_treemap_pick() {
    let mut sample = BTreeMap::new();
    sample.insert(1isize, 1usize);
    sample.insert(2isize, 2usize);
    let v = vec!(1isize);
    let picked = sample.pick(&v);

    assert_eq!(1usize, picked.len());
    assert_eq!(1usize, *picked.get(&1isize).unwrap());
}

#[test]
fn test_treemap_pick_by_filter() {
    let mut sample = BTreeMap::new();
    sample.insert(1isize, 1usize);
    sample.insert(2isize, 2usize);
    let picked = sample.pick_by_filter(sample_filter);

    assert_eq!(1usize, picked.len());
    assert_eq!(1usize, *picked.get(&1isize).unwrap());
}

#[test]
fn test_treemap_omit() {
    let mut sample = BTreeMap::new();
    sample.insert(1isize, 1usize);
    sample.insert(2isize, 2usize);
    let omitted = sample.omit(&vec!(1isize));

    assert_eq!(1usize, omitted.len());
    assert_eq!(2usize, *omitted.get(&2isize).unwrap());
}

#[test]
fn test_treemap_omit_by_filter() {
    let mut sample = BTreeMap::new();
    sample.insert(1isize, 1usize);
    sample.insert(2isize, 2usize);
    let omitted = sample.omit_by_filter(sample_filter);

    assert_eq!(1usize, omitted.len());
    assert_eq!(2usize, *omitted.get(&2isize).unwrap());
}

#[test]
fn test_treemap_defaults() {
    let mut origin = BTreeMap::new();
    origin.insert(1isize, 1usize);
    origin.insert(2isize, 2usize);

    let mut appends = BTreeMap::new();
    appends.insert(1isize, 10000usize);
    appends.insert(3isize, 3usize);

    let defaults = origin.defaults(appends);
    for x in vec!(1isize, 2, 3).iter() {
        assert!(defaults.contains_key(x));
        assert_eq!(*x as usize, *defaults.get(x).unwrap());
    }
}

#[test]
fn test_treemap_pairs() {
    let mut sample = BTreeMap::new();
    sample.insert(1isize, 1usize);
    sample.insert(2isize, 2usize);
    sample.insert(3isize, 3usize);
    let pairs = sample.pairs();

    assert_eq!((&1isize, &1usize), pairs[0]);
    assert_eq!((&2isize, &2usize), pairs[1]);
    assert_eq!((&3isize, &3usize), pairs[2]);
}
