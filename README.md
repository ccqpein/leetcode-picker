# README #

This is a tiny app for picking the quiz from [leetcode](https://leetcode.com), print out quiz description and code snippet.

## Usage ##

Firstly, login leetcode and find out the cookie `csrftoken`, it looks like `c6J80TGFMuNrdy4O9cuedV9fc9k0WURzxxxxxxxxxxxxxxxxxxxxxxxxxxx`. Then: 

```shell
echo 'c6J80TGFMuNrdy4O9cuedV9fc9k0WURzxRUmTqkcgUs9sYbxxxxxxxxxxxx' > ./vault/csrftoken
```

then the `./vault/csrftoken` file is your token file.

Unless you give `--id` or `--name`, you always need `-r` for randomly pick.

**Help**

`leetcode-picker -h`

**Pick random quiz**

`leetcode-picker -r` or `leetcode-picker --random`

**Pick random quiz until you are satisfied**

`leetcode-picker -r -i`

**Pick quiz by name**

`leetcode-picker --name two-sum`

**Pick quiz by id**

`leetcode-picker --id 1`

**Pick special level quiz**

`leetcode-picker -r -l hard`

**Pick code-snippet**

`leetcode-picker --id 1 -c rust` (show rust code snippet)

**Custom description format**

`leetcode-picker --id 1 -c rust --temp-str 'source link: {source}, title: {title}'`

all template fields:

- title
- source
- level
- content

### TODO ###

  * [x] get question
    * [x] get special level
    * [x] get question
  * [x] get descriptions
    * [x] parse it to markdown
  * [x] Get quiz code template
  * [ ] how to login and get token in this app?
    * [ ] where to store token string?
  * [x] cli arguments
    * [ ] ~~default give random~~
    * [x] `-i` after random means ask user if it is ok ~~(sub command)~~
    * [x] `--name`
    * [x] `--id` (cannot find graphql query message of it, might get all quizs then filter)
  * [x] source link
  * [x] custom print format
    * [x] also source link should in format too

