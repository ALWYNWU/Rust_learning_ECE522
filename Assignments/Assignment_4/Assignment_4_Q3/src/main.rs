// The error is we can not assign task.id, because it is immutable.
// We can use Cell smart pointer to realize interior mutability


use std::cell::Cell;

enum Level {
    Low,
    Medium,
    High }
struct Task<T> {
    id: Cell<T>,
    level: Level
}
fn main() {
    let  task = Task {
        id: Cell::new(100),
        level: Level::High
    };
    task.id.set(100);
    println!("Task with ID: {}", task.id.get());
}