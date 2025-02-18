# Control Flow
The ability to run some code depending on whether a condition is `true` and to run some code repeatedly while a condition is `true` are basic building blocks in most programming languages. The most common constructs that let you control the flow of execution of Rust code are if expressions and loops.

## if Expressions
An `if` expression allows you to branch your code depending on conditions. You provide a condition and then state, 

*“If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”*

Create a new project called `branches` in your projects directory to explore the if expression. In the `src/main.rs` file, input the following:

*Filename: src/main.rs*

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

- All `if` expressions start with the keyword `if`, followed by a condition. In this case, the condition checks whether or not the variable number has a value less than 5. We place the block of code to execute if the condition is `true` immediately after the condition inside curly brackets. Blocks of code associated with the conditions in `if` expressions are sometimes called `arms`.

- Optionally, we can also include an `else` expression, which we chose to do here, to give the program an alternative block of code to execute should the condition evaluate to false. If you don’t provide an `else` expression and the condition is false, the program will just skip the if block and move on to the next bit of code.

Try running this code; you should see the following output:
```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
condition was true
```
Let’s try changing the value of number to a value that makes the condition `false` to see what happens:
```rust
    let number = 7;
```
Run the program again, and look at the output:
```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
condition was false
```
It’s also worth noting that the condition in this code must be a `bool`. If the condition isn’t a `bool`, we’ll get an error. For example, try running the following code:

*Filename: src/main.rs*

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```
The `if` condition evaluates to a value of 3 this time, and Rust throws an error:
```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` (bin "branches") due to 1 previous error
```
 The error indicates that Rust expected a bool but got an integer. Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide if with a Boolean as its condition. If we want the if code block to run only when a number is not equal to 0, for example, we can change the if expression to the following:

*Filename: src/main.rs*
```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```
Running this code will print *"number was something other than zero"*.

## Handling Multiple Conditions with else if
You can use multiple conditions by combining `if` and `else` in an` else if` expression. For example:

*Filename: src/main.rs*

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```
This program has four possible paths it can take. After running it, you should see the following output:
```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
number is divisible by 3
```
- When this program executes, it checks each if expression in turn and executes the first body for which the condition evaluates to `true`.

- Note that even though 6 is divisible by 2, we don’t see the output number is divisible by 2, nor do we see the number is not divisible by 4, 3, or 2 text from the else block. That’s because Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.

> [!NOTE]  
> Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code.

## Using if in a let Statement
Because `if` is an `expression`, we can use it on the right side of a `let` statement to assign the outcome to a variable, as in *Listing 3-2*.

*Filename: src/main.rs*
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```
*Listing 3-2: Assigning the result of an if expression to a variable*

The number variable will be bound to a value based on the outcome of the `if` expression. Run this code to see what happens:
```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/branches`
The value of number is: 5
```
Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions. In this case, the value of the whole if expression depends on which block of code executes. **This means the values that have the potential to be results from each arm of the if must be the same type;** in *Listing 3-2*, the results of both the if arm and the else arm were `i32` integers. If the types are mismatched, as in the following example, we’ll get an error:

`Filename: src/main.rs`
```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
```
When we try to compile this code, we’ll get an error. The if and else arms have value types that are incompatible, and Rust indicates exactly where to find the problem in the program:
```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` (bin "branches") due to 1 previous error
```
The expression in the `if` block evaluates to an integer, and the expression in the `else` block evaluates to a `string`. This won’t work because variables must have a single type, and Rust needs to know at compile time what type the number variable is, definitively. Knowing the type of number lets the compiler verify the type is valid everywhere we use number. Rust wouldn’t be able to do that if the type of number was only determined at runtime; the compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.

# Repetition with Loops
It’s often useful to execute a block of code more than once. For this task, Rust provides several loops, which will run through the code inside the loop body to the end and then start immediately back at the beginning. To experiment with loops, let’s make a new project called `loops`.

Rust has three kinds of loops:
- loop

- while

- for.


## Repeating Code with loop
The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

As an example, change the` src/main.rs` file in your loops directory to look like this:

