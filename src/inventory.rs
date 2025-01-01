//use crate::itemid::ItemID;
//
//pub struct ItemType {
//    name: String,
//    description: String,
//    pub use_action: Box<dyn FnMut(&mut Self)>,
//}
//
//pub struct ItemStack {
//    item: ItemType,
//    num: u8,
//}
//
//pub struct Inventory {
//    items: [ItemStack; 12],
//}
//
//impl ItemType {
//    pub fn new(name: &str) -> Self {
//        Self {
//            name: name.into(),
//            description: format!(""),
//            use_action: Box::new(|item: &mut ItemType| {
//                println!("{} was used. DEFAULT ACTION", item.name);
//            }),
//        }
//    }
//
//    fn set_description(&mut self, desc: &str) -> &mut Self {
//        self.description = format!("{}", desc);
//        self
//    }
//
//    fn use_item(&mut self) {
//    }
//
//    pub fn set_use_action<F>(&mut self, new_action: F) -> &mut Self
//    where 
//        F: FnMut(&mut Self) + 'static,
//    {
//        self.use_action = Box::new(new_action);
//        self
//    }
//}
//
//impl Inventory {
//    pub fn new() -> Self {
//        Self {
//            items: [(()); 12],
//        }
//    }
//}
