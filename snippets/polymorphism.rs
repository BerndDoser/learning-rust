// Source: https://oswalt.dev/2021/06/polymorphism-in-rust/

trait Growler {
    fn growl(&self);
}

struct Lion;
impl Growler for Lion {
    #[inline(never)]
    fn growl(&self) {
        println!("Lion says GROWL!");
    }
}
struct Tiger;
impl Growler for Tiger {
    #[inline(never)]
    fn growl(&self) {
        println!("Tiger says GROWL!");
    }
}
struct Bear;
impl Growler for Bear {
    #[inline(never)]
    fn growl(&self) {
        println!("Bear says GROWL!");
    }
}

pub fn main() {
    static_dispatch(Lion{});
    static_dispatch(Tiger{});
    static_dispatch(Bear{});

    dynamic_dispatch(&Lion{});
    dynamic_dispatch(&Tiger{});
    dynamic_dispatch(&Bear{});

    let mut v: Vec<Box<dyn Growler>> = Vec::new();
    v.push(Box::new(Lion{}));
    v.push(Box::new(Bear{}));
    v[1].growl();
}

// This function only shows up once in the resulting assembly,
// because it uses dynamic dispatch to call the correct implementation
// of Growler
fn dynamic_dispatch(t: &dyn Growler) {
    t.growl();
}

// This function shows up three times, since we're using Generics
// here, and the compiler monomorphizes this function as many
// times as its called with a different concrete type
fn static_dispatch<T: Growler>(t: T) {
    t.growl();
}


