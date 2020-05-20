# RPN-calculator

A command line RPN calculator tool with multiple sessions.

### Putting Numbers in the Stack

You can enter numbers into the stack by typing out the number and pressing enter. Multiple numbers can be entered at a time by separating them with whitespace.

### Performing Operations

You can perform operations by postfixing them on the numbers. 
An example would be `4 5 +` resulting in 9.
Unary operations can be entering the operation after the number you wish to perform 
the operation on.
Binary operations are performed on the previous two numbers entered on the stack.
You can also remove the last number from the stack by using the operation `del`.
The last two numbers on the stack can be swapped using the `swap` command.
The numbers can be cyclically permuted using the command `cyc` or `cycle`.

### Manipulating Sessions
New session can be create with the command `new:name_of_session` and can be moved to by `go_to:name_of_session` or `goto:name_of_session`.
You can remove a session using `rm:name_of_session`.