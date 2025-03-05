mod contest;
mod problems;

fn main() {
    problems::easy::questions();
    contest::weekly::contest();
    contest::biweekly::contest();
}
