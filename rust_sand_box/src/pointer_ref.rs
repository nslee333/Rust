// Reference pointers - Point to a resource in memory.

pub fn run() {
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value.
    // You'll need to use a reference (&) to point to the resource.

    // Vector is a non-primitive.
    let vec1 = vec![1, 2, 3]; // This will throw an error
    let vec2 = &vec1;

    // We can't just reference a non-primitive type with a variable, we have to get a reference to it in order for that to work.
    println!("Values: {:?}", (&vec1, vec2));
}