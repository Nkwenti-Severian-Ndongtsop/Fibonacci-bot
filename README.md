

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

- We have the fibonacci function that handles the fibonacci of all the numbers found in a particular pull_request
- We also have the extract_text, that collect all the positive integers in every pull_request 
