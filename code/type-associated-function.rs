#[derive(Debug)]
pub Trait Foo {
    fn bar() -> Self;
}

#[derive(Debug)]
pub struct Baz;

impl Foo for Baz {
    fn bar() -> Baz {
        Baz
    }
}

fn print_bar(foo: &dyn Foo) {
    println!("{:?}", foo.bar());

}

fn main() {
    let baz = Baz::bar();
    print_foo(&baz);
}
