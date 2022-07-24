# sea-monster-toy

I'm handling all the cases.

I'm using fixed point numbers to ensure the exactness of arithmetic operations (up to four decimal places).

I wrote tests that cover the different situations, including special cases.

Errors that should cause the program to exit are propagated down to the `main` function, where they're logged and the
program exits with error code `1`.
Errors that should not cause the program to exit (errors stemming from invalid transactions) are propagated down to the
`process_transactions` function, where they're logged.

## Assumptions

- Each line in the input CSV contains four fields, regardless of the type of transaction.
- The tx field of transaction types dispute, resolve, and chargeback, references a deposit transaction.
- A chargeback can result in a negative balance on a client's account.

## Possible improvements

- Abstract out CSV parsing to make it easier to support different formats in the future.
- Wrap propagated errors with a message at each step to make the resulting logged error message more informative of the
  chain that produced it (can be achieved with crate error_chain).
- Define custom error types for (at least) the invalid transaction errors. This would help if in the future we needed to
  handle specific errors differently.
- Write tests for the `Engine` module, testing whether correct errors are returned.
