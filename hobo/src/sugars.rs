// https://veykril.github.io/tlborm/decl-macros/building-blocks/counting.html#bit-twiddling
#[macro_export]
macro_rules! count {
	() => { 0 };
	($odd:tt $($a:tt $b:tt)*) => { ($crate::count!($($a)*) << 1) | 1 };
	($($a:tt $even:tt)*) => { $crate::count!($($a)*) << 1 };
}

#[macro_export]
macro_rules! hmap {
	() => { ::std::collections::HashMap::new() };
	($($key:expr => $value:expr),+ $(,)?) => {{
		let mut map = ::std::collections::HashMap::with_capacity($crate::count!($($key)*));
		$(let _ = map.insert($key, $value);)+
		map
	}};
}

#[macro_export]
macro_rules! hset {
	() => { ::std::collections::HashSet::new() };
	($($value:expr),+ $(,)?) => {{
		let mut set = ::std::collections::HashSet::with_capacity($crate::count!($($value)*));
		$(let _ = set.insert($value);)+
		set
	}};
}

#[macro_export]
macro_rules! hash {
	($value:expr) => {{
		use ::std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };
        let mut hasher = DefaultHasher::new();
        $value.hash(&mut hasher);
        hasher.finish()
	}};
}
