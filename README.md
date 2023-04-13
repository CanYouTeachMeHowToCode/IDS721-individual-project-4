# Individual Project #4: Serverless Data Engineering Pipeline with Rust
Build a useful, serverless application in Rust.

In this project, I constructed a random Chess960, also called  [Fischer random chess](https://en.wikipedia.org/wiki/Fischer_random_chess) initial state board generator and deployed it onto the [AWS Lambda](https://aws.amazon.com/lambda/) as an AWS Lambda function. 

As its name suggests, Chess960 is a variant of Chess that may have different permutations of first-rank chess pieces (every pieces except for the pawn) for each side at the start state of the game. The specific rules are listed below:
- The bishops must be placed on opposite-color squares.
- The king must be placed on a square between the rooks.
- White pieces and black pieces are symmetric.

One of the possible board configuration of Chess960 is shown below:
![Sample Chess960 Board](https://user-images.githubusercontent.com/50161537/231633971-db9f68b0-62d7-4185-ac0e-3538dc51c7f3.png)

Therefore, doing some quick math, we have 4 light squares for one bishop, 4 dark squares for the other bishop, 6 remaining squares for the queen and 5! / (3! × 2!) = 5 × 4 / 2 = 10 ways to place the two (identical) knights on the remaining 5 squares contributes to 4 × 4 × 6 × 10 × 1 = 960 distinct legal permutations of the starting positions of the first-rank pieces. 

This project generates one of the 960 possible start boards and deployes it onto AWS Lambda. 

## Usage
> Compile the project by first running command `make all`, which also deploys the function to the AWS Lambda.

> Invoke the project by either directly invoke it on AWS or running command `aws lambda invoke --function-name project4  output.json`; this will generate the random start board of Chess960 in json format as the function output and put the  output to `output.json` in the same directory.

> To gain a nicely formatted output board, run `python output_chessboard.py`, and here is the example output:
```
♝♞♜♝♚♜♞♛
♟♟♟♟♟♟♟♟
........
........
........
........
♙♙♙♙♙♙♙♙
♗♘♖♗♔♖♘♕
```

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [Installing or updating the latest version of the AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)
