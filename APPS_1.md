# Applications

## Word frequency count

1. We have file with a single word per line. Read the file.
1. Use a `HashMap` to count the number of times each word appears.
1. Print the words and their appearance in 2 columns: first the number of occurrences, then the word itself
1. Change the program to sort the words by number of occurrence (ascending or descending)

## Shapes and intersections

Note: the checks here don't have to be 100% mathematically correct. The goal is to use traits to compose functionality.

### Core and tests

1. Define a type for Rectangle.
1. Define a trait `Intersects<T>`
    - it must have a single function to check if the object it's called on intersects another
1. Write an implementation for Rectangle to check that it intersects other rectangles
1. Add some other shapes (like Circle or Triangle)
    - add implementations for `Intersects` to check intersections for each pair of shapes
1. Define an enum `Shape`, with a variant to contain each kind of shape
    - optionally also implement `Intersects` for `Shape` (hint: you can just match and call the previous impls)

### Reading from file

1. Establish a simple text format for each shape. Example:
    - "rectangle 2 4 20 30" can mean a rectangle, with two following x & y pairs for 2 corners
    - "circle 10 10 5" can mean a circle a circle, centered at point 10 10 (x y), with radius 5
1. Write a file with some shapes in the format above, one per line
1. Read the file, and parse into a `Vec<Shape>`
    - hint: use `split_whitespace` on each line

### Compute and output

1. Compute intersection sets:
    - divide the vector into a `Vec<HashSet<Shape>>`
    - all shapes in a set must intersect at least one shape in that set
    - the sets must be disjoint (no shape from set 1 may intersect a shape from set 2, etc.)
1. Print the intersection sets
