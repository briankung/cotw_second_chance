# Requirements

1. A Rust toolchain
2. Ruby
3. Ripgrep

# Usage

NOTE: I manually searched for and removed crates that had been nominated Crate of the Week for links that received 5 or more likes in `cleaned.csv`. The rest of the results were filtered with my Ruby script, with somewhat dubious results.

NOTE THE SECOND: the `output.csv` and `cleaned.csv` files are out of date with the current code.

0. `$ cd this-week-in-rust`
1. `$ git pull`
2. `$ cd ..`
3. `cargo run > output.csv`
4. `ruby cleanup.rb > cleaned.csv`

The resulting CSV files are urls arranged in order of likes descending.

# Methodology

I downloaded all of the comments in the TWIR crate of the week thread as JSON via [this url][0]. Then I associated each url with the number of likes it had received via its parent `Post` sorted them by likes in descending order, to get the most liked links at the top of the file.

Originally, I had meant to only compare links submitted within the same week, but eventually decided it just made more sense to look for the most popular links overall. Furthermore, I originally limited the urls to the single most clicked link in a given post, so one post would result in one link (if it had any). I regret both of those decisions, but oh well ¯\\\_(ツ)_/¯

I then began the arduous process of clearing out crates of the week from the resulting list by searching the `this-week-in-rust/content` folder with ripgrep, then decided to automate it with a quick Ruby script, which also turned out to be more difficult than I bargained for. Trying to recreate my Ruby code in Rust only led to more hair tearing so at this point, I'm satisfied with having manually cleared out nominated crate URLs for crate URLs with 5 or more likes.

I now have a much healthier respect for data munging. I might get back to it, but I need a break from this "simple" task for a little bit 😵‍💫

[0]: "https://users.rust-lang.org/t/crate-of-the-week/2704.json?print=true"
