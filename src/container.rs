use crate::Element;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Container: Element {
	fn children(&self) -> &Vec<Box<dyn Element>>;
	fn children_mut(&mut self) -> &mut Vec<Box<dyn Element>>;

	fn attach_child(&mut self, child: impl Element + 'static) where Self: Sized {
		self.attach_child_box(Box::new(child))
	}

	fn attach_child_box(&mut self, child: Box<dyn Element + 'static>) {
		self.append(&child);
		self.children_mut().push(child);
	}

	fn child(mut self, child: impl crate::Element + 'static) -> Self where Self: Sized {
		self.attach_child(child);
		self
	}

	fn child_ref(self, child: &(impl crate::Element + 'static)) -> Self where Self: Sized {
		self.element().append_child(&child.element()).expect("can't append child");
		self
	}
}

impl<T: Container> Container for Rc<RefCell<T>> {
	fn children(&self) -> &Vec<Box<dyn Element>> { unsafe { self.try_borrow_unguarded() }.expect("rc is mutably borrowed").children() }
	fn children_mut(&mut self) -> &mut Vec<Box<dyn Element>> { Rc::get_mut(self).expect("rc is mutably borrowed").get_mut().children_mut() }
}
