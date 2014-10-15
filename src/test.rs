extern crate __;

use __::vec::UnderscoreVec;
use __::hashmap::UnderscoreHashMap;
use __::treemap::UnderscoreTreeMap;
use std::collections::HashMap;
use std::collections::TreeMap;

#[test]
fn test_vec_first() {
    let v = vec!(1i, 2, 3);
    assert_eq!(1i, *v.first().unwrap());
}

#[test]
fn test_vec_without() {
    let v = vec!(1i, 2, 2);
    for x in v.without(&vec!(1i)).iter() {
        assert_eq!(2i, *x);
    }
}

#[test]
fn test_vec_intersection() {
    let v = vec!(1i, 2, 3);
    assert_eq!(vec!(2i, 3), v.intersection(&vec!(2i, 3)));
}

#[test]
fn test_vec_uniq() {
    let v = vec!(0u, 1, 1, 2, 2, 3);
    assert_eq!(vec!(0u, 1, 2, 3), v.uniq());
}

#[test]
fn test_vec_index_of() {
    assert_eq!(2u, vec!(3i, 2, 1).index_of(&1i).unwrap());
    assert_eq!(None, vec!(3i, 2, 1).index_of(&4i));
}

#[test]
fn test_vec_last_index_of() {
    assert_eq!(3u, vec!(1i, 2, 3, 1).last_index_of(&1i).unwrap());
    assert_eq!(0u, vec!(1i, 2, 3).last_index_of(&1i).unwrap());
    assert_eq!(None, vec!(3i, 2, 1).last_index_of(&4i));
}

#[test]
fn test_vec_object() {
    let obj = vec!(0u, 1, 2, 3).object(vec!(0i, 1, 2, 3));

    for key in range(0u, obj.len()) {
        match obj.find(&key) {
            Some(value) => assert_eq!(*value as uint, key),
            None => fail!("{} not found at test_vec_object", key),
        }
    }
}

#[test]
fn test_vec_reject() {
    let rejected = vec!(1i, 2, 10).reject(|&v| v < 10);
    assert_eq!(vec!(10i), rejected);
}

#[test]
fn test_hashmap_invert() {
    let mut sample = HashMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);
    let inverted = sample.invert();

    for x in vec!(1u, 2).iter() {
        assert!(inverted.contains_key(x));
        assert_eq!(*x as int, inverted.get_copy(x));
    }
}

#[test]
fn test_hashmap_pick() {
    let mut sample = HashMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);
    let picked = sample.pick(vec!(1i));

    for key in picked.keys() {
        assert_eq!(1u, picked.get_copy(key));
    }
}

#[allow(unused_variable)]
fn sample_filter(x: &int, y: &uint) -> bool {
    if *x == 1 { return true; }
    return false;
}

#[test]
fn test_hashmap_pick_by_filter() {
    let mut sample = HashMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);
    let picked = sample.pick_by_filter(sample_filter);

    for key in picked.keys() {
        assert_eq!(1u, picked.get_copy(key));
    }
}

#[test]
fn test_hashmap_omit() {
    let mut sample = HashMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);
    let omitted = sample.omit(vec!(1i));

    for key in omitted.keys() {
        assert_eq!(2u, omitted.get_copy(key));
    }
}

#[test]
fn test_hashmap_omit_by_filter() {
    let mut sample = HashMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);
    let omitted = sample.omit_by_filter(sample_filter);

    for key in omitted.keys() {
        assert_eq!(2u, omitted.get_copy(key));
    }
}

#[test]
fn test_hashmap_defaults() {
    let mut origin = HashMap::new();
    origin.insert(1i, 1u);
    origin.insert(2i, 2u);

    let mut appends = HashMap::new();
    appends.insert(1i, 10000u);
    appends.insert(3i, 3u);

    let defaults = origin.defaults(appends);
    for x in vec!(1i, 2, 3).iter() {
        assert!(defaults.contains_key(x));
        assert_eq!(*x as uint, defaults.get_copy(x));
    }
}

#[test]
fn test_treemap_invert() {
    let mut sample = TreeMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);
    let inverted = sample.invert();

    for x in vec!(1u, 2).iter() {
        assert!(inverted.contains_key(x));
        assert_eq!(*x as int, *inverted.find(x).unwrap());
    }
}

#[test]
fn test_treemap_pick() {
    let mut sample = TreeMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);
    let v = vec!(1i);
    let picked = sample.pick(&v);

    assert_eq!(1u, picked.len());
    assert_eq!(1u, *picked.find(&1i).unwrap());
}

#[test]
fn test_treemap_pick_by_filter() {
    let mut sample = TreeMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);
    let picked = sample.pick_by_filter(sample_filter);

    assert_eq!(1u, picked.len());
    assert_eq!(1u, *picked.find(&1i).unwrap());
}

#[test]
fn test_treemap_omit() {
    let mut sample = TreeMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);
    let omitted = sample.omit(&vec!(1i));

    assert_eq!(1u, omitted.len());
    assert_eq!(2u, *omitted.find(&2i).unwrap());
}

#[test]
fn test_treemap_omit_by_filter() {
    let mut sample = TreeMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);
    let omitted = sample.omit_by_filter(sample_filter);

    assert_eq!(1u, omitted.len());
    assert_eq!(2u, *omitted.find(&2i).unwrap());
}

#[test]
fn test_treemap_defaults() {
    let mut origin = TreeMap::new();
    origin.insert(1i, 1u);
    origin.insert(2i, 2u);

    let mut appends = TreeMap::new();
    appends.insert(1i, 10000u);
    appends.insert(3i, 3u);

    let defaults = origin.defaults(appends);
    for x in vec!(1i, 2, 3).iter() {
        assert!(defaults.contains_key(x));
        assert_eq!(*x as uint, *defaults.find(x).unwrap());
    }
}

#[test]
#[allow(deprecated)]
fn test_treemap_pairs() {
    let mut sample = TreeMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);
    sample.insert(3i, 3u);
    let mut pairs = sample.pairs();

    for i in vec!(1i, 2, 3).iter() {
        let element = pairs.shift().unwrap();
        assert_eq!(*i, *element.val0());
        assert_eq!(*i as uint, *element.val1());
    }
}
