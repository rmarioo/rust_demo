# Ownership Rules
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

## Benefits 
- **Avoid need to have a GC** to regularly looks for no-longer used memory 
- **Avoid dangling pointers** at compile time
- **Avoid prevent data races** at at compile time
- **Ensure memory safety** mostly at compile time

# Examples   

## Value borrowed by a function after a move : example 1  
The following code _**does not compile**_ because the value BARCELONA is moved from X to Y and after that x is used

```rust
#[test]
fn  ownership_test() {

    let  x = "BARCELONA".to_string();
    let  y = x;

    let res = format!("word {} has lenght {}", x, x.len());
                                            // ^ value borrowed here after move

    assert_eq!(res,"word BARCELONA has lenght 9")
}

```
![Rust_Borrowing](rust_borrowing.png)

## Value borrowed by a function after a move : example 2

```rust
    #[test]
    fn count_words_in_string() {  
      
        let input = String::from("first second");
        let num: usize = words_in(input);

        println!("number of work in {} is {}", input, num);
                                          //   ^^^^^ value borrowed here after move
        assert_eq!(num, 2);
    }

    fn words_in(input: String) -> usize {
        let split = input.split(" ");
        split.count()
    }
```

## Value borrowed by a function after a move : example 3

```rust

    struct Product {
        name: String,
        productType: ProductType
    }

    #[derive(PartialEq,Clone)]
    enum ProductType {
        FRUIT,
        DRINK
    }


    #[test]
    fn  count_product_by_type() {
        let products = vec![
            Product { name: "apple".to_string(), productType: FRUIT },
            Product { name: "pear".to_string() , productType: FRUIT },
            Product { name: "mojito".to_string(), productType: DRINK }
        ];

        let  fruits = count_fruits(products);
                         //        -------- value moved here
        let  drinks = count_drinks(products);
                         //        ^^^^^^^^ value used here after move

        assert_eq!(fruits,2);
        assert_eq!(drinks,1);

    }

```


# References to the rescue!
Let’s recap what we’ve discussed about references:

1. At any given time, you can have either one mutable reference or any number of immutable references.  
2. References must always be valid.  
