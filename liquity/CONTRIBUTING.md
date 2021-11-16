# Contributing
---


## Types of Contributions


### Report Bugs

Report bugs at https://github.com/RatioFinance/ratio-contracts/issues .


### Fix Bugs

Look through the GitHub issues for bugs. Anything tagged with "bug"
and "help wanted" is open to whoever wants to implement it.


### Implement Features

Look through the GitHub issues for features. Anything tagged with "enhancement"
and "help wanted" is open to whoever wants to implement it. Those that are
tagged with "first-timers-only" is suitable for those getting started in open-source software.

**Guidelines:**
1. For each feature being implemented, please setup a new `/feature` branch that is branched off of the latest `test` branch. 
2. Once you feel the feature is ready for an initial review, please send a pull request (PR) from your `/feature` branch, and set the `test` branch as the base branch. Please label the PR as a [Draft] if you think it is still not ready to be merged.
3. From here, I will review the pull request, and may ask necessary clarification, or changes.
4. Once I feel the pull request is ready, I will approve and merge the PR into test, which will be setup for testing from the client side
5. If any issues are found from here, additional changes may still be requested. These may be reflected on a PR from a new `/fix` branch, or from the original `/feature` branch.


### Write Documentation

`ratio-contracts` could always use more documentation, whether as part of the
official `ratio-contracts` docs, in docstrings, and such.


### Submit Feedback

The best way to send feedback is to file an issue at https://github.com/RatioFinance/ratio-contracts/issues .

If you are proposing a feature:

* Explain in detail how it would work.
* Keep the scope as narrow as possible, to make it easier to implement.
* Remember that this is a volunteer-driven project, and that contributions
  are welcome :)

## Get Started!


Ready to contribute? Here's how to set up `fastquant` for local development.

1. Clone the `ratio-contracts` repo
    ```shell
    $ git clone https://github.com/RatioFinance/ratio-contracts.git
    ```

2. Create a branch for local development
    ```shell
    $ git checkout -b name-of-your-bugfix-or-feature
    ```
    Now you can make your changes locally.

3. Commit your changes and push your branch to GitHub
    ```shell
    $ git add .
    $ git commit -m "Your detailed description of your changes."
    $ git push origin name-of-your-bugfix-or-feature
    ```

    In brief, commit messages should follow these conventions:

    * Always contain a subject line which briefly describes the changes made. For example "Update CONTRIBUTING.md".
    * Subject lines should not exceed 50 characters.
    * The commit body should contain context about the change - how the code worked before, how it works now and why you decided to solve the issue in the way you did.

    More detail on commit guidelines can be found at https://chris.beams.io/posts/git-commit

7. Submit a pull request with the base branch set as `test`


## Pull Request Guidelines

Before you submit a pull request, check that it meets these guidelines:

1. The pull request should include tests when relevant.
2. If the pull request adds functionality, the docs should be updated. Put
   your new functionality into a function with a docstring, and add the
   feature to the list in README.md.

