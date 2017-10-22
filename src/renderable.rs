use renderable_object::RenderableObject;
use game_object::GameObject;
use std::cell::RefCell;
use std::rc::Rc;
use std::mem;

pub trait Renderable: GameObject {
    fn get_renderable_object(&self) -> &RenderableObject;
}

// pub trait AsRenderable {
//     // fn as_renderable(&self) -> &Renderable;
//     fn as_renderable(renderable: &Rc<RefCell<T>>) -> Rc<RefCell<Renderable>>;
// }

// impl<T: Renderable> AsRenderable for T {
//     fn as_renderable(&self) -> &Renderable { self }
// }

// impl<T: Renderable + ?Sized> AsRenderable for T {
//     fn as_renderable(renderable: &Rc<RefCell<T>>) -> Rc<RefCell<Renderable>> { 
//         unsafe {
//             mem::transmute(renderable.clone())
//         } 
//     }
// }

// pub trait AsRefCellRenderable {
//     fn as_refcell_renderable(&self) -> &RefCell<Renderable>;
// }

// impl<T: Renderable + ?Sized> where Self: RefCell<T> {
//     fn as_refcell_renderable(&self) -> &RefCell<Renderable> { self }
// }

// impl<T: Renderable + ?Sized> AsRefCellRenderable for T {
//     fn as_refcell_renderable(&self) -> &RefCell<Renderable> { panic!() }
// }

// pub fn as_renderable<T: Renderable + ?Sized>(renderable: &Rc<RefCell<T>>) -> Rc<RefCell<Renderable>> {
//     unsafe {
//         mem::transmute::<Rc<RefCell<T>>, Rc<RefCell<Renderable>>>(renderable.clone())
//     }
// }
