# DS210-HW7
This is my DS210 HW7 Fall 2023

32.5/40

− 3 pts
Grading comment:
Q1 Auxilliary Function

− 1.5 pts
Grading comment:
Q1 Double perimeter method incorrect

− 3 pts
Grading comment:
Q1 Method for Correctness

1. (20 points) Design a data type Shape that can be used for storing a triangle, rectangle, or
circle. Additionally:
• For a triangle or rectangle, the data type should store the lengths of its sides.
• For a circle, it should store its radius.
Also create:
• an auxiliary function that helps create instances of the type,
• a method that computes the area of the shape,
• a method that computes the perimeter of the shape,
• a method that doubles the perimeter of the shape,
• a method that verifies the correctness of the parameters of the shape (i.e., there
should be no negative numbers and the triangle inequality should hold).
Hint: You may find Heron’s formula useful.
2. (20 points) Create a struct and a trait for a canonical polygon with an arbitrary number of
sides and length (i.e. the number of sides and the length of the sides is a parameter to the
constructor).
● Ensure that the trait includes method prototypes for the perimeter, area and radius of this
polygon.
● You can add other method prototypes if you consider them useful.
● Evaluate the area of polygons with {6, 12, 24, 128, 256, 512, 1024, 2048, 65536} sides
with a few different side lengths and compare that to the area of a circle with the same
radius as those polygons.
● What are your observations?
Hint: You may find the formula for a polygon apothem useful.
