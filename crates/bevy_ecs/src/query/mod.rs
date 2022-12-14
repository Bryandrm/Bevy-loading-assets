mod access;
mod fetch;
mod filter;
mod iter;
mod state;

pub use access::*;
pub use fetch::*;
pub use filter::*;
pub use iter::*;
pub use state::*;

#[cfg(test)]
mod tests {
    use super::AnyOf;
    use crate::{self as bevy_ecs, component::Component, world::World};
    use std::collections::HashSet;

    #[derive(Component, Debug, Hash, Eq, PartialEq)]
    struct A(usize);
    #[derive(Component, Debug, Eq, PartialEq)]
    struct B(usize);
    #[derive(Component, Debug, Eq, PartialEq)]
    struct C(usize);

    #[derive(Component, Debug, Eq, PartialEq)]
    #[component(storage = "SparseSet")]
    struct Sparse(usize);

    #[test]
    fn query() {
        let mut world = World::new();
        world.spawn().insert_bundle((A(1), B(1)));
        world.spawn().insert_bundle((A(2),));
        let values = world.query::<&A>().iter(&world).collect::<Vec<&A>>();
        assert_eq!(values, vec![&A(1), &A(2)]);

        for (_a, mut b) in world.query::<(&A, &mut B)>().iter_mut(&mut world) {
            b.0 = 3;
        }
        let values = world.query::<&B>().iter(&world).collect::<Vec<&B>>();
        assert_eq!(values, vec![&B(3)]);
    }

    #[test]
    fn query_iter_combinations() {
        let mut world = World::new();

        world.spawn().insert_bundle((A(1), B(1)));
        world.spawn().insert_bundle((A(2),));
        world.spawn().insert_bundle((A(3),));
        world.spawn().insert_bundle((A(4),));

        let mut a_query = world.query::<&A>();
        assert_eq!(a_query.iter_combinations::<0>(&world).count(), 0);
        assert_eq!(
            a_query.iter_combinations::<0>(&world).size_hint(),
            (0, Some(0))
        );
        assert_eq!(a_query.iter_combinations::<1>(&world).count(), 4);
        assert_eq!(
            a_query.iter_combinations::<1>(&world).size_hint(),
            (0, Some(4))
        );
        assert_eq!(a_query.iter_combinations::<2>(&world).count(), 6);
        assert_eq!(
            a_query.iter_combinations::<2>(&world).size_hint(),
            (0, Some(6))
        );
        assert_eq!(a_query.iter_combinations::<3>(&world).count(), 4);
        assert_eq!(
            a_query.iter_combinations::<3>(&world).size_hint(),
            (0, Some(4))
        );
        assert_eq!(a_query.iter_combinations::<4>(&world).count(), 1);
        assert_eq!(
            a_query.iter_combinations::<4>(&world).size_hint(),
            (0, Some(1))
        );
        assert_eq!(a_query.iter_combinations::<5>(&world).count(), 0);
        assert_eq!(
            a_query.iter_combinations::<5>(&world).size_hint(),
            (0, Some(0))
        );
        assert_eq!(a_query.iter_combinations::<1024>(&world).count(), 0);
        assert_eq!(
            a_query.iter_combinations::<1024>(&world).size_hint(),
            (0, Some(0))
        );

        let values: Vec<[&A; 2]> = world.query::<&A>().iter_combinations(&world).collect();
        assert_eq!(
            values,
            vec![
                [&A(1), &A(2)],
                [&A(1), &A(3)],
                [&A(1), &A(4)],
                [&A(2), &A(3)],
                [&A(2), &A(4)],
                [&A(3), &A(4)],
            ]
        );
        let size = a_query.iter_combinations::<3>(&world).size_hint();
        assert_eq!(size.1, Some(4));
        let values: Vec<[&A; 3]> = a_query.iter_combinations(&world).collect();
        assert_eq!(
            values,
            vec![
                [&A(1), &A(2), &A(3)],
                [&A(1), &A(2), &A(4)],
                [&A(1), &A(3), &A(4)],
                [&A(2), &A(3), &A(4)],
            ]
        );

        let mut query = world.query::<&mut A>();
        let mut combinations = query.iter_combinations_mut(&mut world);
        while let Some([mut a, mut b, mut c]) = combinations.fetch_next() {
            a.0 += 10;
            b.0 += 100;
            c.0 += 1000;
        }

        let values: Vec<[&A; 3]> = a_query.iter_combinations(&world).collect();
        assert_eq!(
            values,
            vec![
                [&A(31), &A(212), &A(1203)],
                [&A(31), &A(212), &A(3004)],
                [&A(31), &A(1203), &A(3004)],
                [&A(212), &A(1203), &A(3004)]
            ]
        );

        let mut b_query = world.query::<&B>();
        assert_eq!(
            b_query.iter_combinations::<2>(&world).size_hint(),
            (0, Some(0))
        );
        let values: Vec<[&B; 2]> = b_query.iter_combinations(&world).collect();
        assert_eq!(values, Vec::<[&B; 2]>::new());
    }

