// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

//
//只通过重新排序'main（）'中的行来编译，但不添加、更改或删除任何行。只通过重新排序'main（）'中的行来编译，但不添加、更改或删除任何行。
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;

    *z += 1000;
    assert_eq!(x, 1200);
}
