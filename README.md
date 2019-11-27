
# markov-name-generator
**How to use:**

Clone and enter the cargo project with this command:

`git clone https://github.com/uzkbwza/markov-name-generator.git && cd markov-name-generator`

You'll need cargo to build.

First, fill out the input.txt file with your own names for the program to derive from. Example of an input file (included in repo):
```
George Washington
John Adams
Thomas Jefferson
James Madison
James Monroe
John Quincy Adams
...
```
At least one name should start alphanumerically or the program will panic (will be changed down the line).

To generate results run the sub-command:
`cargo run generate <COUNT> --ngram_size <n> --min_length <l> (optional)`

- `ngram_size` determines exactly how big a sequence of chars will be for the Markov chain will recognize to be a unique unit. Higher values will lead to more coherent, but less interesting results. A value of 1 will result in a lot of variance, but it will be complete gibberish. 3 will tend to just shuffle around first- and last-names. I personally use 2, because it seems to be the best middle ground.
- `min_length` is the shortest length a generated result will be.

For example,`cargo run generate 1000 -n 2 -l 10` will generate 1000 results with an ngram size of 2 chars, and a minimum length of 10 chars.

Running that will populate the `output.txt` file. You can see the results by peeking in there, or can print any random ones using `get <COUNT>`

Running `cargo run get 5` gives us these alternate-reality U.S. presidents:

```
Klincy Howerter
Dwight D. Henjam Linton
Lysses M. B. Nixonalvington
Richard Roosenjam M. Hartin B. Jefferson
Xonald M. Nixon
```
