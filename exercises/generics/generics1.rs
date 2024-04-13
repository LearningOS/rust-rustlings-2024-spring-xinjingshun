// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn main() {

    fn append_item <T>(vec:&mut Vec<T>, item: T){
        vec.push(item);
    }
    let mut shopping_list:Vec<&str> = Vec::new();
    append_item(&mut shopping_list,"milk");
    println!("{:?}",shopping_list)
}
