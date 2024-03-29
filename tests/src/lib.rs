#[cfg(test)]
mod tests {
    use bundle::bundle;
    use tiny_serde::prelude::*;
    use tiny_serde::Serialize;

    trait Foo {
        fn bar(self) -> u8;
    }

    #[derive(Clone)]
    struct A;
    #[derive(Clone, Debug, PartialEq)]
    struct B;
    #[derive(Clone)]
    struct C;

    impl Foo for A {
        fn bar(self) -> u8 {
            0
        }
    }

    impl Foo for B {
        fn bar(self) -> u8 {
            1
        }
    }

    impl Foo for C {
        fn bar(self) -> u8 {
            2
        }
    }

    #[test]
    fn basic() {
        #[bundle]
        enum MyBundle {
            A,
            B,
            C,
        }

        let bundle = MyBundle::B(B);

        assert_eq!(use_my_bundle!(bundle, |inner| { inner.bar() }), 1);
    }

    #[test]
    fn impls() {
        #[bundle]
        enum MyBundle {
            A,
            B,
            C,
        }

        // if you want the bundle itself to
        // implement the common trait as well
        // so the use macro is not needed elsewhere,
        // you can do so like this
        impl Foo for MyBundle {
            fn bar(self) -> u8 {
                use_my_bundle!(self, |inner| { inner.bar() })
            }
        }

        let bundle = MyBundle::A(A);

        assert_eq!(bundle.bar(), 0);
    }

    #[test]
    fn derive() {
        #[bundle]
        #[derive(Clone)]
        enum MyBundle {
            A,
            B,
            C,
        }

        let bundle = MyBundle::C(C);

        assert_eq!(use_my_bundle!(bundle, |inner| { inner.bar() }), 2);
    }

    #[test]
    fn tiny_serde() {
        #[derive(Serialize)]
        struct A {
            val: u8,
        }

        #[derive(Serialize)]
        struct B {
            val: u16,
        }

        #[derive(Serialize)]
        struct C {
            val: u8,
            other: A,
        }

        impl Foo for A {
            fn bar(self) -> u8 {
                self.val
            }
        }

        impl Foo for B {
            fn bar(self) -> u8 {
                self.val as u8
            }
        }

        impl Foo for C {
            fn bar(self) -> u8 {
                self.val + self.other.val
            }
        }

        #[bundle]
        #[derive(Serialize)]
        #[repr(u8)]
        enum MyBundle {
            A,
            B,
            C = 0x10,
        }

        let buf = MyBundle::C(C {
            val: 15,
            other: A { val: 20 },
        })
        .serialize();

        assert_eq!(buf, [0x10, 15, 20]);
    }

    #[test]
    fn generics() {
        trait Foo {}
        trait Bar {}

        impl Foo for u8 {}
        impl Bar for u8 {}

        struct A<T: Bar> {
            #[allow(unused)]
            val: T,
        }
        struct B;
        struct C<T: Foo> {
            #[allow(unused)]
            val: T,
        }

        impl<T: Bar> Foo for A<T> {}
        impl Foo for B {}
        impl<T: Foo> Foo for C<T> {}

        #[bundle]
        enum MyBundle<T: Bar, U: Foo> {
            A(A<T>),
            B,
            C(C<U>),
        }

        let _bundle: MyBundle<_, B> = MyBundle::A(A { val: 0u8 });
        let _bundle: MyBundle<u8, _> = MyBundle::C(C { val: B });
    }
}
