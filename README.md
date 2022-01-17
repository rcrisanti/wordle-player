# wordle-player

A bot that can play [Wordle](https://www.powerlanguage.co.uk/wordle/), written completely in Rust.

See below for a demo of it in action!

![A demo of it in action!](images/wordle-player-demo.gif)

## Stats

I'll keep track of the stats for how well it performs on the daily official Wordle puzzle below.

|    date    | word  |       solved       | turns | version |
| ---------- | ----- | ------------------ | ----- | ------- |
| 2022-01-17 | shire | :white_check_mark: |   4   |  0.1.0  |

## Sources
Currently, the bot uses static text files as its dictionary & to know how frequent letters occur in the English language. Here are the sources of those files:
 1. word database comes from: https://github.com/dwyl/english-words/
 2. letter frequency comes from: https://www3.nd.edu/~busiforc/handouts/cryptography/letterfrequencies.html
