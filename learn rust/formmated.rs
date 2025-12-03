fn main(){
    // in genral we use {} as a placeholder will be automatically replaced with any arguments we provide
    println!("{} days", 31);
    // positional arguments can be used multiple times. 
    // Specifying an integer inside the curly brackets{} selects which argument to place there.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // as can named arguments be used
    println!("{subject} {verb} {object}",
        object="the lazy dog", subject="the quick brown fox", verb="jumps over");   
        
    // Differnet formating can be specified by appending a colon (:) 
    // after the placeholder and then the format specifier
    println!("Base 10:  {}", 69420);
    println!("Base 2:   {:b}", 69420);
    println!("Base 8:   {:o}", 69420);
    println!("Base 16:  {:x}", 69420);
    println!("Base 16:  {:X}", 69420);

    // Text with a specified width
    println!("{number:>5}", number=1); // right-aligns number in a space of 5 characters
    println!("{number:0>5}", number=1); // pads with zeros instead of spaces
    println!("{number:0<5}", number=1); // left-aligns number in a space of 5 characters
    println!("{number:^5}", number=1); // centers number in a space of 5 characters
    // Rust even checks to make sure the correct number of arguments are used.
    // The next line would cause a compile-time error due to missing arguments
    println!("My name is {0}, {1} {0}", "James", "Bond");

    let  number: f64 = 1.0;
    let width: usize = 6;
    println!("{number:>width$}", number=number, width=width);

//    named parameters
    format!("{argument}", argument = "test");   // => "test"
    format!("{name} {}", 1, name = 2);          // => "2 1"
    format!("{a} {c} {b}", a="a", b='b', c=3);  // => "a 3 b" 
}   