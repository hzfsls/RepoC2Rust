pub fn string_nocase_compare(mut string1: Ptr<Void>, mut string2: Ptr<Void>) -> i32 {
    let mut p1: Ptr<u8> = string1.cast::<Ptr<u8>>();
    let mut p2: Ptr<u8> = string2.cast::<Ptr<u8>>();
    let mut c1: i32 = Default::default();
    let mut c2: i32 = Default::default();

    loop {
        c1 = c_tolower!(*p1).cast();
        c2 = c_tolower!(*p2).cast();

        if c1 != c2 {
            return if c1 < c2 { -1 } else { 1 };
        }

        if c1 == 0 {
            break;
        }

        p1 = p1 + 1;
        p2 = p2 + 1;
    }

    return 0;
}