*Filename: src/main.rs*
```rust
fn main() {
    loop {
        println!("again!");
    }
}
```
When we run this program, we’ll see again! printed over and over continuously until we stop the program manually. Most terminals support the keyboard shortcut` ctrl-c` to interrupt a program that is stuck in a continual loop. Give it a try:
```
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```
The symbol `^C` represents where you pressed` ctrl-c`. You may or may not see the word again! printed after the `^C`, depending on where the code was in the loop when it received the interrupt signal.

Fortunately, Rust also provides a way to break out of a loop using code. You can place the `break` keyword within the loop to tell the program when to stop executing the loop. 

> [!TIP]  
> The `continue` keyword in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

## Returning Values from Loops
One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. You might also need to pass the result of that operation out of the loop to the rest of your code. To do this, you can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it, as shown here:
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```
- Before the loop, we declare a variable named `counter` and initialize it to `0`.

- Then we declare a variable named `result` to hold the value returned from the loop.

- On every iteration of the loop, we add `1` to the counter variable, and then check whether the counter is equal to `10`.

- When it is, we use the `break` keyword with the value `counter * 2`.

- After the loop, we use a semicolon to end the statement that assigns the value to result. 

- Finally, we print the value in result, which in this case is `20`.

You can also return from inside a loop. While break only exits the current loop, return always exits the current function.

## Loop Labels to Disambiguate Between Multiple Loops
If you have loops within loops, `break` and `continue` apply to the innermost loop at that point. You can optionally specify a loop label on a loop that you can then use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote. Here’s an example with two nested loops:

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```
The outer loop has the label `'counting_up`, and it will count up from `0` to `2`. The inner loop without a label counts down from `10` to `9`. The first break that doesn’t specify a label will exit the inner loop only. The break `'counting_up;` statement will exit the outer loop. This code prints:
```
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/loops`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```
## Conditional Loops with while
A program will often need to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program calls break, stopping the loop. It’s possible to implement behavior like this using a combination of loop, `if`, `else`, and `break`; you could try that now in a program, if you’d like. 

However, this pattern is so common that Rust has a built-in language construct for it, called a `while` loop. In *Listing 3-3*, we use `while` to loop the program three times, counting down each time, and then, after the loop, print a message and exit.

*Filename: src/main.rs*
```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```
*Listing 3-3: Using a while loop to run code while a condition holds true*

This construct eliminates a lot of nesting that would be necessary if you used `loop`, `if`, `else`, and `break`, and it’s clearer. While a condition evaluates to true, the code runs; otherwise, it exits the loop.

## Looping Through a Collection with for
You can also use the `while` construct to loop over the elements of a collection, such as an `array`. For example, the loop in *Listing 3-4* prints each element in the `array a`.

*Filename: src/main.rs*
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```
*Listing 3-4: Looping through each element of a collection using a while loop*

Here, the code counts up through the elements in the array. It starts at index 0, and then loops until it reaches the final index in the array (that is, when index < 5 is no longer true). Running this code will print every element in the array:
```
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```
All five array values appear in the terminal, as expected. Even though index will reach a value of 5 at some point, the loop stops executing before trying to fetch a sixth value from the array.

> [!NOTE]  
> However, this approach is error prone; we could cause the program to panic if the index value or test condition is incorrect. 
<br/>  
> For example, if you changed the definition of the a array to have four elements but forgot to update the condition to while `index < 4`, the code would panic. It’s also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.
<br/>  
> As a more concise alternative, you can use a for loop and execute some code for each item in a collection. A for loop looks like the code in *Listing 3-5*.

*Filename: src/main.rs*
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```
*Listing 3-5: Looping through each element of a collection using a for loop*

When we run this code, we’ll see the same output as in *Listing 3-4*. More importantly, we’ve now increased the safety of the code and eliminated the chance of bugs that might result from going beyond the end of the array or not going far enough and missing some items.

Using the `for` loop, you wouldn’t need to remember to change any other code if you changed the number of values in the array, as you would with the method used in *Listing 3-4*.

Here’s what the countdown would look like using a `for` loop and another method we’ve not yet talked about, `rev`, to reverse the range:

*Filename: src/main.rs*
```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```


# Summary
You learned about `variables`, `scalar` and `compound data types`, `functions`, `comments`, `if` expressions, and `loops`! To practice with the concepts discussed in this chapter, try building programs to do the following:

- Convert temperatures between Fahrenheit and Celsius.

- Generate the nth Fibonacci number.

- Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.


