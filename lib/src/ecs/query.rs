use super::*;

pub trait BasicQuery: 'static {
	fn exists(world: &World, entity: Entity) -> bool;
	fn added(world: &World, entity: Entity) -> bool;
	fn modified(world: &World, entity: Entity) -> bool;
	fn removed(world: &World, entity: Entity) -> bool;
}

pub trait Query {
	fn query(world: &World, entity: Entity) -> bool;
}

pub trait SubQuery {
	fn query(world: &World, entity: Entity) -> bool;
}

// Added<(T1, T2, T3)> implies that one of T1, T2, T3 was added
// the use-case of an archetype that was just entered would be Query<(Added<(T1, T2, T3)>, (T1, T2, T3))>
pub struct Added<T: BasicQuery>(PhantomData<T>);
// Removed<(T1, T2, T3)> implies that at least one of T1, T2, T3 was removed
// the use-case of an archetype having been left could be Query<(Removed<(T1, T2, T3)>)>
pub struct Removed<T: BasicQuery>(PhantomData<T>);
// Modified<(T1, T2, T3)> implies that one of T1, T2, T3 was changed
pub struct Modified<T: BasicQuery>(PhantomData<T>);

impl<T: 'static> BasicQuery for T {
	fn exists(world: &World, entity: Entity) -> bool {
		world.storage::<Self>().has(entity)
	}

	fn added(world: &World, entity: Entity) -> bool {
		world.storage::<Self>().added.contains(&entity)
	}

	fn modified(world: &World, entity: Entity) -> bool {
		world.storage::<Self>().modified.contains(&entity)
	}

	fn removed(world: &World, entity: Entity) -> bool {
		world.storage::<Self>().removed.contains_key(&entity)
	}
}

macro_rules! tuple_query {
	() => {};
	($first:ident $($id:ident)*) => {
		paste::item! {
			impl<$first: SubQuery, $($id: SubQuery),*> Query for ($first, $($id),*) {
				fn query(world: &World, entity: Entity) -> bool {
					$first::query(world, entity)
					$(&& $id::query(world, entity))*
				}
			}

			impl<$first: BasicQuery, $($id: BasicQuery),*> SubQuery for ($first, $($id),*) {
				fn query(world: &World, entity: Entity) -> bool {
					$first::exists(world, entity)
					$(&& $id::exists(world, entity))*
				}
			}

			impl<$first: BasicQuery, $($id: BasicQuery),*> SubQuery for Added<($first, $($id),*)> {
				fn query(world: &World, entity: Entity) -> bool {
					$first::added(world, entity)
					$(|| $id::added(world, entity))*
				}
			}

			impl<$first: BasicQuery, $($id: BasicQuery),*> SubQuery for Modified<($first, $($id),*)> {
				fn query(world: &World, entity: Entity) -> bool {
					$first::modified(world, entity)
					$(|| $id::modified(world, entity))*
				}
			}

			impl<$first: BasicQuery, $($id: BasicQuery),*> SubQuery for Removed<($first, $($id),*)> {
				fn query(world: &World, entity: Entity) -> bool {
					// total - bitmask with 1s for every component queried
					// present - bitmask with 1s for every queried component that exists
					// missing - bitmask with 1s for every queried component that is marked as removed

					let mut total: u32;
					let mut present: u32;
					let mut missing: u32;

					total = 1;
					present = if $first::exists(world, entity) { 1 } else { 0 };
					missing = if $first::removed(world, entity) { 1 } else { 0 };

					$(
						total = (total << 1) + 1;
						present = (present << 1) + if $id::exists(world, entity) { 1 } else { 0 };
						missing = (missing << 1) + if $id::removed(world, entity) { 1 } else { 0 };
					)*

					(present != total) && ((present | missing) == total)
				}
			}
		}
		tuple_query! {$($id)*}
	};
}

tuple_query! {A B C D E F G H I J K L M N O P Q R S T U V W X Y Z}