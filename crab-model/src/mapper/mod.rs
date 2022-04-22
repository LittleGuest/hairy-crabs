pub mod gen_table;
pub mod sys_menu;
pub mod sys_user;

pub trait Mapper: Sized {
    fn insert(&self);
    fn update(&self);
    fn delete(&self);

    // fn one(id: i64) -> Self;
    // fn list(id: i64) -> Vec<Self>;
    // fn page(id: i64) -> Page<Self>;
}
