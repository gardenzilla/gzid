# gzid
Gardenzilla internal ID checker and generator helper tool

It uses a modified Luhn algorithm to create checksum (2 digit)
and verify a String by using it.

```bash
foo@bar:~$ gzid create 1
b5

foo@bar:~$ gzid check b5
true
```