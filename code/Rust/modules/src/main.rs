mod foo;
mod sound;

fn main() {
    test_module::do_nothing();
    sound::instrument::clarinet();
    foo::foo();
    foo::test::test();
}

mod test_module {
    pub fn do_nothing() {}
}
