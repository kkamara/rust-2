/*
Declare a `season` variable set to a string with
your favorite season. Provide an explicit type annotation.
The type of a string is a `&str`. We'll discuss what
the & symbol means later in the course.

Declare a `points_scored` variable set to 28.
Provide an explicit type annotation. The type of
an integer is `i32`.

It's time to update the team's score. Declare the
`points_scored` variable to be mutable. Set its
new value to 35.

Declare a `TOUCHDOWN_POINTS` constant at the file
level set to the value 6.

Declare a `event_time` variable set to a string of
"06:00".

Use variable shadowing to redeclare `event_time` set
to a integer of 6.

Use interpolation to print out all of the
declared variables and constants in a println! call.
Practice using direct interpolation {value}, sequential
arguments ( {} ), and numeric arguments ( {0} ).

Declare a `favorite_beverage` variable set to a string
of your favorite drink. Use an underscore to silence
the compiler warning about the variable being unused.

Remove the underscore. Provide a compiler directive
to silence the compiler warning about the variable
being unused.
*/

const TOUCHDOWN_POINTS: i8 = 6;

fn main() {
    let season: String = String::from("Spring");
    #[allow(unused_assignments)]
    let mut points_scored: i8 = 28;
    points_scored = 35;
    #[allow(unused_variables)]
    let event_time = "06:00";
    let event_time: i8 = 6;

    println!(
        "Season: {season}, Points Scored: {points_scored}, Touchdown Points: {TOUCHDOWN_POINTS}, Event Time: {event_time}"
    );
    println!(
        "Season: {}, Points Scored: {}, Touchdown Points: {}, Event Time: {}",
        season, points_scored, TOUCHDOWN_POINTS, event_time
    );
    println!(
        "Season: {0}, Points Scored: {1}, Touchdown Points: {2}, Event Time: {3}",
        season, points_scored, TOUCHDOWN_POINTS, event_time
    );

    // A _ prefix on a variable name tells the compiler that we won't be using this variable. This is useful when we want to declare a variable but don't need to use it in our code, and it prevents the compiler from giving us a warning about an unused variable.
    // let _favorite_beverage = "Kool-Aid";
    #[allow(unused_variables)]
    let favorite_beverage = "Kool-Aid";
}
