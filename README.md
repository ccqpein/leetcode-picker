# README #

This is a tiny app for picking the quiz from [leetcode](https://leetcode.com), print out quiz description and code snippet.

## Usage ##

Firstly, login leetcode and find out the cookie `csrftoken` in cache, it looks like `c6J80TGFMuNrdy4O9cuedV9fSSDw112xRUmTqkcgUs9sYbxxxxxxxxxxxx`. Then: 

`leetcode-picker --id 1 -c rust --token c6J80TGFMuNrdy4O9cuedV9fSSDw112xRUmTqkcgUs9sYbxxxxxxxxxxxx`.

Or you can put token inside separate file `./vault/csrftoken`

```shell
echo 'c6J80TGFMuNrdy4O9cuedV9fSSDw112xRUmTqkcgUs9sYbxxxxxxxxxxxx' > ./vault/csrftoken
```

then the `./vault/csrftoken` file is your token file, and run like `leetcode-picker --id 1 -c rust` will do the same thing as command before.

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

  * [ ] how to login and get token in this app?
    * [x] where to store token string?
  * [ ] emacs mode?
