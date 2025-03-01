
[![Run FibBot](https://github.com/Nkwenti-Severian-Ndongtsop/Fibonacci-bot/actions/workflows/fibbot.yml/badge.svg?branch=main)](https://github.com/Nkwenti-Severian-Ndongtsop/Fibonacci-bot/actions/workflows/fibbot.yml)

# Fibonacci Bot 

This is a bot designed using github actions that take every number below the threshold limit and finds the fibonacci of the numbers 
and outputs the result as a new comment

This set up contains so many files whose role will be properly broken down:

We have the action.yml file that takes variable from the workflow file

### Break down of the action.yml file

- This file has all the variables that are passed to my rust binaries during the work flow run
- This file build and passes arguments directly to all the rust binaries is my project

### break down of the files in the src directory

 I have the;

 - extract_text
 - fibonacci
 - ge_pull_request
 - post_comment_to_github
 - main
This files work in collaboration with each other

We use the get_pull_request to obtain the pull_request using an external crate called **Octocrab** that needs to included in our cargo.toml file.

it resturns a string that is passed to annother function that extracts numbers from the string.

The function returns a vector of intergers 

This vector of integer is passed onto the my fibonacci function as iterators returning each fiboonacci as a string slice

Now my post_comment function uses Github APIs to make a post request to github server, creating a comment under the pull_request with the fibonacci of numbers gotten from the changes done on the pull_request
   
