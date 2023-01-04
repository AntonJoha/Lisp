
# What is this? 

This is the main subdirectory for the evaluation part. 
The star of the show is the function __process__ which is located in src/eval/mod.rs. 
Process is the main execution loop of the code. 
This in turn then calls to other places. 
Mainly either __operator::operator_eval__ or __function::function_eval__ depending on if it's an operator or function. 


## What are operators?

Operators are defined as the standard characters such as "+", "-", "==" etc.
A more comprehensive list can be found in the documentation. 

## What are functions?

Functions are defined by either def or made beforehand in the implementation and is everything that's not an operator. 


