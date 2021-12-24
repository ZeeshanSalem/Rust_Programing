struct Point<T,U>{
    x : T,
    y : U,
}

impl<T,U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V,W>) -> Point<T,W>{
        Point{
            x : self.x,
            y : other.y,
        }
    }
}


fn main() {

    // This is code to find largest now we to make code more optmise and generic let make function
    let number_list = vec![34, 50,25, 100,65];

    let  largest = get_largets(number_list);

    println!("The Larger Number is {}", largest);


    // Note here can do same for largest char for this we two method 
    // first we create another function for but still code is not to much optimize
    // for this we need to combine a function


    let char_list = vec!['y', 'm', 'a', 'q'];
    let  largest = get_largets(char_list);

    println!("The Larger Number is {}", largest);


    // Now let MAKE Generic Example of Struct
    let p1 = Point{x:5, y: 10.3};
    let p2 = Point{x: "Hello", y : 'x'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


}

// Now this function is for Finding largest number this function for specific largest number
// fn get_largets(number_list : Vec<i32>) -> i32{
//     let mut largest = number_list[0];

//     for number in  number_list{
//         if number > largest {
//             largest  = number;
//         }
//     }

//     largest
// }
 
// NOW THIS FUNCTION is generic
fn get_largets<T: PartialOrd + Copy>(number_list : Vec<T>) -> T{
    let mut largest = number_list[0];

    for number in  number_list{
        if number > largest {
            largest  = number;
        }
    }

    largest
}