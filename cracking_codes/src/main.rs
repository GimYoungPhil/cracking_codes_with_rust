use chapter_00;
// use chapter_01;
// use chapter_02;

fn main() {
    // let message = "There can keep a secret, if two of them are dead.";
    // println!("{}", chapter_01::reverse_chiper(message));
    // chapter_02::test_message(message);

    // chapter_00::euclid(127);
    // chapter_00::euclid(1);
    // chapter_00::euclid(0);
    // chapter_00::euclid(-1);
    // chapter_00::euclid(-127);
    // chapter_00::euclid(-128);
    // chapter_00::display_rem(5);

    // let integers: [(i8, i8); 4] = [
    //     (11_i8, 26_i8),
    //     (11_i8, -26_i8),
    //     (-11_i8, 26_i8),
    //     (-11_i8, -26_i8),
    // ];
    let integers: [(i8, i8); 4] = [
        (i8::MAX, 1_i8),
        (i8::MAX, -1_i8),
        (i8::MIN, 1_i8),
        (i8::MIN, -1_i8),
    ];
    // chapter_00::display_div(&integers);
    // chapter_00::display_div_euclid(&integers);
    chapter_00::display_checked_div(&integers);
    chapter_00::display_checked_div_euclid(&integers);
}