    #[test]
    fn query_filtered_iter_combinations() {
        use bevy_ecs::query::{Added, Changed, Or, With, Without};

        let mut world = World::new();

        world.spawn().insert_bundle((A(1), B(1)));
        world.spawn().insert_bundle((A(2),));
        world.spawn().insert_bundle((A(3),));
        world.spawn().insert_bundle((A(4),));

        let mut a_query_with_b = world.query_filtered::<&A, With<B>>();
        assert_eq!(a_query_with_b.iter_combinations::<0>(&world).count(), 0);
        assert_eq!(
            a_query_with_b.iter_combinations::<0>(&world).size_hint(),
            (0, Some(0))
        );
        assert_eq!(a_query_with_b.iter_combinations::<1>(&world).count(), 1);
        assert_eq!(
            a_query_with_b.iter_combinations::<1>(&world).size_hint(),
            (0, Some(1))
        );
        assert_eq!(a_query_with_b.iter_combinations::<2>(&world).count(), 0);
        assert_eq!(
            a_query_with_b.iter_combinations::<2>(&world).size_hint(),
            (0, Some(0))
        );
        assert_eq!(a_query_with_b.iter_combinations::<3>(&world).count(), 0);
        assert_eq!(
            a_query_with_b.iter_combinations::<3>(&world).size_hint(),
            (0, Some(0))
        );
        assert_eq!(a_query_with_b.iter_combinations::<4>(&world).count(), 0);
        assert_eq!(
            a_query_with_b.iter_combinations::<4>(&world).size_hint(),
            (0, Some(0))
        );
        assert_eq!(a_query_with_b.iter_combinations::<5>(&world).count(), 0);
        assert_eq!(
            a_query_with_b.iter_combinations::<5>(&world).size_hint(),
            (0, Some(0))
        );
        assert_eq!(a_query_with_b.iter_combinations::<1024>(&world).count(), 0);
        assert_eq!(
            a_query_with_b.iter_combinations::<1024>(&world).size_hint(),
            (0, Some(0))
        );

        let mut a_query_without_b = world.query_filtered::<&A, Without<B>>();
        assert_eq!(a_query_without_b.iter_combinations::<0>(&world).count(), 0);
        assert_eq!(
            a_query_without_b.iter_combinations::<0>(&world).size_hint(),
            (0, Some(0))
        );
        assert_eq!(a_query_without_b.iter_combinations::<1>(&world).count(), 3);
        assert_eq!(
            a_query_without_b.iter_combinations::<1>(&world).size_hint(),
            (0, Some(3))
        );
        assert_eq!(a_query_without_b.iter_combinations::<2>(&world).count(), 3);
        assert_eq!(
            a_query_without_b.iter_combinations::<2>(&world).size_hint(),
            (0, Some(3))
        );
        assert_eq!(a_query_without_b.iter_combinations::<3>(&world).count(), 1);
        assert_eq!(
            a_query_without_b.iter_combinations::<3>(&world).size_hint(),
            (0, Some(1))
        );
        assert_eq!(a_query_without_b.iter_combinations::<4>(&world).count(), 0);
        assert_eq!(
            a_query_without_b.iter_combinations::<4>(&world).size_hint(),
            (0, Some(0))
        );
        assert_eq!(a_query_without_b.iter_combinations::<5>(&world).count(), 0);
        assert_eq!(
            a_query_without_b.iter_combinations::<5>(&world).size_hint(),
            (0, Some(0))
        );
        assert_eq!(
            a_query_without_b.iter_combinations::<1024>(&world).count(),
            0
        );
        assert_eq!(
            a_query_without_b
                .iter_combinations::<1024>(&world)
                .size_hint(),
            (0, Some(0))
        );

        let values: HashSet<[&A; 2]> = a_query_without_b.iter_combinations(&world).collect();
        assert_eq!(
            values,
            [[&A(2), &A(3)], [&A(2), &A(4)], [&A(3), &A(4)],]
                .into_iter()
                .collect::<HashSet<_>>()
        );

        let values: HashSet<[&A; 3]> = a_query_without_b.iter_combinations(&world).collect();
        assert_eq!(
            values,
            [[&A(2), &A(3), &A(4)],].into_iter().collect::<HashSet<_>>()
        );

        let mut query = world.query_filtered::<&A, Or<(With<A>, With<B>)>>();
        let values: HashSet<[&A; 2]> = query.iter_combinations(&world).collect();
        assert_eq!(
            values,
            [
                [&A(1), &A(2)],
                [&A(1), &A(3)],
                [&A(1), &A(4)],
                [&A(2), &A(3)],
                [&A(2), &A(4)],
                [&A(3), &A(4)],
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        );

        let mut query = world.query_filtered::<&mut A, Without<B>>();
        let mut combinations = query.iter_combinations_mut(&mut world);
        while let Some([mut a, mut b, mut c]) = combinations.fetch_next() {
            a.0 += 10;
            b.0 += 100;
            c.0 += 1000;
        }

        let values: HashSet<[&A; 3]> = a_query_without_b.iter_combinations(&world).collect();
        assert_eq!(
            values,
            [[&A(12), &A(103), &A(1004)],]
                .into_iter()
                .collect::<HashSet<_>>()
        );

        // Check if Added<T>, Changed<T> works
        let mut world = World::new();

        world.spawn().insert_bundle((A(1), B(1)));
        world.spawn().insert_bundle((A(2), B(2)));
        world.spawn().insert_bundle((A(3), B(3)));
        world.spawn().insert_bundle((A(4), B(4)));

        let mut query_added = world.query_filtered::<&A, Added<A>>();

        world.clear_trackers();
        world.spawn().insert_bundle((A(5),));

        assert_eq!(query_added.iter_combinations::<2>(&world).count(), 0);

        world.clear_trackers();
        world.spawn().insert_bundle((A(6),));
        world.spawn().insert_bundle((A(7),));

        assert_eq!(query_added.iter_combinations::<2>(&world).count(), 1);

        world.clear_trackers();
        world.spawn().insert_bundle((A(8),));
        world.spawn().insert_bundle((A(9),));
        world.spawn().insert_bundle((A(10),));

        assert_eq!(query_added.iter_combinations::<2>(&world).count(), 3);

        world.clear_trackers();

        let mut query_changed = world.query_filtered::<&A, Changed<A>>();

        let mut query = world.query_filtered::<&mut A, With<B>>();
        let mut combinations = query.iter_combinations_mut(&mut world);
        while let Some([mut a, mut b, mut c]) = combinations.fetch_next() {
            a.0 += 10;
            b.0 += 100;
            c.0 += 1000;
        }

        let values: HashSet<[&A; 3]> = query_changed.iter_combinations(&world).collect();
        assert_eq!(
            values,
            [
                [&A(31), &A(212), &A(1203)],
                [&A(31), &A(212), &A(3004)],
                [&A(31), &A(1203), &A(3004)],
                [&A(212), &A(1203), &A(3004)]
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn query_iter_combinations_sparse() {
        let mut world = World::new();

        world.spawn_batch((1..=4).map(|i| (Sparse(i),)));

        let mut query = world.query::<&mut Sparse>();
        let mut combinations = query.iter_combinations_mut(&mut world);
        while let Some([mut a, mut b, mut c]) = combinations.fetch_next() {
            a.0 += 10;
            b.0 += 100;
            c.0 += 1000;
        }

        let mut query = world.query::<&Sparse>();
        let values: Vec<[&Sparse; 3]> = query.iter_combinations(&world).collect();
        assert_eq!(
            values,
            vec![
                [&Sparse(31), &Sparse(212), &Sparse(1203)],
                [&Sparse(31), &Sparse(212), &Sparse(3004)],
                [&Sparse(31), &Sparse(1203), &Sparse(3004)],
                [&Sparse(212), &Sparse(1203), &Sparse(3004)]
            ]
        );
    }

    #[test]
    fn multi_storage_query() {
        let mut world = World::new();

        world.spawn().insert_bundle((Sparse(1), B(2)));
        world.spawn().insert_bundle((Sparse(2),));

        let values = world
            .query::<&Sparse>()
            .iter(&world)
            .collect::<Vec<&Sparse>>();
        assert_eq!(values, vec![&Sparse(1), &Sparse(2)]);

        for (_a, mut b) in world.query::<(&Sparse, &mut B)>().iter_mut(&mut world) {
            b.0 = 3;
        }

        let values = world.query::<&B>().iter(&world).collect::<Vec<&B>>();
        assert_eq!(values, vec![&B(3)]);
    }

    #[test]
    fn any_query() {
        let mut world = World::new();

        world.spawn().insert_bundle((A(1), B(2)));
        world.spawn().insert_bundle((A(2),));
        world.spawn().insert_bundle((C(3),));

        let values: Vec<(Option<&A>, Option<&B>)> =
            world.query::<AnyOf<(&A, &B)>>().iter(&world).collect();

        assert_eq!(
            values,
            vec![(Some(&A(1)), Some(&B(2))), (Some(&A(2)), None),]
        );
    }
}
