#![allow(dead_code)]
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate entity_store_helper;
#[macro_use] extern crate maplit;
extern crate entity_store_code_gen;
extern crate cgmath;
extern crate direction;

macro_rules! include_spec {
    ($spec:expr) => {
        include_str!(concat!("../specs/", $spec))
    }
}

#[cfg(test)]
mod tests {
    use cgmath::Vector2;
    use entity_store_code_gen::*;
    use direction::Direction;
    mod simple { include_entity_store!("simple.rs"); }
    mod all_aggregates { include_entity_store!("all_aggregates.rs"); }

    #[test]
    fn empty() {
        match GeneratedCode::generate(include_spec!("empty.toml")) {
            Err(GenError::NoComponents) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn simple() {
        use self::simple::*;
        let mut store = EntityStore::new();
        let c = insert::flag(0);
        store.commit(c);
        assert!(store.flag.contains(&0));
    }

    #[test]
    fn all_aggregates() {
        use self::all_aggregates::*;
        let mut store = EntityStore::new();
        let mut sh = SpatialHashTable::new(4, 6);

        let mut changes = vec![
            insert::position(0, Vector2::new(1, 1)),
            insert::value(0, 4),
        ];

        for c in changes.drain(..) {
            sh.update(&store, &c, 1);
            store.commit(c);
        }

        assert_eq!(sh.get(Vector2::new(1, 1)).unwrap().count, 1);
        assert_eq!(sh.get(Vector2::new(1, 1)).unwrap().total, 4);
        assert_eq!(sh.get(Vector2::new(1, 1)).unwrap().set, hashset!{ 0 });
        assert_eq!(sh.get(Vector2::new(1, 2)).unwrap().neighbour_count.get(Direction::North), 1);
        assert_eq!(sh.get(Vector2::new(0, 0)).unwrap().neighbour_count.get(Direction::SouthEast), 1);
        assert_eq!(sh.get(Vector2::new(1, 1)).unwrap().last_updated, 1);

        let mut changes = vec![
            insert::position(1, Vector2::new(1, 1)),
            insert::void(1),
        ];

        for c in changes.drain(..) {
            sh.update(&store, &c, 2);
            store.commit(c);
        }

        assert_eq!(sh.get(Vector2::new(1, 1)).unwrap().last_updated, 2);

        let mut changes = vec![
            insert::position(2, Vector2::new(1, 1)),
            insert::value(2, 3),
        ];

        for c in changes.drain(..) {
            sh.update(&store, &c, 3);
            store.commit(c);
        }

        assert_eq!(sh.get(Vector2::new(1, 1)).unwrap().count, 2);
        assert_eq!(sh.get(Vector2::new(1, 1)).unwrap().total, 7);
        assert_eq!(sh.get(Vector2::new(1, 1)).unwrap().set, hashset!{ 0, 2 });
        assert_eq!(sh.get(Vector2::new(1, 2)).unwrap().neighbour_count.get(Direction::North), 2);
        assert_eq!(sh.get(Vector2::new(0, 0)).unwrap().neighbour_count.get(Direction::SouthEast), 2);
        assert_eq!(sh.get(Vector2::new(1, 1)).unwrap().last_updated, 3);

        let change = insert::position(0, Vector2::new(0, 1));
        sh.update(&store, &change, 4);
        store.commit(change);

        assert_eq!(sh.get(Vector2::new(1, 1)).unwrap().count, 1);
        assert_eq!(sh.get(Vector2::new(1, 1)).unwrap().total, 3);
        assert_eq!(sh.get(Vector2::new(1, 1)).unwrap().set, hashset!{ 2 });
        assert_eq!(sh.get(Vector2::new(1, 2)).unwrap().neighbour_count.get(Direction::North), 1);
        assert_eq!(sh.get(Vector2::new(0, 0)).unwrap().neighbour_count.get(Direction::SouthEast), 1);
        assert_eq!(sh.get(Vector2::new(1, 1)).unwrap().last_updated, 4);

        assert_eq!(sh.get(Vector2::new(0, 1)).unwrap().count, 1);
        assert_eq!(sh.get(Vector2::new(0, 1)).unwrap().total, 4);
        assert_eq!(sh.get(Vector2::new(0, 1)).unwrap().set, hashset!{ 0 });
        assert_eq!(sh.get(Vector2::new(0, 2)).unwrap().neighbour_count.get(Direction::North), 1);
        assert_eq!(sh.get(Vector2::new(1, 0)).unwrap().neighbour_count.get(Direction::SouthWest), 1);
        assert_eq!(sh.get(Vector2::new(0, 1)).unwrap().last_updated, 4);
    }
}
