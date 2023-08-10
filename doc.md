# Documentation

### Variables
In Storyteller, variables are characters in your plot. All variables in Storyteller must be constructed from uppercase letters, lowercase letters and spaces. Variables must obey the following regex: `[a-zA-Z\s]+`

Variable names are case-sensitive, so `prince`, `Prince`, and `pRinCe` refer to the same variable.

Furthermore, for multi-word variables, variable names are insensitive to the number of spaces between the words. That is, regardless of how many spaces are between `the` and `wizard`, it will always refer to the variable `the wizard`. 

Lastly, all variables only have one type: they are all unbounded integers.

### Number Literal
Now, let's be honest: numbers are cool and all, but they are admittedly quite disappointing when they appear right in the middle of an intense action sequence.

Therefore, Storyteller has a feature known as poetic literals.

draws inspiration from other influential languages like [Rockstar](https://github.com/RockstarLang/rockstar). 

For example, `xd` corresponds to the number literal `k`, and `do` corresponds to the number literal `m`. 

### Statements
In Storyteller, statements are sentences of the English language. Each statement can end with `.`, `?` or `!`. There are many different types of statements, and each of their syntax and semantics are outlined below:

### Assignment
The statement below assigns the value of `NUMBER_LITERAL | VARIABLE` to `VARIABLE`:
```
[VARIABLE] [TO_BE_KEYWORD] [NUMBER_LITERAL | VARIABLE].
```
`TO_BE_KEYWORD` can take the values `is`, `was`, `were`

Some examples of assignment are as follows:

### Arithmetic
Addition
```
[VARIABLE] felt as [POSITIVE_ADJECTIVE] as [NUMBER_LITERAL|VARIABLE].
```

Subtraction
```
[VARIABLE] felt as [NEGATIVE_ADJECTIVE] as [NUMBER_LITERAL|VARIABLE].
```
### Goto Statements
```
* [GOTO_KEYWORD] [NUMBER_LITERAL | VARIABLE]
```
`go to`, `goes to`, `went to`, `gone to` and `going to`

### Input/Output Statements

```
[VARIABLE] (looks|looked) up to the skies above, waiting for an answer.
```

```
"[QUOTE]", [VARIABLE] [SAID].
```
In order to print the VARIABLE
```
"[QUOTE]", [VARIABLE] [SAID] [ADVERB].
```

### Conditionals

### Exit Statements

Any statement that contains the word `end`, and that does not match any of the statement patterns described above, causes the program to exit with `EXIT_SUCCESS`.

### Comments
All other sentences that do not match the rules of the statements above is a comment. Comment use is considered highly idiomatic since it allows you to add eloquence and flair to your plot and characters.

### Pronouns

The keywords `it`, `he`, `she`, `him`, `her`, `they`, `them`, `ze`, `hir`, `zie`, `zir`, `xe`, `xem`, `ve`, and `ver` refer to the last named variable part of a non-comment statement.

### Surbodinate Clause
For any of the statements above, instead of ending the sentence directly, you can 