/*!
# Problem

It's like conway's game of life, but with waiting room seats.

Input example:

    L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL

 * L = a seat
 * . = floor
 * # = An occupied seat

Each turn:

 * If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
 * If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
 * Otherwise, the seat's state does not change.
 * Floor (.) never changes

Keep running turns / rounds until the state stabalizes. How many seats end up occupied ?
*/

fn main() {
    println!("Hello, world!");
}
