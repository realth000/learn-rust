use crate::modules::learn_mod::m2;

pub fn m1() {
    m2::m21::m21_pub();
    // 在m2/mod.rs里，以pub use的形式pub了m22_private_public();
    // 虽然m22在m2/mod.rs里不是pub的，但是m22_private_public依然可用，
    // 而且看上去这个函数就相当于定义在m2里。
    m2::m22_private_public();
}
