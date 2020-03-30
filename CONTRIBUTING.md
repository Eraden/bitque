# How to contribute

I'm really glad you're reading this, because we need volunteer developers to help this project come to fruition.

## Testing

Use nothing other than standard rust tests.

## Submitting changes

Please send a [GitHub Pull Request to jirs](https://github.com/Eraden/hirs/pull/new/master) with a clear list of what you've done (read more about [pull requests](http://help.github.com/pull-requests/)). When you send a pull request, we will love you forever if you include RSpec examples. We can always use more test coverage. Please follow our coding conventions (below) and make sure all of your commits are atomic (one feature per commit).

Always write a clear log message for your commits. One-line messages are fine for small changes, but bigger changes should look like this:

    $ git commit -m "Bugfix(Area of change): A brief summary of the commit
    > 
    > A paragraph describing what changed and its impact."

## Coding conventions

Start reading our code and you'll get the hang of it. We optimize for readability:

  * We ALWAYS run `cargo fmt` before commit
  * We ALWAYS run `cargo clippy` before commit
  * We avoid local variables and prefer functions in theirs place
  * We prefer rust over JavaScript
  * We avoid putting logic in view
  * This is open source software. Consider the people who will read your code, and make it look nice for them. It's sort of like driving a car: Perhaps you love doing donuts when you're alone, but with passengers the goal is to make the ride as smooth as possible.

Thanks,
Adrian Woźniak
